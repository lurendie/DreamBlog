use crate::entity::exception_log;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExceptionLog {
    pub id: i64,
    pub uri: String,
    pub method: String,
    pub param: Option<String>,
    pub description: Option<String>,
    pub error: Option<String>,
    pub ip: Option<String>,
    #[serde(rename = "ipSource")]
    pub ip_source: Option<String>,
    pub os: Option<String>,
    pub browser: Option<String>,
    #[serde(rename = "createTime")]
    pub create_time: NaiveDateTime,
    #[serde(rename = "userAgent")]
    pub user_agent: Option<String>,
}

impl From<exception_log::Model> for ExceptionLog {
    fn from(model: exception_log::Model) -> Self {
        Self {
            id: model.id,
            uri: model.uri,
            method: model.method,
            param: model.param,
            description: model.description,
            error: model.error,
            ip: model.ip,
            ip_source: model.ip_source,
            os: model.os,
            browser: model.browser,
            create_time: model.create_time,
            user_agent: model.user_agent,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ExceptionLogQuery {
    #[serde(rename = "pageNum")]
    pub page_num: Option<u32>,
    #[serde(rename = "pageSize")]
    pub page_size: Option<u32>,
    pub date: Option<String>,
}
