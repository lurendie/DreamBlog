use crate::entity::visit_log;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisitLog {
    pub id: i64,
    pub uuid: Option<String>,
    pub uri: String,
    pub method: String,
    pub param: String,
    pub behavior: Option<String>,
    pub content: Option<String>,
    pub remark: Option<String>,
    pub ip: Option<String>,
    pub ip_source: Option<String>,
    pub os: Option<String>,
    pub browser: Option<String>,
    pub times: i32,
    pub create_time: NaiveDateTime,
    pub user_agent: Option<String>,
}

impl From<visit_log::Model> for VisitLog {
    fn from(item: visit_log::Model) -> Self {
        VisitLog {
            id: item.id,
            uuid: item.uuid,
            uri: item.uri,
            method: item.method,
            param: item.param,
            behavior: item.behavior,
            content: item.content,
            remark: item.remark,
            ip: item.ip,
            ip_source: item.ip_source,
            os: item.os,
            browser: item.browser,
            times: item.times,
            create_time: item.create_time,
            user_agent: item.user_agent,
        }
    }
}
