use chrono::{Local, NaiveDateTime, NaiveTime};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection, DbBackend, EntityTrait,
    QueryFilter, Statement, TransactionTrait, TryGetable,
};

use crate::entity::{city_visitor, visit_record};
use crate::error::DataBaseError;

pub struct VisitStatsService;

impl VisitStatsService {
    pub async fn aggregate_visit_stats(db: &DatabaseConnection) -> Result<(), DataBaseError> {
        let (today_str, start, end) = today_range();

        let (pv, uv) = Self::query_today_pv_uv(db, start, end).await?;
        Self::upsert_visit_record(db, &today_str, pv, uv).await?;

        let city_stats = Self::query_city_visitors(db).await?;
        Self::replace_city_visitors(db, city_stats).await?;

        Ok(())
    }

    async fn query_today_pv_uv(
        db: &DatabaseConnection,
        start: NaiveDateTime,
        end: NaiveDateTime,
    ) -> Result<(i32, i32), DataBaseError> {
        let sql = Statement::from_sql_and_values(
            DbBackend::MySql,
            r#"
SELECT
  COALESCE(SUM(times), 0) AS pv,
  COUNT(DISTINCT IFNULL(uuid, ip)) AS uv
FROM visit_log
WHERE create_time BETWEEN ? AND ?
"#,
            [start.into(), end.into()],
        );

        let row = db.query_one(sql).await?;
        if let Some(row) = row {
            let pv: i32 = row.try_get("", "pv").unwrap_or(0);
            let uv: i32 = row.try_get("", "uv").unwrap_or(0);
            return Ok((pv, uv));
        }
        Ok((0, 0))
    }

    async fn upsert_visit_record(
        db: &DatabaseConnection,
        date: &str,
        pv: i32,
        uv: i32,
    ) -> Result<(), DataBaseError> {
        let existing = visit_record::Entity::find()
            .filter(visit_record::Column::Date.eq(date.to_string()))
            .one(db)
            .await?;

        match existing {
            Some(model) => {
                let mut active: visit_record::ActiveModel = model.into();
                active.pv = sea_orm::ActiveValue::Set(pv);
                active.uv = sea_orm::ActiveValue::Set(uv);
                active.update(db).await?;
            }
            None => {
                let new_record = visit_record::ActiveModel {
                    id: sea_orm::ActiveValue::NotSet,
                    pv: sea_orm::ActiveValue::Set(pv),
                    uv: sea_orm::ActiveValue::Set(uv),
                    date: sea_orm::ActiveValue::Set(date.to_string()),
                };
                new_record.insert(db).await?;
            }
        }
        Ok(())
    }

    async fn query_city_visitors(
        db: &DatabaseConnection,
    ) -> Result<Vec<(String, i32)>, DataBaseError> {
        let sql = Statement::from_sql_and_values(
            DbBackend::MySql,
            r#"
SELECT
  COALESCE(
    NULLIF(SUBSTRING_INDEX(SUBSTRING_INDEX(ip_source, '|', 4), '|', -1), ''),
    NULLIF(SUBSTRING_INDEX(SUBSTRING_INDEX(ip_source, '|', 3), '|', -1), ''),
    NULLIF(SUBSTRING_INDEX(SUBSTRING_INDEX(ip_source, '|', 2), '|', -1), ''),
    '未知'
  ) AS city,
  COUNT(DISTINCT IFNULL(uuid, ip)) AS uv
FROM visit_log
WHERE ip_source IS NOT NULL AND ip_source <> ''
GROUP BY city
ORDER BY uv DESC
"#,
            [],
        );

        let rows = db.query_all(sql).await?;
        let mut result = Vec::with_capacity(rows.len());
        for row in rows {
            let city: String = row.try_get("", "city").unwrap_or_else(|_| "未知".to_string());
            let uv: i32 = row.try_get("", "uv").unwrap_or(0);
            result.push((city, uv));
        }
        Ok(result)
    }

    async fn replace_city_visitors(
        db: &DatabaseConnection,
        data: Vec<(String, i32)>,
    ) -> Result<(), DataBaseError> {
        db.transaction(|txn| {
            Box::pin(async move {
                city_visitor::Entity::delete_many().exec(txn).await?;
                for (city, uv) in data {
                    let model = city_visitor::ActiveModel {
                        city: sea_orm::ActiveValue::Set(city),
                        uv: sea_orm::ActiveValue::Set(uv),
                    };
                    model.insert(txn).await?;
                }
                Ok(())
            })
        })
        .await?;
        Ok(())
    }
}

fn today_range() -> (String, NaiveDateTime, NaiveDateTime) {
    let today = Local::now().naive_local().date();
    let start = NaiveDateTime::new(today, NaiveTime::from_hms_opt(0, 0, 0).unwrap());
    let end = NaiveDateTime::new(today, NaiveTime::from_hms_opt(23, 59, 59).unwrap());
    let date_str = Local::now().format("%m-%d").to_string();
    (date_str, start, end)
}
