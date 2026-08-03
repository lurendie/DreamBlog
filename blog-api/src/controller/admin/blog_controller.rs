use std::collections::HashMap;

use crate::app::AppState;
use crate::common::ParamUtils;
use crate::error::AppError;
use crate::model::{ApiResponse, BlogVO};
use crate::service::{BlogService, CategoryService, TagService};
use crate::{
    middleware::AppClaims,
    model::{BlogVisibility, SearchRequest},
};
use actix_jwt_session::Authenticated;
use actix_web::web::Json;
use actix_web::{
    routes,
    web::{self, Query},
};
use rbs::value::map::ValueMap;
use rbs::{value, Value};

#[routes] // 定义路由
#[get("/blogs")] // 定义GET请求的路由
pub async fn blogs(
    query: Query<SearchRequest>,
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
) -> Result<ApiResponse<Value>, AppError> {
    // 定义异步函数，返回一个实现了Responder trait的类型
    let connect = app.get_mysql_pool();
    let mut map = ValueMap::new(); // 创建一个ValueMap类型的变量
    let page = BlogService::find_all_page(query.0, connect).await; // 调用BlogService的get_blog_all_page方法，传入query.0，获取博客分页数据
    let categories = CategoryService::find_categories(connect).await; // 调用CategoryService的get_categories方法，获取分类数据
    map.insert(value!("blogs"), value!(page)); // 将博客分页数据插入到map中
    map.insert(value!("categories"), value!(categories)); // 将分类数据插入到map中
    Ok(ApiResponse::success(Some(value!(map))))
}

/**
 * 博文可见性 置顶 密码 推荐
 */
#[routes]
#[put("/blog/{blog_id}/visibility")]
pub async fn visibility(
    path: web::Path<i64>,
    mut query: Json<BlogVisibility>,
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
) -> Result<ApiResponse<Value>, AppError> {
    let id = path.into_inner();
    query.set_id(id as i64);
    BlogService::update_visibility(&query, app.get_mysql_pool()).await?;
    Ok(ApiResponse::<Value>::success_with_msg("更新成功", None))
}

#[routes]
#[put("/blog/top")]
pub async fn top(
    query: Query<BlogVisibility>,
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
) -> Result<ApiResponse<Value>, AppError> {
    BlogService::update_visibility(&query, app.get_mysql_pool()).await?;
    Ok(ApiResponse::<Value>::success_with_msg("更新成功", None))
}

#[routes]
#[put("/blog/recommend")]
pub async fn recommend(
    query: Query<BlogVisibility>,
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
) -> Result<ApiResponse<Value>, AppError> {
    BlogService::update_visibility(&query, app.get_mysql_pool()).await?;
    Ok(ApiResponse::<Value>::success_with_msg("更新成功", None))
}
/**
 * 修改文章 获取分类和标签
 */
#[routes]
#[get("/categoryAndTag")]
pub async fn category_and_tag(
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
) -> Result<ApiResponse<Value>, AppError> {
    let mut map: HashMap<String, Value> = HashMap::new();
    let connect = app.get_mysql_pool();
    let tag_list = TagService::get_tags(connect).await?;
    let category_list = CategoryService::get_list(connect).await?;
    map.insert("categories".to_string(), value!(category_list));
    map.insert("tags".to_string(), value!(tag_list));
    Ok(ApiResponse::success(Some(value!(map))))
}

/**
 * 根据ID查询博文
 */
#[routes]
#[get("/blog")]
pub async fn blog(
    _: Authenticated<AppClaims>,
    query: Query<HashMap<String, String>>,
    app: web::Data<AppState>,
) -> Result<ApiResponse<Value>, AppError> {
    let id = ParamUtils::get_i64_param(&query.0, "id")?;
    let blog = BlogService::find_by_id(id, app.get_mysql_pool()).await?;
    Ok(ApiResponse::success_with_msg(
        "请求成功!",
        Some(value!(blog)),
    ))
}

/**
 * 修改文章
 */
#[routes]
#[put("/blog")]
pub async fn update_blog(
    query: Json<BlogVO>,
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
) -> Result<ApiResponse<Value>, AppError> {
    BlogService::update_blog(query.into_inner(), app.get_mysql_pool()).await?;
    Ok(ApiResponse::<Value>::success_with_msg("更新成功", None))
}
/**
 * 创建文章
 */
#[routes]
#[post("/blog")]
pub async fn create_blog(
    query: Json<BlogVO>,
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
) -> Result<ApiResponse<Value>, AppError> {
    BlogService::update_blog(query.into_inner(), app.get_mysql_pool()).await?;
    Ok(ApiResponse::<Value>::success_with_msg("创建成功", None))
}

/**
 * 删除文章
 */
#[routes]
#[delete("/blog")]
pub async fn delete_blog(
    query: Query<HashMap<String, String>>,
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
) -> Result<ApiResponse<Value>, AppError> {
    // 解析参数 id
    let id = ParamUtils::get_i64_param(&query.0, "id")?;
    BlogService::delete_by_id(id, app.get_mysql_pool()).await?;
    Ok(ApiResponse::<Value>::success_with_msg("删除成功", None))
}
