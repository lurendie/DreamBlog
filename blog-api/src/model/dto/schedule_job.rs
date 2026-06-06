use crate::entity::schedule_job::Model;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ScheduleJob {
    #[serde(rename = "jobId")]
    pub job_id: Option<i64>,
    #[serde(rename = "beanName")]
    pub bean_name: Option<String>,
    #[serde(rename = "methodName")]
    pub method_name: Option<String>,
    pub params: Option<String>,
    pub cron: Option<String>,
    pub status: Option<bool>,
    pub remark: Option<String>,
    #[serde(rename = "createTime")]
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
    #[serde(rename = "pageNum")]
    pub page_num: Option<u32>,
    #[serde(rename = "pageSize")]
    pub page_size: Option<u32>,
    #[serde(rename = "beanName")]
    pub bean_name: Option<String>,
    pub status: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct JobStatusUpdate {
    #[serde(rename = "jobId")]
    pub job_id: i64,
    pub status: bool,
}

#[derive(Debug, Deserialize)]
pub struct JobIdParam {
    #[serde(rename = "jobId")]
    pub job_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct JobLogQuery {
    #[serde(rename = "pageNum")]
    pub page_num: Option<u32>,
    #[serde(rename = "pageSize")]
    pub page_size: Option<u32>,
    #[serde(rename = "jobId")]
    pub job_id: Option<i64>,
    pub status: Option<bool>,
    pub date: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct JobLogIdParam {
    #[serde(rename = "logId")]
    pub log_id: i64,
}
