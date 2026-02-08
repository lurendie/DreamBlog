use std::collections::HashMap;

use crate::common::ParamUtils;
use crate::error::AppError;
use crate::middleware::AppClaims;
use crate::model::{Friend, FriendCommentEnabledUpdate, FriendContentUpdate, FriendQuery, FriendUpdatePublished};
use crate::service::FriendService;
use crate::{app::AppState, model::ApiResponse};
use actix_jwt_session::Authenticated;
use actix_web::{routes, web};
use rbs::{value, Value};
#[routes]
#[get("/friends")]
pub async fn get_friends_by_query(
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
    query: web::Query<FriendQuery>,
) -> Result<ApiResponse<Value>, AppError> {
    let db = app.get_mysql_pool();
    let page_num = query.page_num.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(10);
    let page_num = page_num.saturating_sub(1).saturating_mul(page_size);
    let data = FriendService::friends_by_query(db, page_num, page_size, query.into_inner()).await?;
    let mut result = HashMap::new();
    result.insert("total".to_string(), value!(data.1));
    result.insert("list".to_string(), value!(data.0.clone()));
    result.insert("records".to_string(), value!(data.0));
    Ok(ApiResponse::<Value>::success_with_msg(
        "获取友链列表成功",
        Some(value!(result)),
    ))
}

#[routes]
#[put("/friend/published")]
pub async fn update_friend_published(
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
    params: web::Query<FriendUpdatePublished>,
) -> Result<ApiResponse<Value>, AppError> {
    let db = app.get_mysql_pool();
    FriendService::update_published(params.0, db).await?;
    Ok(ApiResponse::<Value>::success_with_msg(
        "更新友链发布状态成功",
        None,
    ))
}

#[routes]
#[post("/friend")]
pub async fn save_friend(
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
    friend_form: web::Json<Friend>,
) -> Result<ApiResponse<Value>, AppError> {
    let db = app.get_mysql_pool();
    FriendService::save_friend(friend_form.0, db).await?;
    Ok(ApiResponse::<Value>::success_with_msg("添加友链成功", None))
}

#[routes]
#[put("/friend")]
pub async fn update_friend(
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
    friend_form: web::Json<Friend>,
) -> Result<ApiResponse<Value>, AppError> {
    let db = app.get_mysql_pool();
    let friend_id = friend_form
        .id
        .ok_or_else(|| AppError::Custom("友链ID不能为空".to_string()))?;
    FriendService::update_friend(friend_id, friend_form.0, db).await?;
    Ok(ApiResponse::<Value>::success_with_msg("更新友链成功", None))
}

#[routes]
#[delete("/friend")]
pub async fn delete_friend_by_id(
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
    params: web::Query<HashMap<String, String>>,
) -> Result<ApiResponse<Value>, AppError> {
    let db = app.get_mysql_pool();
    let id = ParamUtils::get_i64_param(&params.0, "id")?;
    FriendService::delete_friend(id, db).await?;
    Ok(ApiResponse::<Value>::success(None))
}

#[routes]
#[get("/friendInfo")]
pub async fn get_friend_info(
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
) -> Result<ApiResponse<Value>, AppError> {
    let data = FriendService::get_friend(app.get_mysql_pool()).await?;
    Ok(ApiResponse::<Value>::success_with_msg(
        "获取友链信息成功",
        Some(value!(data)),
    ))
}

#[routes]
#[put("/friendInfo/commentEnabled")]
pub async fn update_friend_comment_enabled(
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
    payload: web::Query<FriendCommentEnabledUpdate>,
) -> Result<ApiResponse<Value>, AppError> {
    FriendService::update_friend_comment_enabled(payload.comment_enabled, app.get_mysql_pool())
        .await?;
    Ok(ApiResponse::<Value>::success_with_msg(
        "更新友链评论启用状态成功",
        None,
    ))
}

#[routes]
#[put("/friendInfo/content")]
pub async fn update_friend_content(
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
    payload: web::Json<FriendContentUpdate>,
) -> Result<ApiResponse<Value>, AppError> {
    FriendService::update_friend_content(payload.content.clone(), app.get_mysql_pool()).await?;
    Ok(ApiResponse::<Value>::success_with_msg(
        "更新友链内容成功",
        None,
    ))
}
