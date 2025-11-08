use deadpool_redis::PoolError;
use sea_orm::{DbErr, TransactionError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataBaseError {
    #[error("RedisError异常消息：{0}")]
    RedisError(#[from] deadpool_redis::redis::RedisError),

    #[error("MySQLError异常消息：{0}")]
    MySQLError(#[from] DbErr),

    #[error("SerdeJsonError异常消息：{0}")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("SerdeYamlError异常消息：{0}")]
    SerdeYamlError(#[from] serde_yaml::Error),

    #[error("RedisPoolError异常消息: {0}")]
    PoolError(#[from] PoolError),

    #[error("MySQLTransactionError异常消息: {0}")]
    TransactionError(#[from] TransactionError<DbErr>),

    #[error("RegexError异常消息: {0}")]
    RegexError(#[from] regex::Error),

    #[error("{0}")]
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
