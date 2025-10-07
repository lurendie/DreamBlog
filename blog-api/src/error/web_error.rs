/*
 * @Author: lurendie
 * @Date: 2024-03-26 00:08:12
 * @LastEditors: lurendie
 * @LastEditTime: 2024-05-15 19:14:37
 */
use crate::error::error_code::WebErrorCode;
use thiserror::Error;

/// 应用错误类型
#[derive(Error, Debug)]
pub enum WebError {
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

    /// 自定义错误
    #[error("自定义错误: {0}")]
    Custom(String),
}

impl WebError {
    /// 获取错误码
    pub fn error_code(&self) -> u16 {
        match self {
            WebError::Validation(_) => WebErrorCode::VALIDATION_ERROR,
            WebError::NotFound(_) => WebErrorCode::NOT_FOUND,
            WebError::Unauthorized(_) => WebErrorCode::UNAUTHORIZED,
            WebError::Internal(_) => WebErrorCode::INTERNAL_ERROR,
            WebError::Business(_) => WebErrorCode::BUSINESS_ERROR,
            WebError::Jwt(_) => WebErrorCode::JWT_ERROR,
            WebError::Custom(_) => WebErrorCode::CUSTOM_ERROR,
        }
    }

    // 获取错误消息
    pub fn message(&self) -> String {
        match self {
            WebError::Validation(msg) => msg.clone().to_string(),
            WebError::NotFound(msg) => msg.clone().to_string(),
            WebError::Unauthorized(msg) => msg.clone().to_string(),
            WebError::Internal(msg) => msg.clone().to_string(),
            WebError::Business(msg) => msg.clone().to_string(),
            WebError::Jwt(msg) => msg.clone().to_string(),
            WebError::Custom(msg) => msg.clone().to_string(),
        }
    }
}

/// 从错误码和消息创建应用错误
impl From<(u16, String)> for WebError {
    fn from((code, message): (u16, String)) -> Self {
        match code {
            WebErrorCode::VALIDATION_ERROR => WebError::Validation(message),
            WebErrorCode::NOT_FOUND => WebError::NotFound(message),
            WebErrorCode::UNAUTHORIZED => WebError::Unauthorized(message),
            WebErrorCode::INTERNAL_ERROR => WebError::Internal(message),
            WebErrorCode::BUSINESS_ERROR => WebError::Business(message),
            WebErrorCode::JWT_ERROR => WebError::Jwt(message),
            WebErrorCode::CUSTOM_ERROR => WebError::Custom(message),
            _ => WebError::Internal(message),
        }
    }
}
