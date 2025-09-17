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
        if !connection.exists::<String, bool>(key.clone()).await? {
            return Err(DataBaseError::Custom(format!("redis {} 不存在", key)));
        }

        let redis_reuslt = connection
            .hget::<String, String, String>(key.to_owned(), hash.to_owned())
            .await?;

        //3.redis反序列化
        let result = serde_json::from_str::<HashMap<String, Value>>(redis_reuslt.as_str())?;
        Ok(result)
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
        let result = connection.get::<String, String>(key).await?;
        Ok(serde_json::from_str::<HashMap<String, Value>>(
            result.as_str(),
        )?)
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
                if !connection
                    .exists::<String, bool>(key.clone())
                    .await
                    .unwrap_or(false)
                {
                    log::info!("redis KEY: {} 没有检索到数据 ", key);
                    return None;
                }
                //4.获取数据
                match connection.get::<String, String>(key.clone()).await {
                    Ok(result) => {
                        //redis 反序列化
                        Some(serde_json::from_str(result.as_str()).unwrap_or_default())
                    }
                    Err(e) => {
                        log::error!("redis {} 反序列化错误：{}", key, e);
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
