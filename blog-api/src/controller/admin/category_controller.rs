use std::collections::HashMap;

use crate::app::AppState;
use crate::common::ParamUtils;
use crate::error::AppError;
use crate::error::WebErrorCode;
use crate::model::ApiResponse;
use crate::model::Category;
use crate::model::SearchRequest;
use crate::service::CategoryService;
use crate::{middleware::AppClaims, service::BlogService};
use actix_jwt_session::Authenticated;
use actix_web::{routes, web};
use rbs::value;
use rbs::Value;

/**
 * 获取分类列表
 */
#[routes]
#[get("/categories")]
pub async fn categories(
    _: Authenticated<AppClaims>,
    params: web::Query<SearchRequest>,
    app: web::Data<AppState>,
) -> Result<ApiResponse<Value>, AppError> {
    let params = ParamUtils::validate_request_params(&params.0).await?;
    let data = CategoryService::get_page_categories(
        params.page_num,
        params.page_size,
        app.get_mysql_pool(),
    )
    .await?;
    Ok(ApiResponse::success(Some(value!(data))))
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
) -> Result<ApiResponse<Value>, AppError> {
    match matches!(form.get_id(), 0) {
        //新增分类
        true => {
            CategoryService::insert_category(form.get_name().to_string(), app.get_mysql_pool())
                .await?;
        }
        //修改分类
        false => {
            CategoryService::update_category(form.0, app.get_mysql_pool()).await?;
        }
    }
    Ok(ApiResponse::<Value>::success_with_msg(
        "新增分类成功!",
        None,
    ))
}

/**
 * 删除分类
 */
#[routes]
#[delete("/category")]
pub async fn delete_category(
    _: Authenticated<AppClaims>,
    query: web::Query<HashMap<String, String>>,
    app: web::Data<AppState>,
) -> Result<ApiResponse<Value>, AppError> {
    let id = ParamUtils::get_i64_param(&query.0, "id")?;
    // 查询分类下是否有文章
    let connection = app.get_mysql_pool();
    match BlogService::check_category_exist_blog(id, connection).await? {
        true => {
            return Ok(ApiResponse::<Value>::error_with_code(
                WebErrorCode::BUSINESS_ERROR,
                "分类下存在文章,不能删除!",
            ));
        }
        false => {
            // 删除分类
            CategoryService::delete_category(id, connection).await?;
            return Ok(ApiResponse::<Value>::success_with_msg(
                "删除分类成功!",
                None,
            ));
        }
    }
}
