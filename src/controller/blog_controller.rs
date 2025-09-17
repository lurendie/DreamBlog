use crate::app_state::AppState;
use crate::model::SearchRequest;
use crate::model::ResponseResult;
use crate::service;
use actix_web::web::{self, Json, Query};
use actix_web::{routes, HttpResponse, Responder};
use rbs::{to_value, Value};
use service::BlogService;
use std::collections::HashMap;

//按置顶、创建时间排序 分页查询博客简要信息列表
#[routes]
//#[options("/site")]
#[get("/blogs")]
pub async fn blogs(mut params: Query<SearchRequest>, app: web::Data<AppState>) -> impl Responder {
    //提供默认值page_num.expect("异常！")
    if params.get_page_num() <= 0 {
        params.set_page_num(Some(1));
    };
    let db_conn = app.get_mysql_pool();

    let page = match BlogService::find_list_by_page(params.get_page_num(), db_conn).await {
        Ok(page) => page,
        Err(e) => {
            return ResponseResult::error(e.to_string()).json();
        }
    };
    let result: ResponseResult<HashMap<String, Value>> =
        ResponseResult::<HashMap<String, Value>>::ok(String::from("请求成功！"), Some(page));
    HttpResponse::Ok().json(result)
}
#[routes]
#[get("/blog")]
pub async fn blog(
    params: Query<HashMap<String, String>>,
    app: web::Data<AppState>,
) -> impl Responder {
    //获取blog_id参数   不是必要参数，如果没有，则返回参数有误的错误信息
    let id: i64 = match params.get("id") {
        Some(id) => id.parse().unwrap_or_default(),
        None => return ResponseResult::error("参数有误!".to_string()).json(),
    };
    //如果id<=0，则返回参数有误的错误信息
    if id <= 0 {
        return ResponseResult::error("参数有误!".to_string()).json();
    }
    let blog = BlogService::find_id_detail(id, app.get_mysql_pool()).await;
    match blog {
        Some(blog) => {
            return ResponseResult::ok("请求成功".to_string(), Some(to_value!(blog))).json();
        }
        None => {
            return ResponseResult::error("没有找到该文章!".to_string()).json();
        }
    }
}

#[routes]
#[get("/category")]
pub async fn category(
    params: Query<HashMap<String, String>>,
    app: web::Data<AppState>,
) -> impl Responder {
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
        return ResponseResult::error("参数有误!".to_string()).json();
    }
    let page = BlogService::find_by_categorya_name(category_name, page, app.get_mysql_pool()).await;
    let result: ResponseResult<HashMap<String, Value>> =
        ResponseResult::<HashMap<String, Value>>::ok(String::from("请求成功！"), Some(page));
    HttpResponse::Ok().json(result)
}

#[routes]
#[get("/tag")]
pub async fn tag(
    params: Query<HashMap<String, String>>,
    app: web::Data<AppState>,
) -> impl Responder {
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
        return ResponseResult::error("参数有误!".to_string()).json();
    }
    let page = BlogService::find_by_tag_name(tag_name, page, app.get_mysql_pool()).await;
    let result: ResponseResult<HashMap<String, Value>> =
        ResponseResult::<HashMap<String, Value>>::ok(String::from("请求成功！"), Some(page));
    HttpResponse::Ok().json(result)
}

/**
 * 检测Blog PassWrod 的正确性
 */
#[routes]
#[post("/checkBlogPassword")]
pub async fn check_blog_password(
    data: Json<SearchRequest>,
    app: web::Data<AppState>,
) -> impl Responder {
    if data.get_blog_id() > 0 {
        let blog_info = BlogService::find_id_detail(data.get_blog_id(), app.get_mysql_pool()).await;
        if let Some(blog_info) = blog_info {
            if blog_info.password.clone().unwrap_or_default() == data.get_password() {
                return ResponseResult::ok(
                    "验证成功,密码正确!".to_string(),
                    Some(to_value!(blog_info)),
                )
                .json();
            }
        }
    }
    ResponseResult::error("参数有误!".to_string()).json()
}

#[routes]
#[get("/searchBlog")]
pub async fn search_blog(
    query: Query<HashMap<String, String>>,
    app: web::Data<AppState>,
) -> impl Responder {
    let blog_title = match query.get("query") {
        Some(title) => title.clone(),
        None => String::new(),
    };
    if blog_title.is_empty() {
        return ResponseResult::error("参数有误!".to_string()).json();
    }
    //查找title内容的文章
    match BlogService::search_content(blog_title, app.get_mysql_pool()).await {
        Ok(result) => ResponseResult::ok("请求成功".to_string(), Some(to_value!(result))).json(),
        Err(e) => ResponseResult::error(e.to_string()).json(),
    }
}
