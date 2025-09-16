/*
 * @Author: lurendie
 * @Date: 2024-03-26 00:08:12
 * @LastEditors: lurendie
 * @LastEditTime: 2024-05-15 19:14:37
 */
use crate::error::AppError;
use actix_web::{HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// 统一API响应结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    /// 状态码
    pub code: u16,
    /// 消息
    pub msg: String,
    /// 数据
    pub data: Option<T>,
}

/// API响应构建器
pub struct ApiResponseBuilder<T> {
    code: u16,
    msg: String,
    data: Option<T>,
}

impl<T> ApiResponseBuilder<T> {
    /// 创建新的响应构建器
    pub fn new() -> Self {
        Self {
            code: crate::error::ErrorCode::SUCCESS,
            msg: "成功".to_string(),
            data: None,
        }
    }

    /// 设置状态码
    pub fn code(mut self, code: u16) -> Self {
        self.code = code;
        self
    }

    /// 设置消息
    pub fn msg(mut self, msg: String) -> Self {
        self.msg = msg;
        self
    }

    /// 设置数据
    pub fn data(mut self, data: Option<T>) -> Self {
        self.data = data;
        self
    }

    /// 构建响应
    pub fn build(self) -> ApiResponse<T> {
        ApiResponse {
            code: self.code,
            msg: self.msg,
            data: self.data,
        }
    }
}

impl<T: Serialize> ApiResponse<T> {
    /// 成功响应
    pub fn success(data: Option<T>) -> Self {
        Self {
            code: crate::error::ErrorCode::SUCCESS,
            msg: "成功".to_string(),
            data,
        }
    }

    /// 成功响应（带自定义消息）
    pub fn success_with_msg(msg: String, data: Option<T>) -> Self {
        Self {
            code: crate::error::ErrorCode::SUCCESS,
            msg,
            data,
        }
    }

    /// 错误响应
    pub fn error(msg: String) -> Self {
        Self {
            code: crate::error::ErrorCode::INTERNAL_ERROR,
            msg,
            data: None,
        }
    }

    /// 错误响应（带自定义状态码）
    pub fn error_with_code(code: u16, msg: String) -> Self {
        Self {
            code,
            msg,
            data: None,
        }
    }

    /// 从错误创建响应
    pub fn from_error(error: &AppError) -> Self {
        Self {
            code: error.error_code(),
            msg: error.message().to_string(),
            data: None,
        }
    }

    /// 转换为HTTP响应
    pub fn json(&self) -> HttpResponse {
        HttpResponse::Ok()
            .content_type("application/json; charset=utf-8")
            .json(&self)
    }
}

impl<T> Default for ApiResponse<T> {
    fn default() -> Self {
        Self {
            code: crate::error::ErrorCode::SUCCESS,
            msg: "成功".to_string(),
            data: None,
        }
    }
}

/// 为AppError实现Responder，使其可以直接用于控制器
impl Responder for AppError {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        let response = ApiResponse::<()>::from_error(&self);
        response.json()
    }
}

/// 为Result<T, AppError>实现便捷方法
pub trait ApiResponseExt<T, E> {
    /// 成功时返回数据，失败时返回错误响应
    fn api_response(self) -> Result<T, AppError>;
    /// 成功时返回数据，失败时返回错误响应（带自定义消息）
    fn api_response_with_msg(self, msg: String) -> Result<T, AppError>;
}

impl<T, E: std::fmt::Display> ApiResponseExt<T, E> for Result<T, E> {
    fn api_response(self) -> Result<T, AppError> {
        self.map_err(|e| AppError::Custom(e.to_string()))
    }

    fn api_response_with_msg(self, msg: String) -> Result<T, AppError> {
        self.map_err(|_| AppError::Custom(msg))
    }
}
