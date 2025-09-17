use crate::models::vo::page_request::SearchRequest;
use crate::models::vo::result::Result;
use crate::service;
use actix_web::web::{Json, Path, Query};
use actix_web::{routes, HttpResponse, Responder};
use rbs::{to_value, Value};
use service::BlogService;
use std::collections::HashMap;

//按置顶、创建时间排序 分页查询博客简要信息列表
#[routes]
//#[options("/site")]
#[get("/blogs")]
pub async fn blogs(mut params: Query<SearchRequest>) -> impl Responder {
    //提供默认值page_num.expect("异常！")
    if params.get_page_num() <= 0 {
        params.set_page_num(Some(1));
    };
    let page = BlogService::get_blog_list_by_is_published(params.get_page_num() as u64).await;
    let result: Result<HashMap<String, Value>> =
        Result::<HashMap<String, Value>>::ok(String::from("请求成功！"), Some(page));
    HttpResponse::Ok().json(result)
}
#[routes]
#[get("/blog")]
pub async fn blog(params: Query<HashMap<String, String>>) -> impl Responder {
    let id: u16 = match params.get("id") {
        Some(id) => id.parse().expect("转换失败"),
        None => 0,
    };
    //如果id<=0，则返回参数有误的错误信息
    if id <= 0 {
        return Result::error("参数有误!".to_string()).error_json();
    }
    let blog = BlogService::get_by_id(id).await;
    let result = Result::new(200, "请求成功".to_string(), blog);
    HttpResponse::Ok().json(result)
}
#[routes]
#[get("/category")]
pub async fn category(params: Query<HashMap<String, String>>) -> impl Responder {
    let category_name: String = match params.get("categoryName") {
        Some(category_name) => category_name.clone(),
        None => String::new(),
    };
    //如果没有page_num参数，则默认取第一页
    let page: usize = match params.get("pageNum") {
        Some(page) => page.parse().expect("转换失败"),
        None => 1,
    };
    if category_name.is_empty() {
        return Result::error("参数有误!".to_string()).error_json();
    }
    let page = BlogService::get_by_name(category_name, page).await;
    let result: Result<HashMap<String, Value>> =
        Result::<HashMap<String, Value>>::ok(String::from("请求成功！"), Some(page));
    HttpResponse::Ok().json(result)
}
#[routes]
#[get("/tag")]
pub async fn tag(params: Query<HashMap<String, String>>) -> impl Responder {
    let tag_name: String = match params.get("tagName") {
        Some(category_name) => category_name.clone(),
        None => String::new(),
    };
    //如果没有page_num参数，则默认取第一页
    let page: usize = match params.get("pageNum") {
        Some(page) => page.parse().expect("转换失败"),
        None => 1,
    };
    if tag_name.is_empty() {
        return Result::error("参数有误!".to_string()).error_json();
    }
    let page = BlogService::get_by_tag_name(tag_name, page).await;
    let result: Result<HashMap<String, Value>> =
        Result::<HashMap<String, Value>>::ok(String::from("请求成功！"), Some(page));
    HttpResponse::Ok().json(result)
}

/**
 * 检测Blog PassWrod 的正确性
 */
#[routes]
#[post("/checkBlogPassword")]
pub async fn check_blog_password(data: Json<SearchRequest>) -> impl Responder {
    if data.get_blog_id() > 0 {
        let blog_info = BlogService::get_by_id(data.get_blog_id()).await;
        if let Some(blog_info) = &blog_info {
            if let Some(password) = &blog_info.password {
                if *password == data.get_password() {
                    return Result::ok(
                        "验证成功,密码正确!".to_string(),
                        Some(to_value!(blog_info)),
                    )
                    .ok_json();
                }
            }
        }
    }
    Result::error("参数有误!".to_string()).error_json()
}

#[routes]
#[get("/searchBlog")]
pub async fn search_blog(query: Query<HashMap<String, String>>) -> impl Responder {
    let blog_title = match query.get("query") {
        Some(title) => title.clone(),
        None => String::new(),
    };
    if blog_title.is_empty() {
        return Result::error("参数有误!".to_string()).error_json();
    }
    //查找title内容的文章
    let result = to_value!(BlogService::search_blog(blog_title).await);
    Result::ok("请求成功".to_string(), Some(result)).ok_json()
}

#[routes]
#[post("/moment/like/{id}")]
pub async fn moment_like(id: Path<u16>) -> impl Responder {
    if *id <= 0 {
        return Result::error("参数有误!".to_string()).error_json();
    }
    let result = BlogService::moment_like(*id).await;
    if let Ok(row) = result {
        if row > 0 {
            return Result::ok("点赞成功".to_string(), Some(to_value!(row))).ok_json();
        } else {
            return Result::error("点赞失败".to_string()).error_json();
        }
    }
    Result::error("参数有误!".to_string()).error_json()
}
