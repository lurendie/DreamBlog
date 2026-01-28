use crate::app::AppState;
use crate::error::AppError;
use crate::middleware::AppClaims;
use crate::model::{ApiResponse, SiteSetting};
use crate::service::SiteSettingService;
use actix_jwt_session::Authenticated;
use actix_web::{routes, web};
use rbs::{value, Value};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct SiteSettingUpdateRequest {
    settings: Vec<SiteSetting>,
    #[serde(rename = "deleteIds", default)]
    delete_ids: Vec<i64>,
}

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
    app: web::Data<AppState>,
    request: web::Json<SiteSettingUpdateRequest>,
) -> Result<ApiResponse<Value>, AppError> {
    let request = request.into_inner();
    SiteSettingService::update_site_settings(
        app.get_mysql_pool(),
        request.settings,
        request.delete_ids,
    )
    .await?;

    Ok(ApiResponse::<Value>::success_with_msg(
        "站点设置更新成功",
        None,
    ))
}

#[routes]
#[get("/webTitleSuffix")]
pub async fn get_web_title_suffix(
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
) -> Result<ApiResponse<Value>, AppError> {
    let suffix = SiteSettingService::get_web_title_suffix(app.get_mysql_pool()).await?;
    Ok(ApiResponse::success_with_msg(
        "获取网站标题后缀成功",
        Some(value!(suffix)),
    ))
}
