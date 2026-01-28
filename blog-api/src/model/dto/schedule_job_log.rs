use crate::entity::schedule_job_log;
use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ScheduleJobLog {
    #[serde(rename = "logId")]
    pub log_id: i64,
    #[serde(rename = "jobId")]
    pub job_id: i64,
    #[serde(rename = "beanName")]
    pub bean_name: Option<String>,
    #[serde(rename = "methodName")]
    pub method_name: Option<String>,
    pub params: Option<String>,
    pub status: bool,
    pub error: Option<String>,
    pub times: i32,
    #[serde(rename = "createTime")]
    pub create_time: Option<NaiveDateTime>,
}

impl From<schedule_job_log::Model> for ScheduleJobLog {
    fn from(model: schedule_job_log::Model) -> Self {
        Self {
            log_id: model.log_id,
            job_id: model.job_id,
            bean_name: model.bean_name,
            method_name: model.method_name,
            params: model.params,
            status: model.status,
            error: model.error,
            times: model.times,
            create_time: model.create_time,
        }
    }
}
