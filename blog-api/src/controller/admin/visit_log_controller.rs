use std::collections::HashMap;

use crate::entity::visit_log;
use crate::middleware::AppClaims;
use crate::model::VisitLog;
use crate::{app_state::AppState, model::ApiResponse};
use actix_jwt_session::Authenticated;
use actix_web::{routes, web, Responder};
use rbs::to_value;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct VisitLogQuery {
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
    pub uri: Option<String>,
    pub ip: Option<String>,
    pub behavior: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LogIdParam {
    pub id: i64,
}

#[routes]
#[get("/visitLogs")]
pub async fn get_visit_log_list(
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
    query: web::Query<VisitLogQuery>,
) -> impl Responder {
    let db = app.get_mysql_pool();
    let page_num = query.page_num.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(10);

    // 构建查询条件
    let mut query_builder = visit_log::Entity::find();

    if let Some(uri) = &query.uri {
        query_builder = query_builder.filter(visit_log::Column::Uri.contains(uri));
    }

    if let Some(ip) = &query.ip {
        query_builder = query_builder.filter(visit_log::Column::Ip.contains(ip));
    }

    if let Some(behavior) = &query.behavior {
        query_builder = query_builder.filter(visit_log::Column::Behavior.contains(behavior));
    }

    // 获取分页数据
    let paginator = query_builder
        .order_by_desc(visit_log::Column::Id)
        .paginate(db, page_size as u64);

    let total = paginator.num_items().await.unwrap_or(0);
    let logs = paginator.fetch_page((page_num - 1) as u64).await;

    match logs {
        Ok(log_models) => {
            let mut result = HashMap::new();
            let mut logs = vec![];
            log_models.into_iter().for_each(|item| {
                logs.push(VisitLog::from(item));
            });
            result.insert("total".to_string(), to_value!(total));
            result.insert("records".to_string(), to_value!(logs));
            ApiResponse::success_with_msg(
                "获取访问日志列表成功".to_string(),
                Some(to_value!(result)),
            )
            .json()
        }
        Err(e) => ApiResponse::<String>::error(format!("获取访问日志列表失败: {}", e)).json(),
    }
}

#[routes]
#[delete("/visitLog")]
pub async fn delete_visit_log_by_id(
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
    params: web::Query<LogIdParam>,
) -> impl Responder {
    let db = app.get_mysql_pool();
    let id = params.id;

    match visit_log::Entity::delete_by_id(id).exec(db).await {
        Ok(result) => {
            if result.rows_affected > 0 {
                ApiResponse::<String>::success_with_msg("删除访问日志成功".to_string(), None).json()
            } else {
                ApiResponse::<String>::error("访问日志不存在".to_string()).json()
            }
        }
        Err(e) => ApiResponse::<String>::error(format!("删除访问日志失败: {}", e)).json(),
    }
}
