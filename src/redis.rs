use once_cell::sync::Lazy;
use redis::Client;

use crate::config;

//Redis客户端
pub static REDIS: Lazy<Client> = Lazy::new(|| {
    let conf = config::default();
    let client = redis::Client::open(format!(
        "redis://{}:{}@{}:{}/{}",
        conf.redis.username, conf.redis.password, conf.redis.host, conf.redis.port, conf.redis.db
    ))
    .unwrap();
    // client.get_connection();
    // log::info!("redis连接初始化完成！");
    client
});

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
