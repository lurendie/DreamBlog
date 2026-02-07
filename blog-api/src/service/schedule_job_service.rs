use chrono::{DateTime, NaiveDateTime, Utc};
use reqwest::Client;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
};
use serde_json::Value as JsonValue;
use std::str::FromStr;
use std::time::Instant;
use tokio::process::Command;

use crate::{
    constant::RedisKeyConstant,
    entity::{schedule_job, schedule_job_log},
    error::DataBaseError,
    model::{JobLogQuery, JobQuery, JobStatusUpdate, ScheduleJob, ScheduleJobLog},
    service::{RedisService, VisitStatsService},
};

pub struct ScheduleJobService;

impl ScheduleJobService {
    pub async fn get_job_list(
        query: JobQuery,
        db: &DatabaseConnection,
    ) -> Result<(Vec<ScheduleJob>, u64), DataBaseError> {
        let page_num = query.page_num.unwrap_or(1);
        let page_size = query.page_size.unwrap_or(10);
        // 构建查询条件
        let mut query_builder = schedule_job::Entity::find();

        if let Some(bean_name) = &query.bean_name {
            query_builder =
                query_builder.filter(schedule_job::Column::BeanName.contains(bean_name));
        }

        if let Some(status) = query.status {
            query_builder = query_builder.filter(schedule_job::Column::Status.eq(status));
        }

        // 获取分页数据
        let paginator = query_builder
            .order_by_desc(schedule_job::Column::JobId)
            .paginate(db, page_size as u64);

        let total = paginator.num_items().await.unwrap_or(0);
        let job_models = paginator.fetch_page((page_num - 1) as u64).await?;

        let mut jobs = Vec::new();
        job_models.into_iter().for_each(|item| {
            jobs.push(ScheduleJob::from(item));
        });
        Ok((jobs, total))
    }

    pub async fn update_job_status(
        params: JobStatusUpdate,
        db: &DatabaseConnection,
    ) -> Result<(), DataBaseError> {
        let job_id = params.job_id;
        let result = schedule_job::Entity::find_by_id(job_id).one(db).await?;
        match result {
            Some(job_model) => {
                let mut active_job: schedule_job::ActiveModel = job_model.into();
                active_job.status = Set(Some(params.status));

                active_job.update(db).await?;
                Ok(())
            }
            None => Err(DataBaseError::Custom("定时任务不存在".to_string())),
        }
    }

    pub async fn delete_job_log(log_id: i64, db: &DatabaseConnection) -> Result<(), DataBaseError> {
        schedule_job_log::Entity::delete_by_id(log_id)
            .exec(db)
            .await?;
        Ok(())
    }

    pub async fn get_job_log_list(
        query: JobLogQuery,
        db: &DatabaseConnection,
    ) -> Result<(Vec<ScheduleJobLog>, u64), DataBaseError> {
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
        if let Some(date) = query.date.as_deref() {
            if let Some((start, end)) = parse_date_range(date) {
                query_builder =
                    query_builder.filter(schedule_job_log::Column::CreateTime.between(start, end));
            }
        }

        // 获取分页数据
        let paginator = query_builder
            .order_by_desc(schedule_job_log::Column::LogId)
            .paginate(db, page_size as u64);

        let total = paginator.num_items().await.unwrap_or(0);
        let log_models = paginator.fetch_page((page_num - 1) as u64).await?;

        let mut logs = vec![];
        log_models.into_iter().for_each(|item| {
            logs.push(ScheduleJobLog::from(item));
        });
        Ok((logs, total))
    }

    pub async fn delete_job_by_id(
        job_id: i64,
        db: &DatabaseConnection,
    ) -> Result<(), DataBaseError> {
        schedule_job::Entity::delete_by_id(job_id).exec(db).await?;
        schedule_job_log::Entity::delete_many()
            .filter(schedule_job_log::Column::JobId.eq(job_id))
            .exec(db)
            .await?;
        Ok(())
    }

    pub async fn save_job(job: ScheduleJob, db: &DatabaseConnection) -> Result<(), DataBaseError> {
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
        new_job.insert(db).await?;
        Ok(())
    }

