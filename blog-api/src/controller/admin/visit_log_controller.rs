use std::collections::HashMap;

use crate::error::AppError;
use crate::middleware::AppClaims;
use crate::model::{LogIdParam, VisitLogQuery};
use crate::service::VisitService;
use crate::{app::AppState, model::ApiResponse};
use actix_jwt_session::Authenticated;
use actix_web::{routes, web};
use rbs::{value, Value};

#[routes]
#[get("/visitLogs")]
pub async fn get_visit_log_list(
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
    query: web::Query<VisitLogQuery>,
) -> Result<ApiResponse<Value>, AppError> {
    let db = app.get_mysql_pool();
    let page_num = query.page_num.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(10);
    let mut result = HashMap::new();
    let list =
        VisitService::get_visit_log_list(query.0, db, page_num as i64, page_size as i64).await?;
    result.insert("total".to_string(), value!(list.1));
    result.insert("records".to_string(), value!(list.0));
    Ok(ApiResponse::success_with_msg(
        "获取访问日志列表成功",
        Some(value!(result)),
    ))
}

#[routes]
#[delete("/visitLog")]
pub async fn delete_visit_log_by_id(
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
    params: web::Query<LogIdParam>,
) -> Result<ApiResponse<Value>, AppError> {
    let db = app.get_mysql_pool();
    let id = params.id;
    VisitService::delete_by_id(db, id).await?;
    Ok(ApiResponse::<Value>::success_with_msg(
        "删除访问日志成功",
        None,
    ))
}
