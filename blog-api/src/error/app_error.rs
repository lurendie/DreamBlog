use actix_web::{body::BoxBody, error, http::StatusCode, HttpResponse};
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
}

impl error::ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        ApiResponse::<String>::error(&self.to_string()).respond()
    }
}
