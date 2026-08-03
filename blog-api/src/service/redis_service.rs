use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;

use crate::app::RedisClient;
use crate::app::CONFIG;
use crate::error::DataBaseError;
use deadpool_redis::redis::AsyncCommands;
use deadpool_redis::redis::FromRedisValue;
use rbs::Value;
use serde::Serialize;

pub struct RedisService;

impl RedisService {
    async fn connection() -> Option<deadpool_redis::Connection> {
        RedisClient::get_connection().await
    }

    fn redis_ttl() -> Option<i64> {
        CONFIG.get_redis_config().and_then(|config| {
            if config.ttl > 0 {
                Some(config.ttl)
            } else {
                None
            }
        })
    }

    pub async fn get_hash_key<T: serde::de::DeserializeOwned>(
        key: String,
        hash: String,
    ) -> Result<T, DataBaseError> {
        let Some(mut connection) = Self::connection().await else {
            return Err(DataBaseError::Custom(format!("redis disabled: {}", key)));
        };
        let value = connection
            .hget::<String, String, Option<String>>(key.clone(), hash.clone())
            .await?;
        match value {
            Some(result_str) => Ok(serde_json::from_str::<T>(&result_str)?),
            None => Err(DataBaseError::Custom(format!(
                "无法从 redis {} 获取字段 {} 的值",
                key, hash
            ))),
        }
    }

    pub async fn set_hash_key<T: Serialize>(
        key: String,
        hash: String,
        value: &T,
    ) -> Result<bool, DataBaseError> {
        let Some(mut connection) = Self::connection().await else {
            return Ok(false);
        };
        let value_str = serde_json::to_string(value)?;
        connection
            .hset::<String, String, String, i64>(key.clone(), hash, value_str)
            .await?;
        if Self::redis_ttl().is_some() {
            Self::set_expire(key).await?;
        }
        Ok(true)
    }

    pub async fn try_set_hash_key<T: Serialize>(key: String, hash: String, value: &T) -> bool {
        match Self::set_hash_key(key.clone(), hash.clone(), value).await {
            Ok(stored) => stored,
            Err(e) => {
                log::debug!("redis key: {} hash: {} 缓存写入失败:{}", key, hash, e);
                false
            }
        }
    }

    pub async fn get_hash_all<K, V>(key: String) -> Result<HashMap<K, V>, DataBaseError>
    where
        K: serde::de::DeserializeOwned + Hash + Eq + FromRedisValue,
        V: serde::de::DeserializeOwned + Hash + Eq + FromRedisValue,
    {
        let Some(mut connection) = Self::connection().await else {
            return Err(DataBaseError::Custom(format!("redis disabled: {}", key)));
        };
        Ok(connection.hgetall::<String, HashMap<K, V>>(key).await?)
    }

    pub async fn set_string<T: Serialize>(key: String, value: &T) -> Result<bool, DataBaseError> {
        let Some(mut connection) = Self::connection().await else {
            return Ok(false);
        };
        let value_str = serde_json::to_string(value)?;
        connection
            .set::<String, String, String>(key.clone(), value_str)
            .await?;
        if Self::redis_ttl().is_some() {
            Self::set_expire(key).await?;
        }
        Ok(true)
    }

    pub async fn try_set_string<T: Serialize>(key: String, value: &T) -> bool {
        match Self::set_string(key.clone(), value).await {
            Ok(stored) => stored,
            Err(e) => {
                log::debug!("redis key: {} 缓存写入失败:{}", key, e);
                false
            }
        }
    }

    pub async fn get_string<T: serde::de::DeserializeOwned>(
        key: String,
    ) -> Result<T, DataBaseError> {
        let Some(mut connection) = Self::connection().await else {
            return Err(DataBaseError::Custom(format!("redis disabled: {}", key)));
        };

        let exists: i32 = connection.exists::<String, i32>(key.clone()).await?;
        if exists == 0 {
            return Err(DataBaseError::Custom(format!("key:{} 不存在", key)));
        }

        let result: String = connection.get::<String, String>(key.clone()).await?;
        if result.is_empty() {
            return Err(DataBaseError::Custom(format!("key:{} 不存在", key)));
        }
        Ok(serde_json::from_str::<T>(&result)?)
    }

