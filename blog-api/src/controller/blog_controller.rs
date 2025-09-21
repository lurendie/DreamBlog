use crate::app::AppState;
use crate::common::ParamUtils;
use crate::error::ErrorCode;
use crate::model::SearchRequest;
use crate::model::ApiResponse;
use crate::service;
use actix_web::web::{self, Json, Query};
use actix_web::{routes, Responder};
use rbs::value;
use service::BlogService;
use std::collections::HashMap;

//按置顶、创建时间排序 分页查询博客简要信息列表
#[routes]
//#[options("/site")]
#[get("/blogs")]
pub async fn blogs(params: Query<SearchRequest>, app: web::Data<AppState>) -> impl Responder {
    //提供默认值page_num
    let page_num = params.get_page_num().max(1);
    let db_conn = app.get_mysql_pool();

    match BlogService::find_list_by_page(page_num, db_conn).await {
        Ok(page) => ApiResponse::success(Some(value!(page))).json(),
        Err(e) => {
            ApiResponse::<String>::error_with_code(ErrorCode::DATABASE_ERROR, e.to_string()).json()
        }
    }
}
#[routes]
#[get("/blog")]
pub async fn blog(
    params: Query<HashMap<String, String>>,
    app: web::Data<AppState>,
) -> impl Responder {
    //获取blog_id参数   不是必要参数，如果没有，则返回参数有误的错误信息
    let id = match ParamUtils::get_i64_param(&params, "id") {
        Ok(id) => id,
        Err(e) => {
            return ApiResponse::<String>::error_with_code(e.error_code(), e.message().to_string())
                .json();
        }
    };

    let blog = BlogService::find_id_detail(id, app.get_mysql_pool()).await;
    match blog {
        Some(blog) => ApiResponse::success(Some(value!(blog))).json(),
        None => {
            ApiResponse::<String>::error_with_code(ErrorCode::NOT_FOUND, "博客不存在".to_string())
                .json()
        }
    }
}

#[routes]
#[get("/category")]
pub async fn category(
    params: Query<HashMap<String, String>>,
    app: web::Data<AppState>,
) -> impl Responder {
    let category_name = match ParamUtils::get_string_param(&params, "categoryName") {
        Ok(name) => name,
        Err(e) => {
            return ApiResponse::<String>::error_with_code(e.error_code(), e.message().to_string())
                .json()
        }
    };

    //使用新的分页参数验证方法
    let (page_num, _) = match ParamUtils::validate_pagination_params(&params) {
        Ok(pagination) => pagination,
        Err(e) => {
            return ApiResponse::<String>::error_with_code(e.error_code(), e.message().to_string())
                .json()
        }
    };

    let page =
        BlogService::find_by_categorya_name(category_name, page_num as usize, app.get_mysql_pool())
            .await;
    ApiResponse::success(Some(value!(page))).json()
}

#[routes]
#[get("/tag")]
pub async fn tag(
    params: Query<HashMap<String, String>>,
    app: web::Data<AppState>,
) -> impl Responder {
    let tag_name = match ParamUtils::get_string_param(&params, "tagName") {
        Ok(name) => name,
        Err(e) => {
            return ApiResponse::<String>::error_with_code(e.error_code(), e.message().to_string())
                .json()
        }
    };

    //使用新的分页参数验证方法
    let (page_num, _) = match ParamUtils::validate_pagination_params(&params) {
        Ok(pagination) => pagination,
        Err(e) => {
            return ApiResponse::<String>::error_with_code(e.error_code(), e.message().to_string())
                .json()
        }
    };

    let page =
        BlogService::find_by_tag_name(tag_name, page_num as usize, app.get_mysql_pool()).await;
    ApiResponse::success(Some(value!(page))).json()
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
    let blog_id = data.get_blog_id();

    if blog_id <= 0 {
        return ApiResponse::<String>::error_with_code(
            ErrorCode::VALIDATION_ERROR,
            "博客ID必须大于0".to_string(),
        )
        .json();
    };

    let blog_info = match BlogService::find_id_detail(blog_id, app.get_mysql_pool()).await {
        Some(info) => info,
        None => {
            return ApiResponse::<String>::error_with_code(
                ErrorCode::NOT_FOUND,
                "博客不存在".to_string(),
            )
            .json()
        }
    };

    let password = data.get_password();
    if blog_info.password.clone().unwrap_or_default() == password {
        ApiResponse::success_with_msg("验证成功,密码正确!".to_string(), Some(value!(blog_info)))
            .json()
    } else {
        ApiResponse::<String>::error_with_code(ErrorCode::VALIDATION_ERROR, "密码错误".to_string())
            .json()
    }
}

#[routes]
#[get("/searchBlog")]
pub async fn search_blog(
    query: Query<HashMap<String, String>>,
    app: web::Data<AppState>,
) -> impl Responder {
    let blog_title = match ParamUtils::get_string_param(&query, "query") {
        Ok(title) => title,
        Err(e) => {
            return ApiResponse::<String>::error_with_code(e.error_code(), e.message().to_string())
                .json()
        }
    };

    //查找title内容的文章
    match BlogService::search_content(blog_title, app.get_mysql_pool()).await {
        Ok(result) => ApiResponse::success(Some(value!(result))).json(),
        Err(e) => {
            ApiResponse::<String>::error_with_code(ErrorCode::DATABASE_ERROR, e.to_string()).json()
        }
    }
}