    pub async fn update_job(
        db: &DatabaseConnection,
        job: ScheduleJob,
    ) -> Result<(), DataBaseError> {
        let job_id = job.job_id.unwrap_or(0);
        let result = schedule_job::Entity::find_by_id(job_id).one(db).await?;
        match result {
            Some(job_model) => {
                let mut active_job: schedule_job::ActiveModel = job_model.into();
                active_job.bean_name = Set(job.bean_name.clone());
                active_job.method_name = Set(job.method_name.clone());
                active_job.params = Set(job.params.clone());
                active_job.cron = Set(job.cron.clone());
                active_job.status = Set(job.status);
                active_job.remark = Set(job.remark.clone());
                active_job.update(db).await?;
                Ok(())
            }
            None => Err(DataBaseError::Custom("定时任务不存在".to_string())),
        }
    }

    pub async fn run_job_once(job_id: i64, db: &DatabaseConnection) -> Result<(), DataBaseError> {
        let job_model = schedule_job::Entity::find_by_id(job_id).one(db).await?;
        let job_model =
            job_model.ok_or_else(|| DataBaseError::Custom("定时任务不存在".to_string()))?;
        Self::execute_job_model(job_model, db).await
    }
}

impl ScheduleJobService {
    pub async fn execute_job_model(
        job_model: schedule_job::Model,
        db: &DatabaseConnection,
    ) -> Result<(), DataBaseError> {
        let start = Instant::now();
        let result = execute_job_action(&job_model, db).await;
        let duration = start.elapsed().as_millis().min(i32::MAX as u128) as i32;

        let (status, error) = match result {
            Ok(_) => (true, None),
            Err(message) => (false, Some(message)),
        };

        let now = Utc::now().naive_utc();
        let log_model = schedule_job_log::ActiveModel {
            log_id: NotSet,
            job_id: Set(job_model.job_id),
            bean_name: Set(job_model.bean_name.clone()),
            method_name: Set(job_model.method_name.clone()),
            params: Set(job_model.params.clone()),
            status: Set(status),
            error: Set(error.clone()),
            times: Set(duration),
            create_time: Set(Some(now)),
        };
        log_model.insert(db).await?;

        if let Some(error) = error {
            return Err(DataBaseError::Custom(error));
        }
        Ok(())
    }
}

async fn execute_job_action(
    job_model: &schedule_job::Model,
    db: &DatabaseConnection,
) -> Result<(), String> {
    let bean_name = job_model
        .bean_name
        .clone()
        .unwrap_or_default()
        .to_lowercase();
    let method_name = job_model.method_name.clone().unwrap_or_default();
    let params = job_model.params.clone().unwrap_or_default();

    if bean_name.starts_with("http") {
        return execute_http_job(&bean_name, &method_name, &params).await;
    }
    if bean_name == "shell" {
        return execute_shell_job(&method_name, &params).await;
    }
    if bean_name == "local" {
        return execute_local_job(&method_name, &params, db).await;
    }
    Err(format!("不支持的任务类型: {}", bean_name))
}

async fn execute_http_job(bean_name: &str, url: &str, params: &str) -> Result<(), String> {
    if url.trim().is_empty() {
        return Err("HTTP任务URL为空".to_string());
    }
    let client = Client::new();
    let http_method = bean_name.split(':').nth(1).unwrap_or("");
    let should_get = http_method.eq_ignore_ascii_case("get") || params.trim().is_empty();

    if should_get {
        let mut request = client.get(url);
        if !params.trim().is_empty() {
            if let Ok(json_value) = serde_json::from_str::<JsonValue>(params) {
                request = request.query(&json_value);
            }
        }
        request.send().await.map_err(|e| e.to_string())?;
        return Ok(());
    }

    let mut request = client.post(url);
    if let Ok(json_value) = serde_json::from_str::<JsonValue>(params) {
        request = request.json(&json_value);
    } else if !params.trim().is_empty() {
        request = request.body(params.to_string());
    }
    request.send().await.map_err(|e| e.to_string())?;
    Ok(())
}

async fn execute_shell_job(command: &str, params: &str) -> Result<(), String> {
    if command.trim().is_empty() {
        return Err("Shell命令为空".to_string());
    }
    let full_command = if params.trim().is_empty() {
        command.to_string()
    } else {
        format!("{} {}", command, params)
    };
    let status = if cfg!(windows) {
        Command::new("cmd")
            .arg("/C")
            .arg(full_command)
            .status()
            .await
            .map_err(|e| e.to_string())?
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(full_command)
            .status()
            .await
            .map_err(|e| e.to_string())?
    };
    if status.success() {
        Ok(())
    } else {
        Err(format!("Shell执行失败: {}", status))
    }
}

