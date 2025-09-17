use std::fmt::Debug;

use actix_web::{http::StatusCode, HttpResponse};
use rbs::Value;
//封装响应结果
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Result<T> {
    pub code: u16,
    pub msg: String,
    pub data: Option<T>,
}

impl<T> Result<T> {
    pub fn new(code: u16, msg: String, data: Option<T>) -> Result<T> {
        Result { code, msg, data }
    }

    //无异常返回
    pub fn ok(msg: String, data: Option<T>) -> Result<T> {
        Result {
            code: 200,
            msg,
            data,
        }
    }

    //无异常返回数据
    pub fn ok_no_data(msg: String) -> Result<T> {
        // 200 OK
        Result {
            code: 200,
            msg,
            data: None,
        }
    }

    pub fn error(msg: String) -> Result<T> {
        // 500 Internal Server Error
        Result {
            code: 500,
            msg,
            data: None,
        }
    }
}
//针对于ValueMap具体实现
impl Result<Value> {
    pub fn new_value(code: u16, msg: String, data: Option<Value>) -> Result<Value> {
        Result { code, msg, data }
    }
    //无异常返回
    pub fn ok_json(&self) -> HttpResponse {
        HttpResponse::Ok()
            .content_type(mime::TEXT_HTML_UTF_8)
            .json(&self)
    }

    pub fn error_json(&self) -> HttpResponse {
        HttpResponse::Ok()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .content_type(mime::TEXT_HTML_UTF_8)
            .json(&self)
    }
}

impl<T: Debug> std::fmt::Display for Result<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {},{:?})", self.code, self.msg, self.data)
    }
}
