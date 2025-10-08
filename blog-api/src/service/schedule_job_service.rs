use chrono::Utc;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
};

use crate::{
    entity::{schedule_job, schedule_job_log},
    error::DataBaseError,
    model::{JobLogQuery, JobQuery, JobStatusUpdate, ScheduleJob, ScheduleJobLog},
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

    pub async fn delete_job_log(job_id: i64, db: &DatabaseConnection) -> Result<(), DataBaseError> {
        schedule_job_log::Entity::delete_by_id(job_id)
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
        schedule_job_log::Entity::delete_by_id(job_id)
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
}
