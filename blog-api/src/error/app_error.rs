/*
 * @Author: lurendie
 * @Date: 2024-03-26 00:08:12
 * @LastEditors: lurendie
 * @LastEditTime: 2024-05-15 19:14:37
 */
use crate::error::error_code::ErrorCode;
use crate::error::DataBaseError;
use thiserror::Error;

/// 应用错误类型
#[derive(Error, Debug)]
pub enum AppError {
    /// 数据库错误
    #[error("数据库错误: {0}")]
    Database(#[from] DataBaseError),

    /// 参数验证错误
    #[error("参数验证错误: {0}")]
    Validation(String),

    /// 资源未找到
    #[error("资源未找到: {0}")]
    NotFound(String),

    /// 权限不足
    #[error("权限不足: {0}")]
    Unauthorized(String),

    /// 内部服务器错误
    #[error("内部服务器错误: {0}")]
    Internal(String),

    /// 业务逻辑错误
    #[error("业务逻辑错误: {0}")]
    Business(String),

    /// JWT错误
    #[error("JWT错误: {0}")]
    Jwt(String),

    /// Redis错误
    #[error("Redis错误: {0}")]
    Redis(String),

    /// 自定义错误
    #[error("自定义错误: {0}")]
    Custom(String),
}

impl AppError {
    /// 获取错误码
    pub fn error_code(&self) -> u16 {
        match self {
            AppError::Database(_) => ErrorCode::DATABASE_ERROR,
            AppError::Validation(_) => ErrorCode::VALIDATION_ERROR,
            AppError::NotFound(_) => ErrorCode::NOT_FOUND,
            AppError::Unauthorized(_) => ErrorCode::UNAUTHORIZED,
            AppError::Internal(_) => ErrorCode::INTERNAL_ERROR,
            AppError::Business(_) => ErrorCode::BUSINESS_ERROR,
            AppError::Jwt(_) => ErrorCode::JWT_ERROR,
            AppError::Redis(_) => ErrorCode::REDIS_ERROR,
            AppError::Custom(_) => ErrorCode::CUSTOM_ERROR,
        }
    }

    // 获取错误消息
    pub fn message(&self) -> String {
        match self {
            AppError::Database(msg) => msg.to_string().clone(),
            AppError::Validation(msg) => msg.clone().to_string(),
            AppError::NotFound(msg) => msg.clone().to_string(),
            AppError::Unauthorized(msg) => msg.clone().to_string(),
            AppError::Internal(msg) => msg.clone().to_string(),
            AppError::Business(msg) => msg.clone().to_string(),
            AppError::Jwt(msg) => msg.clone().to_string(),
            AppError::Redis(msg) => msg.clone().to_string(),
            AppError::Custom(msg) => msg.clone().to_string(),
        }
    }
}

/// 从错误码和消息创建应用错误
impl From<(u16, String)> for AppError {
    fn from((code, message): (u16, String)) -> Self {
        match code {
            ErrorCode::VALIDATION_ERROR => AppError::Validation(message),
            ErrorCode::NOT_FOUND => AppError::NotFound(message),
            ErrorCode::UNAUTHORIZED => AppError::Unauthorized(message),
            ErrorCode::INTERNAL_ERROR => AppError::Internal(message),
            ErrorCode::BUSINESS_ERROR => AppError::Business(message),
            ErrorCode::JWT_ERROR => AppError::Jwt(message),
            ErrorCode::REDIS_ERROR => AppError::Redis(message),
            ErrorCode::CUSTOM_ERROR => AppError::Custom(message),
            _ => AppError::Internal(message),
        }
    }
}
