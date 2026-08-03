use crate::app::AppState;
use crate::common::ParamUtils;
use crate::error::AppError;
use crate::error::WebError;
use crate::error::WebErrorCode;
use crate::model::ApiResponse;
use crate::model::SearchRequest;
use crate::service;
use actix_web::routes;
use actix_web::web::{self, Json, Query};
use rbs::value;
use rbs::Value;
use sea_orm::EntityTrait;
use service::BlogService;
use std::collections::HashMap;

//按置顶、创建时间排序 分页查询博客简要信息列表
#[routes]
//#[options("/site")]
#[get("/blogs")]
pub async fn blogs(
    params: Query<SearchRequest>,
    app: web::Data<AppState>,
) -> Result<ApiResponse<Value>, AppError> {
    //提供默认值page_num
    let query = ParamUtils::validate_request_params(&params.0).await?;
    let db_conn = app.get_mysql_pool();

    let data = BlogService::find_list_by_page(query.page_num, db_conn).await?;
    Ok(ApiResponse::success(Some(value!(data))))
}
#[routes]
#[get("/blog")]
pub async fn blog(
    params: Query<HashMap<String, String>>,
    app: web::Data<AppState>,
) -> Result<ApiResponse<Value>, AppError> {
    //获取blog_id参数   不是必要参数，如果没有，则返回参数有误的错误信息
    let id = ParamUtils::get_i64_param(&params, "id")?;

    let blog = BlogService::find_id_detail(id, app.get_mysql_pool())
        .await
        .ok_or_else(|| WebError::Validation("blogid参数有误".to_string()))?;
    Ok(ApiResponse::success(Some(value!(blog))))
}

#[routes]
#[get("/category")]
pub async fn category(
    params: Query<HashMap<String, String>>,
    app: web::Data<AppState>,
) -> Result<ApiResponse<Value>, AppError> {
    let category_name = ParamUtils::get_string_param(&params, "categoryName")?;
    //使用新的分页参数验证方法
    let (page_num, _) = ParamUtils::validate_pagination_params(&params)?;
    let page =
        BlogService::find_by_categorya_name(category_name, page_num as usize, app.get_mysql_pool())
            .await;
    Ok(ApiResponse::success(Some(value!(page))))
}

#[routes]
#[get("/tag")]
pub async fn tag(
    params: Query<HashMap<String, String>>,
    app: web::Data<AppState>,
) -> Result<ApiResponse<Value>, AppError> {
    let tag_name = ParamUtils::get_string_param(&params, "tagName")?;
    //使用新的分页参数验证方法
    let (page_num, _) = ParamUtils::validate_pagination_params(&params)?;
    let page =
        BlogService::find_by_tag_name(tag_name, page_num as usize, app.get_mysql_pool()).await;
    Ok(ApiResponse::success(Some(value!(page))))
}

/**
 * 检测Blog PassWrod 的正确性
 */
#[routes]
#[post("/checkBlogPassword")]
pub async fn check_blog_password(
    query: Json<SearchRequest>,
    app: web::Data<AppState>,
) -> Result<ApiResponse<Value>, AppError> {
    let query = ParamUtils::validate_request_params(&query.0).await?;
    let blog_info = BlogService::find_id_detail(query.blog_id, app.get_mysql_pool())
        .await
        .ok_or_else(|| WebError::NotFound(format!("BlogID:{}文章不存在", query.blog_id)))?;
    let password = query.password;
    // 密码比较直接查库（详情缓存中不保存密码字段）
    let db_password = crate::entity::blog::Entity::find_by_id(query.blog_id)
        .one(app.get_mysql_pool())
        .await
        .ok()
        .flatten()
        .and_then(|m| m.password)
        .unwrap_or_default();
    if db_password == password {
        Ok(ApiResponse::success_with_msg(
            "验证成功,密码正确!",
            Some(value!(blog_info)),
        ))
    } else {
        Ok(ApiResponse::<Value>::error_with_code(
            WebErrorCode::JWT_ERROR,
            "密码错误",
        ))
    }
}

#[routes]
#[get("/searchBlog")]
pub async fn search_blog(
    query: Query<HashMap<String, String>>,
    app: web::Data<AppState>,
) -> Result<ApiResponse<Value>, AppError> {
    let blog_title = ParamUtils::get_string_param(&query, "query")?;
    //查找title内容的文章
    let result = BlogService::search_content(blog_title, app.get_mysql_pool()).await?;
    Ok(ApiResponse::success(Some(value!(result))))
}
