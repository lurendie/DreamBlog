use chrono::NaiveDateTime;
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
};

use crate::{
    entity::exception_log,
    error::DataBaseError,
    model::{ExceptionLog, ExceptionLogQuery},
};

pub struct ExceptionLogService;

impl ExceptionLogService {
    pub async fn get_exception_log_list(
        query: ExceptionLogQuery,
        db: &DatabaseConnection,
    ) -> Result<(Vec<ExceptionLog>, u64), DataBaseError> {
        let page_num = query.page_num.unwrap_or(1);
        let page_size = query.page_size.unwrap_or(10);
        let mut query_builder = exception_log::Entity::find();

        if let Some(date) = query.date.as_deref() {
            if let Some((start, end)) = parse_date_range(date) {
                query_builder =
                    query_builder.filter(exception_log::Column::CreateTime.between(start, end));
            }
        }

        let paginator = query_builder
            .order_by_desc(exception_log::Column::Id)
            .paginate(db, page_size as u64);
        let total = paginator.num_items().await.unwrap_or(0);
        let models = paginator.fetch_page((page_num - 1) as u64).await?;

        let mut logs = vec![];
        models.into_iter().for_each(|item| {
            logs.push(ExceptionLog::from(item));
        });
        Ok((logs, total))
    }

    pub async fn delete_by_id(db: &DatabaseConnection, id: i64) -> Result<(), DataBaseError> {
        exception_log::Entity::delete_by_id(id).exec(db).await?;
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
