use crate::entity::visitor;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Visitor {
    pub id: i64,
    pub uuid: String,
    pub ip: Option<String>,
    pub ip_source: Option<String>,
    pub os: Option<String>,
    pub browser: Option<String>,
    pub create_time: NaiveDateTime,
    pub last_time: NaiveDateTime,
    pub pv: Option<i32>,
    pub user_agent: Option<String>,
}

impl From<visitor::Model> for Visitor {
    fn from(value: visitor::Model) -> Self {
        Self {
            id: value.id,
            uuid: value.uuid,
            ip: value.ip,
            ip_source: value.ip_source,
            os: value.os,
            browser: value.browser,
            create_time: value.create_time,
            last_time: value.last_time,
            pv: value.pv,
            user_agent: value.user_agent,
        }
    }
}
