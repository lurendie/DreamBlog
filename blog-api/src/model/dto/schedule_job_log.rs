use crate::entity::schedule_job_log;
use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ScheduleJobLog {
    pub log_id: i64,
    pub job_id: i64,
    pub bean_name: Option<String>,
    pub method_name: Option<String>,
    pub params: Option<String>,
    pub status: bool,
    pub error: Option<String>,
    pub times: i32,
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
