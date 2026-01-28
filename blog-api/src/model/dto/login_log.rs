use crate::entity::login_log;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginLog {
    pub id: i64,
    pub username: String,
    pub ip: Option<String>,
    #[serde(rename = "ipSource")]
    pub ip_source: Option<String>,
    pub os: Option<String>,
    pub browser: Option<String>,
    pub status: Option<bool>,
    pub description: Option<String>,
    #[serde(rename = "createTime")]
    pub create_time: NaiveDateTime,
    #[serde(rename = "userAgent")]
    pub user_agent: Option<String>,
}

impl From<login_log::Model> for LoginLog {
    fn from(model: login_log::Model) -> Self {
        Self {
            id: model.id,
            username: model.username,
            ip: model.ip,
            ip_source: model.ip_source,
            os: model.os,
            browser: model.browser,
            status: model.status,
            description: model.description,
            create_time: model.create_time,
            user_agent: model.user_agent,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct LoginLogQuery {
    #[serde(rename = "pageNum")]
    pub page_num: Option<u32>,
    #[serde(rename = "pageSize")]
    pub page_size: Option<u32>,
    pub date: Option<String>,
}
