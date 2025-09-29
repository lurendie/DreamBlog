use crate::app::RedisClient;
use crate::app::CONFIG;
use crate::error::DataBaseError;
use deadpool_redis::redis::AsyncCommands;
use rbs::value::map::ValueMap;
use rbs::Value;
use serde::Serialize;

pub struct RedisService;

impl RedisService {
    /**
        根据KEY HashName 查询HashMap<String, Value>
    */
    pub async fn get_hash_key<T: serde::de::DeserializeOwned>(
        key: String,
        hash: String,
    ) -> Result<T, DataBaseError> {
        //1.获取连接
        let mut connection = RedisClient::get_connection().await?;
        //2.判断key是否存在
        let exists = Self::hexists(&key, &hash).await?;
        if !exists {
            return Err(DataBaseError::Custom(format!(
                "无法从 redis {} 获取字段 {} 的值",
                key, hash
            )));
        }

        let redis_reuslt = connection
            .hget::<String, String, Option<String>>(key.to_owned(), hash.to_owned())
            .await?;
        match redis_reuslt {
            Some(result_str) => {
                let result = serde_json::from_str::<T>(&result_str)?;
                return Ok(result);
            }
            None => {
                return Err(DataBaseError::Custom(format!(
                    "无法从 redis {} 获取字段 {} 的值",
                    key, hash
                )));
            }
        }
    }

    /**
     * 根据hash KEY查询字符串
     */
    pub async fn hexists(key: &str, hash: &str) -> Result<bool, DataBaseError> {
        //1.获取连接
        let mut connection = RedisClient::get_connection().await?;
        //2.判断key是否存在
        let exists = Self::key_exists(key).await?;
        if exists {
            // 检查哈希字段是否存在
            let field_exists: i32 = connection
                .hexists::<String, String, i32>(key.to_string(), hash.to_string())
                .await?;
            if field_exists != 0 {
                return Ok(true);
            }
        }
        Ok(false)
    }
    
    /**
     * 判断key是否存在
     */
    pub async fn key_exists(key: &str) -> Result<bool, DataBaseError> {
        //1.获取连接
        let mut connection = RedisClient::get_connection().await?;
        //2.判断key是否存在
        let exists: i32 = connection.exists::<String, i32>(key.to_string()).await?;
        if exists == 0 {
            return Ok(false);
        }
        Ok(true)
    }

    /**
     * 根据HashName key保存HashMap<String, Value>
     */
    pub async fn set_hash_key<T: Serialize>(
        key: String,
        hash: String,
        value: &T,
    ) -> Result<(), DataBaseError> {
        //redis序列化
        let value_str = serde_json::to_string(&value).unwrap_or_default();
        let mut connection = RedisClient::get_connection().await?;

        connection
            .hset::<String, String, String, i64>(key.clone(), hash, value_str)
            .await?;
        RedisService::set_expire(key).await?;
        Ok(())
    }
    /**
     * Set `key` `value`字符串
     */
    pub async fn set_value_map(key: String, value: &ValueMap) -> Result<(), DataBaseError> {
        //1.序列化
        let value_str = serde_json::to_string(&value).unwrap_or_default();
        //2.获取连接
        let mut connection = RedisClient::get_connection().await?;
        connection
            .set::<String, String, String>(key.clone(), value_str)
            .await?;
        RedisService::set_expire(key).await?;
        Ok(())
    }

    /**
     * 获取`key`字符串
     */
    pub async fn get_value_map(key: String) -> Result<ValueMap, DataBaseError> {
        //1.获取连接
        let mut connection = RedisClient::get_connection().await?;

        // 检查key是否存在
        let exists: i32 = connection.exists::<String, i32>(key.clone()).await?;
        if exists == 0 {
            return Err(DataBaseError::Custom(format!("key:{} 不存在", key)));
        }

        let result: Option<String> = connection
            .get::<String, Option<String>>(key.clone())
            .await?;
        match result {
            Some(value) => Ok(serde_json::from_str::<ValueMap>(value.as_str())?),
            None => Err(DataBaseError::Custom(format!(
                "无法从 redis {} 获取值",
                key
            ))),
        }
    }

    /**
     * Set `key` `value`字符串
     */
    pub async fn set_value_vec(key: String, value: &Value) -> Result<(), DataBaseError> {
        //如果KEY或者VALUE为空则不设置
        if key.is_empty() || value.is_empty() {
            return Err(DataBaseError::Custom(format!(
                "redis 设置key{}的value数据为空",
                key
            )));
        }
        //1.序列化
        let value_str = serde_json::to_string(value)?;
        //2.获取连接
        let mut con = RedisClient::get_connection().await?;
        con.set::<String, String, String>(key.clone(), value_str)
            .await?;
        //5.设置过期时间
        RedisService::set_expire(key).await?;
        Ok(())
    }

    /**
     * 获取`key`字符串
     */
    pub async fn get_value_vec(key: String) -> Option<Value> {
        //1.获取连接
        match RedisClient::get_connection().await {
            //2.获取连接成功
            Ok(mut connection) => {
                //3.a.判断key是否存在
                let exists: i32 = connection
                    .exists::<String, i32>(key.clone())
                    .await
                    .unwrap_or(0);
                if exists == 0 {
                    log::info!("redis KEY: {} 没有检索到数据 ", key);
                    return None;
                }
                //4.获取数据
                match connection.get::<String, Option<String>>(key.clone()).await {
                    Ok(Some(result)) => {
                        //redis 反序列化
                        match serde_json::from_str(result.as_str()) {
                            Ok(value) => Some(value),
                            Err(e) => {
                                log::error!("redis {} 反序列化错误：{}", key, e);
                                None
                            }
                        }
                    }
                    Ok(None) => {
                        log::info!("redis KEY: {} 没有数据", key);
                        None
                    }
                    Err(e) => {
                        log::error!("redis {} 获取数据错误：{}", key, e);
                        None
                    }
                }
            }
            //获取连接失败
            Err(e) => {
                log::error!("redis 设置key: {} 获取连接异常:{}", key, e);
                None
            }
        }
    }

    /**
     * 设置key的过期时间
     */
    pub async fn set_expire(key: String) -> Result<(), DataBaseError> {
        //获取连接
        let mut connection = RedisClient::get_connection().await?;
        match connection
            .expire::<String, i64>(key.clone(), CONFIG.get_redis_config().ttl)
            .await
        {
            Ok(_) => log::info!("redis key: {} 设置过期时间成功", key),
            Err(e) => log::error!("redis key: {} 设置过期时间失败:{}", key, e),
        };
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;
    #[test]
    fn test_json_get() {
        let mut map: HashMap<String, Value> = HashMap::new();
        map.insert("1".to_string(), Value::String("value1".to_string()));

        //let _ = super::set_value("my_sql".to_string(), &map);
    }
}