async fn execute_local_job(
    method_name: &str,
    _params: &str,
    db: &DatabaseConnection,
) -> Result<(), String> {
    match method_name {
        "cache.clear_all" => {
            clear_all_cache().await.map_err(|e| e.to_string())?;
            Ok(())
        }
        "cache.clear_blog" => {
            clear_blog_cache().await.map_err(|e| e.to_string())?;
            Ok(())
        }
        "cache.clear_site" => {
            RedisService::_del_key(RedisKeyConstant::SITE_INFO_MAP)
                .await
                .map_err(|e| e.to_string())?;
            Ok(())
        }
        "cache.clear_about" => {
            RedisService::_del_key(RedisKeyConstant::ABOUT_INFO_MAP)
                .await
                .map_err(|e| e.to_string())?;
            Ok(())
        }
        "cache.clear_friend" => {
            RedisService::_del_key(RedisKeyConstant::FRIEND_INFO_MAP)
                .await
                .map_err(|e| e.to_string())?;
            Ok(())
        }
        "cache.clear_tag" => {
            RedisService::_del_key(RedisKeyConstant::TAG_CLOUD_LIST)
                .await
                .map_err(|e| e.to_string())?;
            Ok(())
        }
        "cache.clear_category" => {
            RedisService::_del_key(RedisKeyConstant::CATEGORY_NAME_LIST)
                .await
                .map_err(|e| e.to_string())?;
            Ok(())
        }
        "cache.clear_blog_views" => {
            RedisService::_del_key(RedisKeyConstant::BLOG_VIEWS_MAP)
                .await
                .map_err(|e| e.to_string())?;
            Ok(())
        }
        "stats.aggregate_visit" => {
            VisitStatsService::aggregate_visit_stats(db)
                .await
                .map_err(|e| e.to_string())?;
            Ok(())
        }
        _ => Err(format!("不支持的本地任务: {}", method_name)),
    }
}

async fn clear_blog_cache() -> Result<(), DataBaseError> {
    RedisService::_del_key(RedisKeyConstant::HOME_BLOG_INFO_LIST).await?;
    RedisService::_del_key(RedisKeyConstant::RANDOM_BLOG_LIST).await?;
    RedisService::_del_key(RedisKeyConstant::NEW_BLOG_LIST).await?;
    RedisService::_del_key(RedisKeyConstant::ARCHIVE_BLOG_MAP).await?;
    Ok(())
}

async fn clear_all_cache() -> Result<(), DataBaseError> {
    clear_blog_cache().await?;
    RedisService::_del_key(RedisKeyConstant::TAG_CLOUD_LIST).await?;
    RedisService::_del_key(RedisKeyConstant::CATEGORY_NAME_LIST).await?;
    RedisService::_del_key(RedisKeyConstant::SITE_INFO_MAP).await?;
    RedisService::_del_key(RedisKeyConstant::ABOUT_INFO_MAP).await?;
    RedisService::_del_key(RedisKeyConstant::FRIEND_INFO_MAP).await?;
    RedisService::_del_key(RedisKeyConstant::BLOG_VIEWS_MAP).await?;
    Ok(())
}

fn normalize_cron(cron: &str) -> Result<String, String> {
    let parts: Vec<&str> = cron.split_whitespace().collect();
    if parts.len() == 5 {
        return Ok(format!("0 {}", cron));
    }
    if parts.len() == 6 || parts.len() == 7 {
        return Ok(cron.to_string());
    }
    Err("Cron表达式格式不正确".to_string())
}

impl ScheduleJobService {
    pub fn next_run_after(
        cron: &str,
        last_run: DateTime<Utc>,
    ) -> Result<Option<DateTime<Utc>>, String> {
        let cron = normalize_cron(cron)?;
        let schedule = cron::Schedule::from_str(&cron).map_err(|e| e.to_string())?;
        Ok(schedule.after(&last_run).next())
    }
}

fn parse_date_range(date: &str) -> Option<(NaiveDateTime, NaiveDateTime)> {
    let mut parts = date.split(',');
    let start = parts.next()?;
    let end = parts.next()?;
    let start_dt = NaiveDateTime::parse_from_str(start, "%Y-%m-%d %H:%M:%S").ok()?;
    let end_dt = NaiveDateTime::parse_from_str(end, "%Y-%m-%d %H:%M:%S").ok()?;
    Some((start_dt, end_dt))
}
