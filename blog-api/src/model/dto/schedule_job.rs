use crate::entity::schedule_job::Model;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ScheduleJob {
    pub job_id: Option<i64>,
    pub bean_name: Option<String>,
    pub method_name: Option<String>,
    pub params: Option<String>,
    pub cron: Option<String>,
    pub status: Option<bool>,
    pub remark: Option<String>,
    pub create_time: Option<NaiveDateTime>,
}

impl From<Model> for ScheduleJob {
    fn from(value: Model) -> Self {
        ScheduleJob {
            job_id: Some(value.job_id),
            bean_name: value.bean_name,
            method_name: value.method_name,
            params: value.params,
            cron: value.cron,
            status: value.status,
            remark: value.remark,
            create_time: Some(value.create_time.unwrap_or_default()),
        }
    }
}




#[derive(Debug, Deserialize)]
pub struct JobQuery {
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
    pub bean_name: Option<String>,
    pub status: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct JobStatusUpdate {
    pub job_id: i64,
    pub status: bool,
}

#[derive(Debug, Deserialize)]
pub struct JobIdParam {
    pub job_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct JobLogQuery {
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
    pub job_id: Option<i64>,
    pub status: Option<bool>,
}
