use std::collections::HashMap;

use crate::redis::REDIS;
use crate::CONFIG;
use rbs::Value;
use redis::{Commands, ConnectionLike};

pub struct RedisService;
impl RedisService {
    /**
        根据KEY HashName 查询HashMap<String, Value>
    */
    pub async fn get_hash_key(key: String, hash: String) -> Option<HashMap<String, Value>> {
        //1.获取连接
        let mut con = REDIS.get_connection().unwrap();
        let redis_reuslt = con
            .hget::<String, String, String>(key.to_owned(), hash.to_owned())
            .unwrap_or_else(|_| {
                log::error!("hash:{} key:{} 缓存数据不存在", hash, key);
                String::new()
            });
        //redis 反序列化
        if redis_reuslt.is_empty() {
            //如果是null 则返回空集合
            return None;
        } else {
            return Some(
                serde_json::from_str::<HashMap<String, Value>>(redis_reuslt.as_str()).unwrap(),
            );
        }
    }

    /**
     * 根据HashName key保存HashMap<String, Value>
     */
    pub async fn set_hash_key(key: String, hash: String, value: &HashMap<String, Value>) {
        //redis序列化
        let value_str = serde_json::to_string(&value).unwrap();
        let mut con: Box<dyn ConnectionLike> = Box::new(REDIS.get_connection().unwrap());
        redis::cmd("HSET")
            .arg(&[key.clone(), hash, value_str])
            .execute(con.as_mut());

        RedisService::expire(key);
    }
    /**
     * Set `key` `value`字符串
     */
    pub async fn set_value_map(key: String, value: &HashMap<String, Value>) {
        //1.序列化
        let value_str = serde_json::to_string(&value).unwrap();
        //2.获取连接
        let mut con = REDIS.get_connection().unwrap();

        let _ = con.set::<String, String, String>(key.clone(), value_str);
        RedisService::expire(key);
    }

    /**
     * 获取`key`字符串
     */
    pub async fn get_value_map(key: String) -> Option<HashMap<String, Value>> {
        //1.获取连接
        let mut con = REDIS.get_connection().unwrap();
        let result = con.get::<String, String>(key).unwrap_or_default();
        //redis 反序列化
        if result.is_empty() {
            return None;
        } else {
            return Some(
                serde_json::from_str::<HashMap<String, Value>>(result.as_str()).unwrap_or_default(),
            );
        }
    }

    /**
     * Set `key` `value`字符串
     */
    pub async fn set_value_vec(key: String, value: &Value) {
        //如果KEY或者VALUE为空则不设置
        if key.is_empty() || value.is_empty() {
            return;
        }
        //1.序列化
        let value_str = serde_json::to_string(value).unwrap_or_default();
        //判断转换字符串是否为空
        if value_str.is_empty() {
            return;
        }
        //2.获取连接
        match REDIS.get_connection() {
            //3.获取连接成功
            Ok(mut con) => {
                //4.设置数据
                let _ = con.set::<String, String, String>(key.clone(), value_str);
                //5.设置过期时间
                RedisService::expire(key);
            }
            //获取连接失败
            Err(e) => {
                log::error!("redis get connection error  set_value_vec:{}", e);
            }
        }
    }

    /**
     * 获取`key`字符串
     */
    pub async fn get_value_vec(key: String) -> Option<Value> {
        //1.获取连接
        match REDIS.get_connection() {
            //2.获取连接成功
            Ok(mut con) => {
                //3.获取数据
                let result = con.get::<String, String>(key).unwrap_or_else(|e| {
                    log::error!("redis get key error:{}", e);
                    String::new()
                });
                if result.is_empty() {
                    return None;
                }
                //redis 反序列化
                Some(serde_json::from_str(result.as_str()).unwrap_or_default())
            }
            //获取连接失败
            Err(e) => {
                log::error!("redis get connection error  get_value_vec:{}", e);
                None
            }
        }
    }

    /**
     * 设置key的过期时间
     */
    pub fn expire(key: String) {
        //获取配置
        match CONFIG.get() {
            Some(config) => {
                //获取连接
                if let Ok(redis) = REDIS.get_connection() {
                    let mut con: Box<dyn ConnectionLike> = Box::new(redis);
                    redis::cmd("EXPIRE")
                        .arg(&[key, config.redis.ttl.clone()])
                        .execute(con.as_mut());
                } else {
                    log::error!("redis连接失败")
                }
            }
            None => {
                log::error!("cofnig配置不存在")
            }
        }
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
