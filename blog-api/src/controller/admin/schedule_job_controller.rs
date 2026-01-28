use std::collections::HashMap;

use crate::error::AppError;
use crate::middleware::AppClaims;
use crate::model::JobIdParam;
use crate::model::JobLogIdParam;
use crate::model::JobLogQuery;
use crate::model::JobQuery;
use crate::model::JobStatusUpdate;
use crate::model::ScheduleJob;
use crate::service::ScheduleJobService;
use crate::{app::AppState, model::ApiResponse};
use actix_jwt_session::Authenticated;
use actix_web::{routes, web};
use rbs::value;
use rbs::Value;

#[routes]
#[get("/jobs")]
pub async fn get_job_list(
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
    query: web::Query<JobQuery>,
) -> Result<ApiResponse<Value>, AppError> {
    let db = app.get_mysql_pool();
    let map = ScheduleJobService::get_job_list(query.0, db).await?;
    let mut result = HashMap::new();
    result.insert("total".to_string(), value!(map.1));
    result.insert("list".to_string(), value!(map.0));
    Ok(ApiResponse::<Value>::success_with_msg(
        "获取定时任务列表成功",
        Some(value!(result)),
    ))
}

#[routes]
#[put("/job/status")]
pub async fn update_job_status(
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
    params: web::Query<JobStatusUpdate>,
) -> Result<ApiResponse<Value>, AppError> {
    let db = app.get_mysql_pool();
    ScheduleJobService::update_job_status(params.0, db).await?;
    Ok(ApiResponse::<Value>::success_with_msg(
        "更新定时任务状态成功",
        None,
    ))
}

#[routes]
#[post("/job/run")]
pub async fn run_job_once(
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
    params: web::Query<JobIdParam>,
) -> Result<ApiResponse<Value>, AppError> {
    let db = app.get_mysql_pool();
    let job_id = params.job_id;
    ScheduleJobService::run_job_once(job_id, db).await?;
    Ok(ApiResponse::<Value>::success_with_msg(
        "执行定时任务成功",
        None,
    ))
}

#[routes]
#[delete("/job")]
pub async fn delete_job_by_id(
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
    params: web::Query<JobIdParam>,
) -> Result<ApiResponse<Value>, AppError> {
    let db = app.get_mysql_pool();
    let job_id = params.job_id;
    ScheduleJobService::delete_job_by_id(job_id, db).await?;
    Ok(ApiResponse::<Value>::success_with_msg(
        "删除定时任务成功",
        None,
    ))
}

#[routes]
#[post("/job")]
pub async fn add_job(
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
    job: web::Json<ScheduleJob>,
) -> Result<ApiResponse<Value>, AppError> {
    let db = app.get_mysql_pool();
    ScheduleJobService::save_job(job.0, db).await?;
    Ok(ApiResponse::<Value>::success_with_msg(
        "添加定时任务成功",
        None,
    ))
}

#[routes]
#[put("/job")]
pub async fn edit_job(
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
    job: web::Json<ScheduleJob>,
) -> Result<ApiResponse<Value>, AppError> {
    let db = app.get_mysql_pool();
    ScheduleJobService::update_job(db, job.0).await?;
    Ok(ApiResponse::<Value>::success_with_msg(
        "更新定时任务成功",
        None,
    ))
}

#[routes]
#[get("/job/logs")]
pub async fn get_job_log_list(
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
    query: web::Query<JobLogQuery>,
) -> Result<ApiResponse<Value>, AppError> {
    let db = app.get_mysql_pool();
    let map = ScheduleJobService::get_job_log_list(query.0, db).await?;
    let mut result = HashMap::new();
    result.insert("total".to_string(), value!(map.1));
    result.insert("list".to_string(), value!(map.0));
    Ok(ApiResponse::<Value>::success_with_msg(
        "获取定时任务日志列表成功",
        Some(value!(result)),
    ))
}

#[routes]
#[delete("/job/log")]
pub async fn delete_job_log_by_log_id(
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
    params: web::Query<JobLogIdParam>,
) -> Result<ApiResponse<Value>, AppError> {
    let db = app.get_mysql_pool();
    let job_id = params.log_id;
    ScheduleJobService::delete_job_log(job_id, db).await?;
    Ok(ApiResponse::<Value>::success_with_msg(
        "删除定时任务日志成功",
        None,
    ))
}
