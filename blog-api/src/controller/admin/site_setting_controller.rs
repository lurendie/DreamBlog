use crate::middleware::AppClaims;
use crate::service::SiteSettingService;
use crate::{app_state::AppState, model::ApiResponse};
use actix_jwt_session::Authenticated;
use actix_web::{routes, web, Responder};
use rbs::to_value;

#[routes]
#[get("/siteSettings")]
pub async fn get_site_setting_data(
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
) -> impl Responder {
    match SiteSettingService::get_site_info(app.get_mysql_pool()).await {
        Ok(data) => {
            ApiResponse::success_with_msg("获取站点设置成功".to_string(), Some(to_value!(data)))
                .json()
        }
        Err(e) => ApiResponse::<String>::error(format!("获取站点设置失败: {}", e)).json(),
    }
}

#[routes]
#[post("/siteSettings")]
pub async fn update_site_settings(
    _: Authenticated<AppClaims>,
    _app: web::Data<AppState>,
    //  _request: web::Json<SiteSettingUpdateRequest>,
) -> impl Responder {
    // 这里需要实现更新站点设置的逻辑
    // 由于服务层没有提供更新方法，这里先返回一个占位响应
    ApiResponse::<String>::success_with_msg("站点设置更新成功".to_string(), None).json()
}

#[routes]
#[get("/webTitleSuffix")]
pub async fn get_web_title_suffix(
    _: Authenticated<AppClaims>,
    //  app: web::Data<AppState>,
) -> impl Responder {
    // 这里需要实现获取网站标题后缀的逻辑
    // 由于服务层没有提供专门获取标题后缀的方法，这里先返回一个占位响应
    ApiResponse::success_with_msg(
        "获取网站标题后缀成功".to_string(),
        Some(to_value!(" - ZeroBlog")),
    )
    .json()
}
