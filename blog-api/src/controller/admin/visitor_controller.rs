use std::collections::HashMap;

use crate::entity::visitor;
use crate::middleware::AppClaims;
use crate::model::Visitor;
use crate::{app_state::AppState, model::ApiResponse};
use actix_jwt_session::Authenticated;
use actix_web::{routes, web, Responder};
use rbs::to_value;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct VisitorQuery {
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
    pub ip: Option<String>,
    pub ip_source: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct VisitorDeleteParam {
    pub id: i64,
    pub uuid: String,
}

#[routes]
#[get("/visitors")]
pub async fn get_visitor_list(
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
    query: web::Query<VisitorQuery>,
) -> impl Responder {
    let db = app.get_mysql_pool();
    let page_num = query.page_num.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(10);

    // 构建查询条件
    let mut query_builder = visitor::Entity::find();

    if let Some(ip) = &query.ip {
        query_builder = query_builder.filter(visitor::Column::Ip.contains(ip));
    }

    if let Some(ip_source) = &query.ip_source {
        query_builder = query_builder.filter(visitor::Column::IpSource.contains(ip_source));
    }

    // 获取分页数据
    let paginator = query_builder
        .order_by_desc(visitor::Column::Id)
        .paginate(db, page_size as u64);

    let total = paginator.num_items().await.unwrap_or(0);
    let visitors = paginator.fetch_page((page_num - 1) as u64).await;

    match visitors {
        Ok(visitor_models) => {
            let mut result = HashMap::new();
            let mut visitors = vec![];
            visitor_models.into_iter().for_each(|item| {
                visitors.push(Visitor::from(item));
            });
            result.insert("total".to_string(), to_value!(total));
            result.insert("records".to_string(), to_value!(visitors));
            ApiResponse::success_with_msg("获取访客列表成功".to_string(), Some(to_value!(result)))
                .json()
        }
        Err(e) => ApiResponse::<String>::error(format!("获取访客列表失败: {}", e)).json(),
    }
}

#[routes]
#[delete("/visitor")]
pub async fn delete_visitor(
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
    params: web::Query<VisitorDeleteParam>,
) -> impl Responder {
    let db = app.get_mysql_pool();
    let id = params.id;
    let uuid = &params.uuid;

    // 检查访客是否存在
    match visitor::Entity::find_by_id(id)
        .filter(visitor::Column::Uuid.eq(uuid))
        .one(db)
        .await
    {
        Ok(Some(_)) => {
            // 访客存在，执行删除
            match visitor::Entity::delete_by_id(id).exec(db).await {
                Ok(result) => {
                    if result.rows_affected > 0 {
                        ApiResponse::<String>::success_with_msg("删除访客成功".to_string(), None)
                            .json()
                    } else {
                        ApiResponse::<String>::error("删除访客失败".to_string()).json()
                    }
                }
                Err(e) => ApiResponse::<String>::error(format!("删除访客失败: {}", e)).json(),
            }
        }
        Ok(None) => ApiResponse::<String>::error("访客不存在".to_string()).json(),
        Err(e) => ApiResponse::<String>::error(format!("查询访客失败: {}", e)).json(),
    }
}
