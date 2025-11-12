use crate::app::AppState;
use crate::common::ParamUtils;
use crate::error::AppError;
use crate::middleware::AppClaims;
use crate::model::ApiResponse;
use crate::model::MomentDTO;
use crate::model::SearchRequest;
use crate::service::MomentService;
use actix_jwt_session::Authenticated;
use actix_web::{routes, web};
use rbs::value;
use rbs::Value;
use std::collections::HashMap;

#[routes]
#[get("/moments")]
pub async fn moments(
    _: Authenticated<AppClaims>,
    mut query: web::Query<SearchRequest>,
    app: web::Data<AppState>,
) -> Result<ApiResponse<Value>, AppError> {
    query.0.set_page_size(Some(5));
    //查询所有moments
    let query = ParamUtils::validate_request_params(&query).await?;
    //分页查询
    let value_map =
        MomentService::get_moments(query.page_num, query.page_size, app.get_mysql_pool()).await?;
    Ok(ApiResponse::success(Some(value!(value_map))))
}

/**
 * 动态发布状态
 */

#[routes]
#[put("/moment/published")]
pub async fn moment_published(
    query: web::Query<HashMap<String, String>>,
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
) -> Result<ApiResponse<Value>, AppError> {
    let id = ParamUtils::get_i64_param(&query, "id")?;
    let is_published = ParamUtils::get_bool_param(&query, "published")?;
    MomentService::update_published(id, is_published, app.get_mysql_pool()).await?;
    Ok(ApiResponse::<Value>::success_with_msg("更新成功", None))
}

#[routes]
#[get("/moment")]
pub async fn get_moment_by_id(
    query: web::Query<HashMap<String, String>>,
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
) -> Result<ApiResponse<Value>, AppError> {
    let id = ParamUtils::get_i64_param(&query, "id")?;
    let moment = MomentService::get_moment_by_id(id, app.get_mysql_pool()).await?;
    Ok(ApiResponse::success(Some(value!(moment))))
}

/**
 * 删除动态
 */

#[routes]
#[delete("/moment")]
pub async fn delete_moment(
    query: web::Query<HashMap<String, String>>,
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
) -> Result<ApiResponse<Value>, AppError> {
    let id = ParamUtils::get_i64_param(&query, "id")?;
    MomentService::delete_moment(id, app.get_mysql_pool()).await?;
    Ok(ApiResponse::<Value>::success_with_msg("删除成功", None))
}

/**
 * 更新动态
 */
#[routes]
#[post("/moment")]
#[put("/moment")]
pub async fn create_and_update(
    moment: web::Json<MomentDTO>,
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
) -> Result<ApiResponse<Value>, AppError> {
    MomentService::create_and_update(moment.into_inner(), app.get_mysql_pool()).await?;
    Ok(ApiResponse::<Value>::success_with_msg("更新成功", None))
}
