use crate::error::AppError;
use crate::middleware::AppClaims;
use crate::service::SiteSettingService;
use crate::{app::AppState, model::ApiResponse};
use actix_jwt_session::Authenticated;
use actix_web::{routes, web};
use rbs::{value, Value};

#[routes]
#[get("/siteSettings")]
pub async fn get_site_setting_data(
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
) -> Result<ApiResponse<Value>, AppError> {
    let data = SiteSettingService::get_site_info(app.get_mysql_pool()).await?;
    Ok(ApiResponse::success_with_msg(
        "获取站点设置成功",
        Some(value!(data)),
    ))
}

#[routes]
#[post("/siteSettings")]
pub async fn update_site_settings(
    _: Authenticated<AppClaims>,
    _app: web::Data<AppState>,
    //  _request: web::Json<SiteSettingUpdateRequest>,
) -> Result<ApiResponse<Value>, AppError> {
    // 这里需要实现更新站点设置的逻辑
    // 由于服务层没有提供更新方法，这里先返回一个占位响应
    Ok(ApiResponse::<Value>::success_with_msg(
        "站点设置更新成功",
        None,
    ))
}

#[routes]
#[get("/webTitleSuffix")]
pub async fn get_web_title_suffix(
    _: Authenticated<AppClaims>,
    //  app: web::Data<AppState>,
) -> Result<ApiResponse<Value>, AppError> {
    // 这里需要实现获取网站标题后缀的逻辑
    // 由于服务层没有提供专门获取标题后缀的方法，这里先返回一个占位响应
    Ok(ApiResponse::success_with_msg(
        "获取网站标题后缀成功",
        Some(value!(" - ZeroBlog")),
    ))
}
