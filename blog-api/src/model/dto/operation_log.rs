use crate::entity::operation_log;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationLog {
    pub id: i64,
    pub username: String,
    pub uri: String,
    pub method: String,
    pub param: Option<String>,
    pub description: Option<String>,
    pub ip: Option<String>,
    #[serde(rename = "ipSource")]
    pub ip_source: Option<String>,
    pub os: Option<String>,
    pub browser: Option<String>,
    pub times: i32,
    #[serde(rename = "createTime")]
    pub create_time: NaiveDateTime,
    #[serde(rename = "userAgent")]
    pub user_agent: Option<String>,
}

impl From<operation_log::Model> for OperationLog {
    fn from(model: operation_log::Model) -> Self {
        Self {
            id: model.id,
            username: model.username,
            uri: model.uri,
            method: model.method,
            param: model.param,
            description: model.description,
            ip: model.ip,
            ip_source: model.ip_source,
            os: model.os,
            browser: model.browser,
            times: model.times,
            create_time: model.create_time,
            user_agent: model.user_agent,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct OperationLogQuery {
    #[serde(rename = "pageNum")]
    pub page_num: Option<u32>,
    #[serde(rename = "pageSize")]
    pub page_size: Option<u32>,
    pub date: Option<String>,
}
