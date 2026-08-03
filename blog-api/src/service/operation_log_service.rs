use chrono::NaiveDateTime;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait,
    PaginatorTrait, QueryFilter, QueryOrder,
};

use crate::{
    entity::operation_log,
    error::DataBaseError,
    model::{OperationLog, OperationLogQuery},
};

pub struct OperationLogService;

impl OperationLogService {
    #[allow(clippy::too_many_arguments)]
    pub async fn save_operation_log(
        db: &DatabaseConnection,
        username: String,
        uri: String,
        method: String,
        param: Option<String>,
        description: Option<String>,
        ip: Option<String>,
        ip_source: Option<String>,
        os: Option<String>,
        browser: Option<String>,
        times: i32,
        create_time: chrono::NaiveDateTime,
        user_agent: Option<String>,
    ) -> Result<(), DataBaseError> {
        let new_log = operation_log::ActiveModel {
            username: Set(username),
            uri: Set(uri),
            method: Set(method),
            param: Set(param),
            description: Set(description),
            ip: Set(ip),
            ip_source: Set(ip_source),
            os: Set(os),
            browser: Set(browser),
            times: Set(times),
            create_time: Set(create_time),
            user_agent: Set(user_agent),
            ..Default::default()
        };

        new_log.save(db).await?;
        Ok(())
    }

    pub async fn get_operation_log_list(
        query: OperationLogQuery,
        db: &DatabaseConnection,
    ) -> Result<(Vec<OperationLog>, u64), DataBaseError> {
        let page_num = query.page_num.unwrap_or(1).max(1);
        let page_size = query.page_size.unwrap_or(10).max(1);
        let mut query_builder = operation_log::Entity::find();

        if let Some(date) = query.date.as_deref() {
            if let Some((start, end)) = parse_date_range(date) {
                query_builder =
                    query_builder.filter(operation_log::Column::CreateTime.between(start, end));
            }
        }

        let paginator = query_builder
            .order_by_desc(operation_log::Column::Id)
            .paginate(db, page_size as u64);
        let total = paginator.num_items().await.unwrap_or(0);
        let models = paginator.fetch_page((page_num - 1) as u64).await?;

        let mut logs = vec![];
        models.into_iter().for_each(|item| {
            logs.push(OperationLog::from(item));
        });
        Ok((logs, total))
    }

    pub async fn delete_by_id(db: &DatabaseConnection, id: i64) -> Result<(), DataBaseError> {
        operation_log::Entity::delete_by_id(id).exec(db).await?;
        Ok(())
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
