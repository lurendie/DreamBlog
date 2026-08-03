use actix_web::{body::BoxBody, error, http::StatusCode, HttpResponse};
use lettre::address::AddressError;
use lettre::error::Error as EmailError;
use sea_orm::DbErr;
use thiserror::Error;

use crate::{
    error::{DataBaseError, WebError},
    model::ApiResponse,
};

#[derive(Error, Debug)]
pub enum AppError {
    #[error("WebError异常消息:{0}")]
    WebError(#[from] WebError),
    #[error("DataBaseError异常消息:{0}")]
    DataBaseError(#[from] DataBaseError),
    #[error("SerdeJsonError异常消息:{0}")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("EmailError异常消息:{0}")]
    EmailError(#[from] EmailServerError),
    #[error("{0}")]
    Custom(String),
}

impl From<DbErr> for AppError {
    fn from(value: DbErr) -> Self {
        AppError::DataBaseError(DataBaseError::MySQLError(value))
    }
}

impl error::ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        // 保持 HTTP 200 + body code 的既有契约；状态码语义问题见 error_response 注释
        StatusCode::OK
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        // 保留 WebError 的业务错误码（400/401/404/500...），避免全部退化为 500
        match self {
            AppError::WebError(e) => ApiResponse::<String>::from_error(e).respond(),
            _ => ApiResponse::<String>::error(&self.to_string()).respond(),
        }
    }
}

#[derive(Error, Debug)]
pub enum EmailServerError {
    #[error("构建出现异常: {0}")]
    AddressError(#[from] AddressError),
    #[error("异常消息: {0}")]
    Email(#[from] EmailError),
    #[error("此评论无需发送邮件")]
    NotSend,
    #[error("{0}")]
    Custom(String),
}
