use deadpool_redis::PoolError;
use sea_orm::{DbErr, TransactionError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataBaseError {
    #[error("redis 异常原因：{0}")]
    RedisError(#[from] deadpool_redis::redis::RedisError),

    #[error("MySQL 异常原因：{0}")]
    MySQLError(#[from] DbErr),

    #[error("serde_json 异常原因是：{0}")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("serde_yaml 异常原因是：{0}")]
    SerdeYamlError(#[from] serde_yaml::Error),

    #[error("Redis连接池异常原因：{0}")]
    PoolError(#[from] PoolError),

    #[error("MySQL事物异常原因 : {0}")]
    TransactionError(#[from] TransactionError<DbErr>),

    #[error("正则表达式异常原因 : {0}")]
    RegexError(#[from] regex::Error),

    #[error("异常原因：{0}")]
    Custom(String),
}
#[cfg(test)]
mod tests {

    use std::fs::read_to_string;

    fn render() -> Result<String, MyError> {
        let file = std::env::var("MARKDOWN")?;
        let source = read_to_string(file)?;
        Ok(source)
    }

    #[derive(thiserror::Error, Debug)]
    enum MyError {
        #[error("Environment variable not found")]
        EnvironmentVariableNotFound(#[from] std::env::VarError),
        #[error(transparent)]
        IOError(#[from] std::io::Error),
    }
    #[test]
    fn test_render() {
        let result = render();
        match result {
            Ok(s) => println!("Rendered markdown: {}", s),
            Err(e) => println!("Error: {}", e),
        }
    }
}
