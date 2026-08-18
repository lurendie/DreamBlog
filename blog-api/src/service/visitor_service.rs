use sea_orm::prelude::Expr;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel,
    PaginatorTrait, QueryFilter, QueryOrder,
};

use crate::entity::visitor;
use crate::error::DataBaseError;
use crate::model::{Visitor, VisitorQuery};
use chrono::NaiveDateTime;

pub struct VisitorService;

impl VisitorService {
    /**
     * 根据UUID获取访客
     */
    pub async fn get_by_uuid(uuid: &str, db: &DatabaseConnection) -> Option<Visitor> {
        match visitor::Entity::find()
            .filter(visitor::Column::Uuid.eq(uuid))
            .one(db)
            .await
            .ok()
        {
            Some(visitor) => match visitor {
                Some(visitor) => Some(visitor.into()),
                None => None,
            },
            None => None,
        }
    }
    /**
     * 保存访客
     */
    pub async fn save_visitor(
        visitor: Visitor,
        db: &DatabaseConnection,
    ) -> Result<(), DataBaseError> {
        match VisitorService::get_by_uuid(&visitor.uuid, db).await {
            Some(model) => {
                visitor::Entity::update_many()
                    .col_expr(visitor::Column::Pv, Expr::col(visitor::Column::Pv).add(1).into())
                    .col_expr(
                        visitor::Column::Ip,
                        Expr::value(visitor.ip.unwrap_or_default()).into(),
                    )
                    .col_expr(
                        visitor::Column::IpSource,
                        Expr::value(visitor.ip_source.unwrap_or_default()).into(),
                    )
                    .col_expr(
                        visitor::Column::Os,
                        Expr::value(visitor.os.unwrap_or_default()).into(),
                    )
                    .col_expr(
                        visitor::Column::Browser,
                        Expr::value(visitor.browser.unwrap_or_default()).into(),
                    )
                    .col_expr(
                        visitor::Column::LastTime,
                        Expr::value(visitor.last_time).into(),
                    )
                    .col_expr(
                        visitor::Column::UserAgent,
                        Expr::value(visitor.user_agent.unwrap_or_default()).into(),
                    )
                    .filter(visitor::Column::Id.eq(model.id))
                    .exec(db)
                    .await?;
            }
            None => {
                let mut model = visitor::Model::from(visitor).into_active_model();
                model.not_set(visitor::Column::Id);
                model.save(db).await?;
            }
        }
        Ok(())
    }

    /**
     * 根据id删除访客
     */
    pub async fn delete_visitor(
        id: i64,
        uuid: &str,
        db: &DatabaseConnection,
    ) -> Result<(), DataBaseError> {
        let delete_visitor = visitor::ActiveModel {
            id: Set(id),
            uuid: Set(uuid.to_string()),
            ..Default::default()
        };
        visitor::Entity::delete(delete_visitor).exec(db).await?;
        Ok(())
    }

    pub async fn get_visitor_list(
        query: VisitorQuery,
        db: &DatabaseConnection,
    ) -> Result<(Vec<Visitor>, u64), DataBaseError> {
        let page_num = query.page_num.unwrap_or(1).max(1);
        let page_size = query.page_size.unwrap_or(10).max(1);
        // 构建查询条件
        let mut query_builder = visitor::Entity::find();

        if let Some(ip) = &query.ip {
            query_builder = query_builder.filter(visitor::Column::Ip.contains(ip));
        }

        if let Some(ip_source) = &query.ip_source {
            query_builder = query_builder.filter(visitor::Column::IpSource.contains(ip_source));
        }

        if let Some(date) = &query.date {
            // "开始,结束"（YYYY-MM-DD HH:mm:ss），过滤 last_time 区间；只给开始则只限下限
            let parts: Vec<&str> = date.split(',').collect();
            if let Some(start) = parts
                .first()
                .and_then(|p| NaiveDateTime::parse_from_str(p.trim(), "%Y-%m-%d %H:%M:%S").ok())
            {
                query_builder = query_builder.filter(visitor::Column::LastTime.gte(start));
                if let Some(end) = parts.get(1).and_then(|p| {
                    NaiveDateTime::parse_from_str(p.trim(), "%Y-%m-%d %H:%M:%S").ok()
                }) {
                    query_builder = query_builder.filter(visitor::Column::LastTime.lte(end));
                }
            }
        }

        // 获取分页数据
        let paginator = query_builder
            .order_by_desc(visitor::Column::Id)
            .paginate(db, page_size as u64);

        let total = paginator.num_items().await.unwrap_or(0);
        let visitor_models = paginator.fetch_page((page_num - 1) as u64).await?;

        let mut visitors = vec![];
        visitor_models.into_iter().for_each(|item| {
            visitors.push(Visitor::from(item));
        });
        Ok((visitors, total))
    }
}
