use std::collections::HashMap;

use crate::error::AppError;
use crate::middleware::AppClaims;
use crate::model::{VisitorDeleteParam, VisitorQuery};
use crate::service::VisitorService;
use crate::{app::AppState, model::ApiResponse};
use actix_jwt_session::Authenticated;
use actix_web::{routes, web};
use rbs::{value, Value};

#[routes]
#[get("/visitors")]
pub async fn get_visitor_list(
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
    query: web::Query<VisitorQuery>,
) -> Result<ApiResponse<Value>, AppError> {
    let db = app.get_mysql_pool();
    let map = VisitorService::get_visitor_list(query.0, db).await?;
    let mut result = HashMap::new();
    result.insert("total".to_string(), value!(map.1));
    result.insert("records".to_string(), value!(map.0));
    Ok(ApiResponse::success_with_msg(
        "获取访客列表成功",
        Some(value!(result)),
    ))
}

#[routes]
#[delete("/visitor")]
pub async fn delete_visitor(
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
    params: web::Query<VisitorDeleteParam>,
) -> Result<ApiResponse<Value>, AppError> {
    let db = app.get_mysql_pool();
    let id = params.id;
     let uuid = params.uuid.as_str();
    VisitorService::delete_visitor(id,uuid, db).await?;
    Ok(ApiResponse::success(None))
}
