use std::collections::HashMap;

use crate::app_state::AppState;
use crate::model::BlogVO;
use crate::service::{BlogService, CategoryService, TagService};
use crate::{
    middleware::AppClaims,
    model::{BlogVisibility, ResponseResult, SearchRequest},
};
use actix_jwt_session::Authenticated;
use actix_web::web::Json;
use actix_web::{
    routes,
    web::{self, Query},
    Responder,
};
use rbs::value::map::ValueMap;
use rbs::{to_value, Value};

#[routes] // 定义路由
#[get("/blogs")] // 定义GET请求的路由
pub async fn blogs(
    query: Query<SearchRequest>,
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
) -> impl Responder {
    // 定义异步函数，返回一个实现了Responder trait的类型
    let connect = app.get_mysql_pool();
    let mut map = ValueMap::new(); // 创建一个ValueMap类型的变量
    let page = BlogService::find_all_page(query.0, connect).await; // 调用BlogService的get_blog_all_page方法，传入query.0，获取博客分页数据
    let categories = CategoryService::find_categories(connect).await; // 调用CategoryService的get_categories方法，获取分类数据
    map.insert(to_value!("blogs"), to_value!(page)); // 将博客分页数据插入到map中
    map.insert(to_value!("categories"), to_value!(categories)); // 将分类数据插入到map中
    ResponseResult::ok("请求成功".to_string(), Some(to_value!(map))).json()
    // 返回一个包含map的JSON响应
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
) -> impl Responder {
    let id = path.into_inner();
    query.set_id(id as i64);
    match BlogService::update_visibility(&query, app.get_mysql_pool()).await {
        Ok(_) => ResponseResult::ok_no_data("更新成功".to_string()).json(),
        Err(e) => ResponseResult::error(e.to_string()).json(),
    }
}

#[routes]
#[put("/blog/top")]
pub async fn top(
    query: Query<BlogVisibility>,
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
) -> impl Responder {
    match BlogService::update_visibility(&query, app.get_mysql_pool()).await {
        Ok(_) => ResponseResult::ok_no_data("更新成功".to_string()).json(),
        Err(e) => ResponseResult::error(e.to_string()).json(),
    }
}

#[routes]
#[put("/blog/recommend")]
pub async fn recommend(
    query: Query<BlogVisibility>,
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
) -> impl Responder {
    match BlogService::update_visibility(&query, app.get_mysql_pool()).await {
        Ok(_) => ResponseResult::ok_no_data("更新成功".to_string()).json(),
        Err(e) => ResponseResult::error(e.to_string()).json(),
    }
}
/**
 * 修改文章 获取分类和标签
 */
#[routes]
#[get("/categoryAndTag")]
pub async fn category_and_tag(app: web::Data<AppState>) -> impl Responder {
    let mut map: HashMap<String, Value> = HashMap::new();
    let connect = app.get_mysql_pool();
    let tag_list = match TagService::get_tags(connect).await {
        Ok(tag_list) => tag_list,
        Err(e) => return ResponseResult::error(e.to_string()).json(),
    };
    let category_list = match CategoryService::get_list(connect).await {
        Ok(category_list) => category_list,
        Err(e) => return ResponseResult::error(e.to_string()).json(),
    };
    map.insert("categories".to_string(), to_value!(category_list));
    map.insert("tags".to_string(), to_value!(tag_list));
    ResponseResult::ok("请求成功!".to_string(), Some(to_value!(map))).json()
}

/**
 * 根据ID查询博文
 */
#[routes]
#[get("/blog")]
pub async fn blog(
    query: Query<HashMap<String, String>>,
    app: web::Data<AppState>,
) -> impl Responder {
    let id = query
        .get("id")
        .unwrap_or(&"0".to_string())
        .parse::<u16>()
        .unwrap_or_default();
    if id <= 0 {
        return ResponseResult::error("参数错误".to_string()).json();
    }
    let blog = BlogService::find_by_id(id, app.get_mysql_pool()).await;
    match blog {
        Ok(blog) => ResponseResult::ok("请求成功!".to_string(), Some(to_value!(blog))).json(),
        Err(e) => ResponseResult::error(e.to_string()).json(),
    }
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
) -> impl Responder {
    match BlogService::update_blog(query.into_inner(), app.get_mysql_pool()).await {
        Ok(_) => ResponseResult::ok_no_data("更新成功".to_string()).json(),
        Err(e) => ResponseResult::error(e.to_string()).json(),
    }
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
) -> impl Responder {
    match BlogService::update_blog(query.into_inner(), app.get_mysql_pool()).await {
        Ok(_) => ResponseResult::ok_no_data("更新成功".to_string()).json(),
        Err(e) => ResponseResult::error(e.to_string()).json(),
    }
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
) -> impl Responder {
    // 解析参数 id
    let id = match query.get("id") {
        Some(id) => id.parse::<i64>().unwrap_or_default(),
        None => 0,
    };
    if id <= 0 {
        return ResponseResult::error("参数错误".to_string()).json();
    }
    match BlogService::delete_by_id(id, app.get_mysql_pool()).await {
        Ok(_) => ResponseResult::ok_no_data("删除成功".to_string()).json(),
        Err(e) => ResponseResult::error(e.to_string()).json(),
    }
}
