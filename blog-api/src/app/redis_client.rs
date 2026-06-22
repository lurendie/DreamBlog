use super::app_config::CONFIG;
use deadpool_redis::{Config, Pool, Runtime, Timeouts};
use std::sync::LazyLock;

fn build_redis_url(config: &crate::app::app_config::RedisConfig) -> String {
    if config.username.is_empty() && config.password.is_empty() {
        format!("redis://{}:{}/{}", config.host, config.port, config.db)
    } else {
        format!(
            "redis://{}:{}@{}:{}/{}",
            config.username, config.password, config.host, config.port, config.db
        )
    }
}

// Redis客户端，可选初始化，失败后自动降级为 None
pub static REDIS_CLIENT: LazyLock<Option<Pool>> = LazyLock::new(|| {
    let Some(redis_config) = CONFIG.get_redis_config() else {
        log::info!("Redis 已关闭，缓存层将自动降级为数据库");
        return None;
    };

    let redis_url = build_redis_url(&redis_config);
    match Config::from_url(redis_url.as_str()).create_pool(Some(Runtime::Tokio1)) {
        Ok(client) => {
            log::info!("Redis 连接池初始化成功");
            Some(client)
        }
        Err(e) => {
            log::warn!("Redis 连接池创建失败，缓存将自动降级: {e}");
            None
        }
    }
});

pub struct RedisClient;

impl RedisClient {
    /**
     * 获取redis连接，如 Redis 未启用或连接失败则返回 None
     */
    pub async fn get_connection() -> Option<deadpool_redis::Connection> {
        match &*REDIS_CLIENT {
            Some(pool) => pool.timeout_get(&Timeouts::wait_millis(300)).await.ok(),
            None => None,
        }
    }
}
