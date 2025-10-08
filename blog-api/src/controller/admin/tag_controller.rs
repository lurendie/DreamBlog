use std::collections::HashMap;

use crate::{
    app::AppState,
    common::ParamUtils,
    error::AppError,
    middleware::AppClaims,
    model::{ApiResponse, SearchRequest, TagDTO},
    service::TagService,
};
use actix_jwt_session::Authenticated;
use actix_web::{routes, web};
use rbs::{value, Value};

#[routes]
#[get("/tags")]
pub async fn get_all_tags(
    _: Authenticated<AppClaims>,
    params: web::Query<SearchRequest>,
    app: web::Data<AppState>,
) -> Result<ApiResponse<Value>, AppError> {
    ParamUtils::validate_request_params(&params).await?;
    let tags_result = TagService::get_tags_by_page(
        params.get_page_num(),
        params.get_page_size(),
        app.get_mysql_pool(),
    )
    .await?;
    Ok(ApiResponse::success(Some(value!(tags_result))))
}

#[routes]
#[put("/tag")]
#[post("/tag")]
pub async fn insert_or_update(
    _: Authenticated<AppClaims>,
    tag: web::Json<TagDTO>,
    app: web::Data<AppState>,
) -> Result<ApiResponse<Value>, AppError> {
    TagService::insert_or_update(tag.into_inner(), app.get_mysql_pool()).await?;
    Ok(ApiResponse::<Value>::success_with_msg("操作成功！", None))
}

#[routes]
#[delete("/tag")]
pub async fn delete_by_id(
    _: Authenticated<AppClaims>,
    query: web::Query<HashMap<String, String>>,
    app: web::Data<AppState>,
) -> Result<ApiResponse<Value>, AppError> {
    let id = ParamUtils::get_i64_param(&query, "id")?;
    TagService::delete_by_id(id, app.get_mysql_pool()).await?;
    Ok(ApiResponse::<Value>::success_with_msg("操作成功！", None))
}