    pub async fn set_value_vec(key: String, value: &Value) -> Result<bool, DataBaseError> {
        if key.is_empty() || value.is_empty() {
            return Err(DataBaseError::Custom(format!(
                "redis 设置key{}的value数据为空",
                key
            )));
        }
        let Some(mut connection) = Self::connection().await else {
            return Ok(false);
        };
        let value_str = serde_json::to_string(value)?;
        connection
            .set::<String, String, String>(key.clone(), value_str)
            .await?;
        if Self::redis_ttl().is_some() {
            Self::set_expire(key).await?;
        }
        Ok(true)
    }

    pub async fn try_set_value_vec(key: String, value: &Value) -> bool {
        match Self::set_value_vec(key.clone(), value).await {
            Ok(stored) => stored,
            Err(e) => {
                log::debug!("redis key: {} 缓存写入失败:{}", key, e);
                false
            }
        }
    }

    pub async fn get_value_vec(key: String) -> Option<Value> {
        let Some(mut connection) = Self::connection().await else {
            return None;
        };

        let exists: i32 = connection
            .exists::<String, i32>(key.clone())
            .await
            .unwrap_or(0);
        if exists == 0 {
            return None;
        }
        match connection.get::<String, Option<String>>(key.clone()).await {
            Ok(Some(result)) => serde_json::from_str(result.as_str()).ok(),
            Ok(None) => None,
            Err(e) => {
                log::debug!("redis {} 获取数据错误：{}", key, e);
                None
            }
        }
    }

    pub async fn set_expire(key: String) -> Result<(), DataBaseError> {
        let Some(mut connection) = Self::connection().await else {
            return Ok(());
        };
        let Some(ttl) = Self::redis_ttl() else {
            return Ok(());
        };
        let _ = connection
            .expire::<String, i64>(key.clone(), ttl)
            .await
            .map_err(|e| log::debug!("redis key: {} 设置过期时间失败:{}", key, e));
        Ok(())
    }

    pub async fn _del_key(key: &str) -> Result<(), DataBaseError> {
        let Some(mut connection) = Self::connection().await else {
            return Ok(());
        };
        let _ = connection
            .del::<String, i64>(key.to_string())
            .await
            .map_err(|e| log::debug!("redis key: {} 删除失败:{}", key, e));
        Ok(())
    }

    pub async fn try_del_key(key: &str) {
        let _ = Self::_del_key(key).await;
    }

    /// 判断 key 是否存在（Redis 关闭时返回 false，调用方应将其视为“不限频”）
    pub async fn key_exists(key: &str) -> bool {
        let Some(mut connection) = Self::connection().await else {
            return false;
        };
        match connection.exists::<String, i64>(key.to_string()).await {
            Ok(count) => count > 0,
            Err(e) => {
                log::debug!("redis key: {} 判断存在失败:{}", key, e);
                false
            }
        }
    }

    /// 设置带独立过期时间的字符串值（秒），用于限频等短时效数据
    pub async fn set_string_ttl<T: Serialize>(key: String, value: &T, ttl_secs: u64) -> bool {
        let Some(mut connection) = Self::connection().await else {
            return false;
        };
        let value_str = match serde_json::to_string(value) {
            Ok(s) => s,
            Err(e) => {
                log::debug!("redis key: {} 序列化失败:{}", key, e);
                return false;
            }
        };
        match connection
            .set_ex::<String, String, ()>(key.clone(), value_str, ttl_secs)
            .await
        {
            Ok(_) => true,
            Err(e) => {
                log::debug!("redis key: {} 设置过期值失败:{}", key, e);
                false
            }
        }
    }

    /// 限频检查：窗口期内已存在则拒绝，否则占用窗口。Redis 关闭时始终放行。
    pub async fn check_rate_limit(key: &str, window_secs: u64) -> bool {
        if Self::key_exists(key).await {
            return false;
        }
        Self::set_string_ttl(key.to_string(), &1u8, window_secs).await
    }

    /// 自增计数，首次自增时设置过期时间（秒），用于登录失败锁定等。Redis 关闭时返回 None。
    pub async fn incr_with_ttl(key: &str, ttl_secs: u64) -> Option<i64> {
        let Some(mut connection) = Self::connection().await else {
            return None;
        };
        let count: i64 = match connection
            .incr::<String, i64, i64>(key.to_string(), 1)
            .await
        {
            Ok(n) => n,
            Err(e) => {
                log::debug!("redis key: {} 自增失败:{}", key, e);
                return None;
            }
        };
        if count == 1 {
            let _ = connection
                .expire::<String, i64>(key.to_string(), ttl_secs as i64)
                .await;
        }
        Some(count)
    }
}
