use crate::entity::visitor;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
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

impl Visitor {
    pub fn new(
        id: i64,
        uuid: String,
        ip: Option<String>,
        ip_source: Option<String>,
        os: Option<String>,
        browser: Option<String>,
        create_time: NaiveDateTime,
        last_time: NaiveDateTime,
        pv: Option<i32>,
        user_agent: Option<String>,
    ) -> Self {
        Self {
            id: id,
            uuid: uuid,
            ip: ip,
            ip_source: ip_source,
            os: os,
            browser: browser,
            create_time: create_time,
            last_time: last_time,
            pv: pv,
            user_agent: user_agent,
        }
    }
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

#[derive(Debug, Deserialize)]
pub struct VisitorQuery {
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
    pub ip: Option<String>,
    pub ip_source: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct VisitorDeleteParam {
    pub id: i64,
    pub uuid: String,
}
