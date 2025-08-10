use crate::config::CONFIG;
use deadpool_redis::{Config, Pool, PoolError, Runtime};
use std::sync::LazyLock;

static REDIS_URL: LazyLock<String> = LazyLock::new(|| {
    let redis_config = CONFIG.get_redis_config();
    format!(
        "redis://{}:{}@{}:{}/{}",
        redis_config.username,
        redis_config.password,
        redis_config.host,
        redis_config.port,
        redis_config.db
    )
});

//Redis客户端
pub static REDIS_CL_IENT: LazyLock<Pool> = LazyLock::new(|| {
    match Config::from_url(REDIS_URL.as_str()).create_pool(Some(Runtime::Tokio1)) {
        Ok(client) => client,
        Err(e) => {
            //log::error!("redis连接池创建失败！ 错误信息：{}", e);
            panic!("redis连接池创建失败！ 错误信息：{}", e);
        }
    }
    // client.get_connection();
    // log::info!("redis客户端初始化完成！");
});

/**
 * 获取redis连接 如没有获取到连接则返回None
 */
pub async fn get_connection() -> Result<deadpool_redis::Connection, PoolError> {
    match REDIS_CL_IENT.get().await {
        Ok(conn) => Ok(conn),
        Err(e) => Err(e),
    }
}

/**
 * 获取redis连接 如5000ms内没有获取到连接则返回None
 */
pub async fn _timeout_get_connection() -> Result<deadpool_redis::Connection, PoolError> {
    match REDIS_CL_IENT
        .timeout_get(&deadpool_redis::Timeouts::wait_millis(5000))
        .await
    {
        Ok(conn) => Ok(conn),
        Err(e) => Err(e),
    }
}

pub async fn get_redis_pool() -> Pool {
    REDIS_CL_IENT.clone()
}

// //redis 单元测试
// #[cfg(test)]
// mod test {
//     use super::REDIS;
//     use redis::{Cmd, ToRedisArgs};

//     #[test]
//     fn test_connect() -> redis::RedisResult<()> {
//         let mut con = REDIS.get_connection().unwrap();
//         let _ = Cmd::set("my_key".to_redis_args(), 1.to_redis_args()).query::<()>(&mut con);
//         Ok(())
//     }
// }
