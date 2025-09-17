use std::collections::HashMap;

use crate::config::CONFIG;
use crate::enums::DataBaseError;
use crate::redis_client;
use deadpool_redis::redis::AsyncCommands;
use rbs::Value;

pub struct RedisService;

impl RedisService {
    /**
        根据KEY HashName 查询HashMap<String, Value>
    */
    pub async fn get_hash_key(
        key: String,
        hash: String,
    ) -> Result<HashMap<String, Value>, DataBaseError> {
        //1.获取连接
        let mut connection = redis_client::get_connection().await?;
        //2.判断key是否存在
        let exists: i32 = connection.exists::<String, i32>(key.clone()).await?;
        if exists == 0 {
            return Err(DataBaseError::Custom(format!("redis {} 不存在", key)));
        }

        // 检查哈希字段是否存在
        let field_exists: i32 = connection.hexists::<String, String, i32>(key.clone(), hash.clone()).await?;
        if field_exists == 0 {
            return Err(DataBaseError::Custom(format!("redis {} 中不存在字段 {}", key, hash)));
        }

        let redis_reuslt: Option<String> = connection
            .hget::<String, String, Option<String>>(key.to_owned(), hash.to_owned())
            .await?;

        match redis_reuslt {
            Some(result) => {
                //3.redis反序列化
                let parsed_result = serde_json::from_str::<HashMap<String, Value>>(result.as_str())?;
                Ok(parsed_result)
            }
            None => Err(DataBaseError::Custom(format!("无法从 redis {} 获取字段 {} 的值", key, hash))),
        }
    }

    /**
     * 根据HashName key保存HashMap<String, Value>
     */
    pub async fn set_hash_key(
        key: String,
        hash: String,
        value: &HashMap<String, Value>,
    ) -> Result<(), DataBaseError> {
        //redis序列化
        let value_str = serde_json::to_string(&value).unwrap_or_default();
        let mut connection = redis_client::get_connection().await?;

        let _ = connection
            .hset::<String, String, String, String>(key.clone(), hash, value_str)
            .await?;
        RedisService::set_expire(key).await?;
        Ok(())
    }
    /**
     * Set `key` `value`字符串
     */
    pub async fn set_value_map(
        key: String,
        value: &HashMap<String, Value>,
    ) -> Result<(), DataBaseError> {
        //1.序列化
        let value_str = serde_json::to_string(&value).unwrap_or_default();
        //2.获取连接
        let mut connection = redis_client::get_connection().await?;
        connection
            .set::<String, String, String>(key.clone(), value_str)
            .await?;
        RedisService::set_expire(key).await?;
        Ok(())
    }

    /**
     * 获取`key`字符串
     */
    pub async fn get_value_map(key: String) -> Result<HashMap<String, Value>, DataBaseError> {
        //1.获取连接
        let mut connection = redis_client::get_connection().await?;

        // 检查key是否存在
        let exists: i32 = connection.exists::<String, i32>(key.clone()).await?;
        if exists == 0 {
            return Err(DataBaseError::Custom(format!("redis {} 不存在", key)));
        }

        let result: Option<String> = connection.get::<String, Option<String>>(key).await?;
        match result {
            Some(value) => {
                Ok(serde_json::from_str::<HashMap<String, Value>>(value.as_str())?)
            }
            None => {
                Err(DataBaseError::Custom(format!("无法从 redis {} 获取值", key)))
            }
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
        let mut con = redis_client::get_connection().await?;
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
        match redis_client::get_connection().await {
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
        let mut connection = redis_client::get_connection().await?;
        connection
            .expire::<String, i64>(key, CONFIG.get_redis_config().ttl)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_json_get() {
        let mut map: HashMap<String, Value> = HashMap::new();
        map.insert("1".to_string(), Value::String("value1".to_string()));

        //let _ = super::set_value("my_sql".to_string(), &map);
    }
}
