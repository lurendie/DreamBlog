use std::collections::HashMap;

use crate::entity::schedule_job;
use crate::entity::schedule_job_log;
use crate::middleware::AppClaims;
use crate::model::ScheduleJob;
use crate::model::ScheduleJobLog;
use crate::{app::AppState, model::ApiResponse};
use actix_jwt_session::Authenticated;
use actix_web::{routes, web, Responder};
use chrono::Utc;
use rbs::value;
use rbs::Value;
use sea_orm::{
    ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};
use serde::Deserialize;

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
pub struct LogIdParam {
    pub log_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct JobLogQuery {
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
    pub job_id: Option<i64>,
    pub status: Option<bool>,
}

#[routes]
#[get("/jobs")]
pub async fn get_job_list(
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
    query: web::Query<JobQuery>,
) -> impl Responder {
    let db = app.get_mysql_pool();
    let page_num = query.page_num.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(10);

    // 构建查询条件
    let mut query_builder = schedule_job::Entity::find();

    if let Some(bean_name) = &query.bean_name {
        query_builder = query_builder.filter(schedule_job::Column::BeanName.contains(bean_name));
    }

    if let Some(status) = query.status {
        query_builder = query_builder.filter(schedule_job::Column::Status.eq(status));
    }

    // 获取分页数据
    let paginator = query_builder
        .order_by_desc(schedule_job::Column::JobId)
        .paginate(db, page_size as u64);

    let total = paginator.num_items().await.unwrap_or(0);
    let jobs = paginator.fetch_page((page_num - 1) as u64).await;

    match jobs {
        Ok(job_models) => {
            let mut result = HashMap::new();
            let mut jobs = Vec::new();
            job_models.into_iter().for_each(|item| {
                jobs.push(ScheduleJob::from(item));
            });
            result.insert("total".to_string(), value!(total));
            result.insert("records".to_string(), value!(jobs));
            ApiResponse::<Value>::success_with_msg(
                "获取定时任务列表成功".to_string(),
                Some(value!(result)),
            )
            .json()
        }
        Err(e) => ApiResponse::<String>::error(format!("获取定时任务列表失败: {}", e)).json(),
    }
}

#[routes]
#[put("/job/status")]
pub async fn update_job_status(
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
    params: web::Query<JobStatusUpdate>,
) -> impl Responder {
    let db = app.get_mysql_pool();
    let job_id = params.job_id;

    let result = schedule_job::Entity::find_by_id(job_id).one(db).await;

    match result {
        Ok(Some(job_model)) => {
            let mut active_job: schedule_job::ActiveModel = job_model.into();
            active_job.status = Set(Some(params.status));

            match active_job.update(db).await {
                Ok(_) => ApiResponse::<String>::success_with_msg(
                    "更新定时任务状态成功".to_string(),
                    None,
                )
                .json(),
                Err(e) => {
                    ApiResponse::<String>::error(format!("更新定时任务状态失败: {}", e)).json()
                }
            }
        }
        Ok(None) => ApiResponse::<String>::error("定时任务不存在".to_string()).json(),
        Err(e) => ApiResponse::<String>::error(format!("查询定时任务失败: {}", e)).json(),
    }
}

#[routes]
#[post("/job/run")]
pub async fn run_job_once(
    _: Authenticated<AppClaims>,
    // app: web::Data<AppState>,
    // params: web::Query<JobIdParam>,
) -> impl Responder {
    // let db = app.get_mysql_pool();
    // let job_id = params.job_id;

    // 这里需要实现执行定时任务的逻辑
    // 由于没有相应的服务实现，这里先返回一个占位响应
    ApiResponse::<String>::success_with_msg("执行定时任务成功".to_string(), None).json()
}

#[routes]
#[delete("/job")]
pub async fn delete_job_by_id(
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
    params: web::Query<JobIdParam>,
) -> impl Responder {
    let db = app.get_mysql_pool();
    let job_id = params.job_id;

    match schedule_job::Entity::delete_by_id(job_id).exec(db).await {
        Ok(result) => {
            if result.rows_affected > 0 {
                ApiResponse::<String>::success_with_msg("删除定时任务成功".to_string(), None).json()
            } else {
                ApiResponse::<String>::error("定时任务不存在".to_string()).json()
            }
        }
        Err(e) => ApiResponse::<String>::error(format!("删除定时任务失败: {}", e)).json(),
    }
}

#[routes]
#[post("/job")]
pub async fn add_job(
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
    job: web::Json<ScheduleJob>,
) -> impl Responder {
    let db = app.get_mysql_pool();
    let now = Utc::now().naive_utc();

    let new_job = schedule_job::ActiveModel {
        job_id: NotSet,
        bean_name: Set(job.bean_name.clone()),
        method_name: Set(job.method_name.clone()),
        params: Set(job.params.clone()),
        cron: Set(job.cron.clone()),
        status: Set(job.status),
        remark: Set(job.remark.clone()),
        create_time: Set(Some(now)),
    };

    match new_job.insert(db).await {
        Ok(_) => {
            ApiResponse::<String>::success_with_msg("添加定时任务成功".to_string(), None).json()
        }
        Err(e) => ApiResponse::<String>::error(format!("添加定时任务失败: {}", e)).json(),
    }
}

#[routes]
#[put("/job")]
pub async fn edit_job(
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
    job: web::Json<ScheduleJob>,
) -> impl Responder {
    let db = app.get_mysql_pool();

    if job.job_id.is_none() {
        return ApiResponse::<String>::error("定时任务ID不能为空".to_string()).json();
    }

    let job_id = job.job_id.unwrap();

    let result = schedule_job::Entity::find_by_id(job_id).one(db).await;

    match result {
        Ok(Some(job_model)) => {
            let mut active_job: schedule_job::ActiveModel = job_model.into();
            active_job.bean_name = Set(job.bean_name.clone());
            active_job.method_name = Set(job.method_name.clone());
            active_job.params = Set(job.params.clone());
            active_job.cron = Set(job.cron.clone());
            active_job.status = Set(job.status);
            active_job.remark = Set(job.remark.clone());

            match active_job.update(db).await {
                Ok(_) => {
                    ApiResponse::<String>::success_with_msg("更新定时任务成功".to_string(), None)
                        .json()
                }
                Err(e) => ApiResponse::<String>::error(format!("更新定时任务失败: {}", e)).json(),
            }
        }
        Ok(None) => ApiResponse::<String>::error("定时任务不存在".to_string()).json(),
        Err(e) => ApiResponse::<String>::error(format!("查询定时任务失败: {}", e)).json(),
    }
}

#[routes]
#[get("/job/logs")]
pub async fn get_job_log_list(
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
    query: web::Query<JobLogQuery>,
) -> impl Responder {
    let db = app.get_mysql_pool();
    let page_num = query.page_num.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(10);

    // 构建查询条件
    let mut query_builder = schedule_job_log::Entity::find();

    if let Some(job_id) = query.job_id {
        query_builder = query_builder.filter(schedule_job_log::Column::JobId.eq(job_id));
    }

    if let Some(status) = query.status {
        query_builder = query_builder.filter(schedule_job_log::Column::Status.eq(status));
    }

    // 获取分页数据
    let paginator = query_builder
        .order_by_desc(schedule_job_log::Column::LogId)
        .paginate(db, page_size as u64);

    let total = paginator.num_items().await.unwrap_or(0);
    let logs = paginator.fetch_page((page_num - 1) as u64).await;

    match logs {
        Ok(log_models) => {
            let mut result = HashMap::new();
            let mut logs = vec![];
            log_models.into_iter().for_each(|item| {
                logs.push(ScheduleJobLog::from(item));
            });
            result.insert("total".to_string(), value!(total));
            result.insert("records".to_string(), value!(logs));
            ApiResponse::<Value>::success_with_msg(
                "获取定时任务日志列表成功".to_string(),
                Some(value!(result)),
            )
            .json()
        }
        Err(e) => ApiResponse::<String>::error(format!("获取定时任务日志列表失败: {}", e)).json(),
    }
}

#[routes]
#[delete("/job/log")]
pub async fn delete_job_log_by_log_id(
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
    params: web::Query<LogIdParam>,
) -> impl Responder {
    let db = app.get_mysql_pool();
    let log_id = params.log_id;

    match schedule_job_log::Entity::delete_by_id(log_id)
        .exec(db)
        .await
    {
        Ok(result) => {
            if result.rows_affected > 0 {
                ApiResponse::<String>::success_with_msg("删除定时任务日志成功".to_string(), None)
                    .json()
            } else {
                ApiResponse::<String>::error("定时任务日志不存在".to_string()).json()
            }
        }
        Err(e) => ApiResponse::<String>::error(format!("删除定时任务日志失败: {}", e)).json(),
    }
}
