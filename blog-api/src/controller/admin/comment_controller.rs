use std::collections::HashMap;

use actix_jwt_session::Authenticated;
use actix_web::{
    routes,
    web::{self, Data},
    HttpRequest,
};
use rbs::{value, Value};

use crate::{
    app::AppState,
    common::{IpRegion, ParamUtils},
    error::AppError,
    middleware::AppClaims,
    model::{ApiResponse, CommentDTO, CommentNoticeUpdate, CommentPublishedUpdate, SearchRequest},
    service::{BlogService, CommentService},
};

#[routes]
#[get("/comments")]
pub async fn find_comments(
    _: Authenticated<AppClaims>,
    app: Data<AppState>,
    query: web::Query<SearchRequest>,
) -> Result<ApiResponse<Value>, AppError> {
    //验证请求参数
    let query = ParamUtils::validate_request_params(&query.0).await?;
    let comments = CommentService::find_comment_dto(
        query.page_num,
        query.page_size,
        query.page,
        query.blog_id,
        app.get_mysql_pool(),
    )
    .await?;
    Ok(ApiResponse::success_with_msg(
        "请求成功！",
        Some(value!(comments)),
    ))
}

#[routes]
#[get("/blogIdAndTitle")]
pub async fn find_blog_id_and_title(
    _: Authenticated<AppClaims>,
    app: Data<AppState>,
) -> Result<ApiResponse<Value>, AppError> {
    let comments = BlogService::find_blogs_and_title(app.get_mysql_pool()).await?;
    Ok(ApiResponse::success(Some(value!(comments))))
}

//更新评论
#[routes]
#[put("/comment")]
pub async fn update_comment(
    _: Authenticated<AppClaims>,
    app: Data<AppState>,
    comment: web::Json<CommentDTO>,
    req: HttpRequest,
) -> Result<ApiResponse<Value>, AppError> {
    let ip = IpRegion::get_real_client_ip(
        &req,
        crate::app::CONFIG.get_server_config().trust_proxy,
    );
    CommentService::save_comment(comment.0, &app.get_mysql_pool(), ip, true).await?;
    Ok(ApiResponse::<Value>::success_with_msg("更新成功！", None))
}

#[routes]
#[put("/comment/published")]
pub async fn update_comment_published(
    _: Authenticated<AppClaims>,
    app: Data<AppState>,
    query: web::Query<CommentPublishedUpdate>,
) -> Result<ApiResponse<Value>, AppError> {
    CommentService::update_published(query.id, query.published, app.get_mysql_pool()).await?;
    Ok(ApiResponse::<Value>::success_with_msg("更新成功！", None))
}

#[routes]
#[put("/comment/notice")]
pub async fn update_comment_notice(
    _: Authenticated<AppClaims>,
    app: Data<AppState>,
    query: web::Query<CommentNoticeUpdate>,
) -> Result<ApiResponse<Value>, AppError> {
    CommentService::update_notice(query.id, query.notice, app.get_mysql_pool()).await?;
    Ok(ApiResponse::<Value>::success_with_msg("更新成功！", None))
}

//删除评论
#[routes]
#[delete("/comment")]
pub async fn delete_comment(
    _: Authenticated<AppClaims>,
    app: Data<AppState>,
    parameter: web::Query<HashMap<String, String>>,
) -> Result<ApiResponse<Value>, AppError> {
    let id = ParamUtils::get_i64_param(&parameter.0, "id")?;
    CommentService::delete_comment_recursive(id, app.get_mysql_pool()).await?;
    Ok(ApiResponse::<Value>::success_with_msg("删除成功！", None))
}
