use std::collections::HashMap;

use actix_jwt_session::Authenticated;
use actix_web::{routes, web};
use rbs::{value, Value};

use crate::{
    app::AppState,
    error::AppError,
    middleware::AppClaims,
    model::{ExceptionLogQuery, LogIdParam},
    service::ExceptionLogService,
    model::ApiResponse,
};

#[routes]
#[get("/exceptionLogs")]
pub async fn get_exception_log_list(
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
    query: web::Query<ExceptionLogQuery>,
) -> Result<ApiResponse<Value>, AppError> {
    let db = app.get_mysql_pool();
    let list = ExceptionLogService::get_exception_log_list(query.0, db).await?;
    let mut result = HashMap::new();
    result.insert("total".to_string(), value!(list.1));
    result.insert("list".to_string(), value!(list.0));
    Ok(ApiResponse::success_with_msg(
        "获取异常日志列表成功",
        Some(value!(result)),
    ))
}

#[routes]
#[delete("/exceptionLog")]
pub async fn delete_exception_log_by_id(
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
    params: web::Query<LogIdParam>,
) -> Result<ApiResponse<Value>, AppError> {
    let db = app.get_mysql_pool();
    let id = params.id;
    ExceptionLogService::delete_by_id(db, id).await?;
    Ok(ApiResponse::<Value>::success_with_msg(
        "删除异常日志成功",
        None,
    ))
}
