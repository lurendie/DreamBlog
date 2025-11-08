use actix_web::{body::BoxBody, error, http::StatusCode, HttpResponse};
use lettre::address::AddressError;
use lettre::error::Error as EmailError;
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
    #[error("EmailError异常消息:{0}")]
    EmailError(#[from] EmailServerError),
    #[error("{0}")]
    Custom(String),
}

impl error::ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        ApiResponse::<String>::error(&self.to_string()).respond()
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
