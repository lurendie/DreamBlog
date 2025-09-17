use std::collections::HashMap;

use crate::models::vo::blog_vo::BlogVO;
use crate::service::{BlogService, CategoryService, TagService};
use crate::{
    middleware::AppClaims,
    models::{vo::blog_visibility::BlogVisibility, vo::page_request::SearchRequest, Result},
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
pub async fn blogs(query: Query<SearchRequest>, _: Authenticated<AppClaims>) -> impl Responder {
    // 定义异步函数，返回一个实现了Responder trait的类型
    let mut map = ValueMap::new(); // 创建一个ValueMap类型的变量
    let page = BlogService::get_blog_all_page(query.0).await; // 调用BlogService的get_blog_all_page方法，传入query.0，获取博客分页数据
    let categories = CategoryService::get_categories().await; // 调用CategoryService的get_categories方法，获取分类数据
    map.insert(to_value!("blogs"), to_value!(page)); // 将博客分页数据插入到map中
    map.insert(to_value!("categories"), to_value!(categories)); // 将分类数据插入到map中
    Result::ok("请求成功".to_string(), Some(to_value!(map))).ok_json() // 返回一个包含map的JSON响应
}

/**
 * 博文可见性 置顶 密码 推荐
 */
#[routes]
#[put("/blog/{blog_id}/visibility")]
pub async fn visibility(
    path: web::Path<u16>,
    mut query: Json<BlogVisibility>,
    _: Authenticated<AppClaims>,
) -> impl Responder {
    let id = path.into_inner();
    query.set_id(id as u32);
    match BlogService::update_visibility(&query).await {
        true => Result::ok_no_data("更新成功".to_string()).ok_json(),
        false => Result::error("更新失败".to_string()).error_json(),
    }
}

#[routes]
#[put("/blog/top")]
pub async fn top(query: Query<BlogVisibility>, _: Authenticated<AppClaims>) -> impl Responder {
    match BlogService::update_visibility(&query).await {
        true => Result::ok_no_data("更新成功".to_string()).ok_json(),
        false => Result::error("更新失败".to_string()).error_json(),
    }
}

#[routes]
#[put("/blog/recommend")]
pub async fn recommend(
    query: Query<BlogVisibility>,
    _: Authenticated<AppClaims>,
) -> impl Responder {
    match BlogService::update_visibility(&query).await {
        true => Result::ok_no_data("更新成功".to_string()).ok_json(),
        false => Result::error("更新失败".to_string()).error_json(),
    }
}
/**
 * 修改文章 获取分类和标签
 */
#[routes]
#[get("/categoryAndTag")]
pub async fn category_and_tag() -> impl Responder {
    let mut map: HashMap<String, Value> = HashMap::new();
    let tag_list = TagService::get_tags().await;
    let category_list = CategoryService::get_list().await;
    map.insert("categories".to_string(), to_value!(category_list));
    map.insert("tags".to_string(), to_value!(tag_list));
    Result::ok("请求成功!".to_string(), Some(to_value!(map))).ok_json()
}

/**
 * 根据ID查询博文
 */
#[routes]
#[get("/blog")]
pub async fn blog(query: Query<HashMap<String, String>>) -> impl Responder {
    let id = query.get("id").unwrap().parse::<u16>().unwrap();
    let blog = BlogService::get_blog_dto(id).await;
    Result::ok("请求成功!".to_string(), Some(to_value!(blog))).ok_json()
}

/**
 * 修改文章
 */
#[routes]
#[put("/blog")]
pub async fn update_blog(query: Json<BlogVO>, _: Authenticated<AppClaims>) -> impl Responder {
    match BlogService::update_blog_dto(query.into_inner()).await {
        true => Result::ok_no_data("更新成功".to_string()).ok_json(),
        false => Result::error("更新失败".to_string()).error_json(),
    }
}
/**
 * 创建文章
 */
#[routes]
#[post("/blog")]
pub async fn create_blog(query: Json<BlogVO>, _: Authenticated<AppClaims>) -> impl Responder {
    match BlogService::update_blog_dto(query.into_inner()).await {
        true => Result::ok_no_data("更新成功".to_string()).ok_json(),
        false => Result::error("更新失败".to_string()).error_json(),
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
) -> impl Responder {
    // 解析参数 id
    let id = match query.get("id") {
        Some(id) => id.parse::<u16>().unwrap_or_default(),
        None => 0,
    };
    if id <= 0 {
        return Result::error("参数错误".to_string()).error_json();
    }
    match BlogService::delete_blog(id).await {
        true => Result::ok_no_data("删除成功".to_string()).ok_json(),
        false => Result::error("删除失败".to_string()).error_json(),
    }
}
