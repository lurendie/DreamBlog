use std::collections::HashMap;

use crate::app::AppState;
use crate::error::WebErrorCode;
use crate::model::Category;
use crate::model::SearchRequest;
use crate::model::ApiResponse;
use crate::service::CategoryService;
use crate::{middleware::AppClaims, service::BlogService};
use actix_jwt_session::Authenticated;
use actix_web::{routes, web, Responder};
use rbs::value;

/**
 * 获取分类列表
 */
#[routes]
#[get("/categories")]
pub async fn categories(
    _: Authenticated<AppClaims>,
    params: web::Query<SearchRequest>,
    app: web::Data<AppState>,
) -> impl Responder {
    if params.get_page_num() <= 0 || params.get_page_size() <= 0 {
        return ApiResponse::<String>::error_with_code(
            WebErrorCode::VALIDATION_ERROR,
            "参数有误!".to_string(),
        )
        .json();
    }
    match CategoryService::get_page_categories(
        params.get_page_num() as u64,
        params.get_page_size() as u64,
        app.get_mysql_pool(),
    )
    .await
    {
        Ok(data) => ApiResponse::success(Some(value!(data))).json(),
        Err(e) => {
            ApiResponse::<String>::error_with_code(WebErrorCode::DATABASE_ERROR, e.to_string()).json()
        }
    }
}

/**
 * 修改分类
 */
#[routes]
#[put("/category")]
pub async fn update_category(
    _: Authenticated<AppClaims>,
    form: web::Json<Category>,
    app: web::Data<AppState>,
) -> impl Responder {
    //参数校验
    if form.get_name().is_empty() {
        return ApiResponse::<String>::error_with_code(
            WebErrorCode::VALIDATION_ERROR,
            "参数有误!".to_string(),
        )
        .json();
    }
    match form.get_id() == 0 {
        //新增分类
        true => {
            let _ =
                CategoryService::insert_category(form.get_name().to_string(), app.get_mysql_pool())
                    .await;
            return ApiResponse::<String>::success_with_msg("新增分类成功!".to_string(), None)
                .json();
        }
        //修改分类
        false => {
            let _ = CategoryService::update_category(form.0, app.get_mysql_pool()).await;
            return ApiResponse::<String>::success_with_msg("修改分类成功!".to_string(), None)
                .json();
        }
    }
}

/**
 * 删除分类
 */
#[routes]
#[delete("/category")]
pub async fn delete_category(
    _: Authenticated<AppClaims>,
    query: web::Query<HashMap<String, i64>>,
    app: web::Data<AppState>,
) -> impl Responder {
    let id = match query.get("id") {
        Some(id) => {
            if *id == 0 {
                return ApiResponse::<String>::error_with_code(
                    WebErrorCode::VALIDATION_ERROR,
                    "参数有误!".to_string(),
                )
                .json();
            }
            *id
        }
        None => {
            return ApiResponse::<String>::error_with_code(
                WebErrorCode::VALIDATION_ERROR,
                "参数有误!".to_string(),
            )
            .json()
        }
    };
    // 查询分类下是否有文章
    let connection = app.get_mysql_pool();
    match BlogService::check_category_exist_blog(id, connection).await {
        Ok(true) => {
            return ApiResponse::<String>::error_with_code(
                WebErrorCode::BUSINESS_ERROR,
                "分类下存在文章,不能删除!".to_string(),
            )
            .json()
        }
        Ok(false) => {
            // 删除分类
            match CategoryService::delete_category(id, connection).await {
                Ok(_) => ApiResponse::<String>::success_with_msg("删除分类成功!".to_string(), None)
                    .json(),
                Err(e) => {
                    ApiResponse::<String>::error_with_code(WebErrorCode::DATABASE_ERROR, e.to_string())
                        .json()
                }
            }
        }
        Err(e) => {
            ApiResponse::<String>::error_with_code(WebErrorCode::DATABASE_ERROR, e.to_string()).json()
        }
    }
}
