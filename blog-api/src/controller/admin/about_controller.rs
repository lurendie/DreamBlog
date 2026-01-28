use crate::app::AppState;
use crate::error::AppError;
use crate::middleware::AppClaims;
use crate::model::{AboutForm, ApiResponse};
use crate::service::AboutService;
use actix_jwt_session::Authenticated;
use actix_web::{get, put, web};
use rbs::{value, Value};

#[get("/about")]
pub async fn get_about(
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
) -> Result<ApiResponse<Value>, AppError> {
    let value_map = AboutService::get_about_raw(app.get_mysql_pool()).await?;
    Ok(ApiResponse::success(Some(value!(value_map))))
}

#[put("/about")]
pub async fn update_about(
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
    form: web::Json<AboutForm>,
) -> Result<ApiResponse<Value>, AppError> {
    AboutService::update_about(form.into_inner(), app.get_mysql_pool()).await?;
    Ok(ApiResponse::<Value>::success_with_msg("更新成功", None))
}
