use crate::error::AppError;
use crate::service::AboutService;
use crate::{app::AppState, model::ApiResponse};
use actix_web::{get, web};
use rbs::{value, Value};

//关于我
#[get("/about")]
pub(crate) async fn about(app: web::Data<AppState>) -> Result<ApiResponse<Value>, AppError> {
    let value_map = AboutService::get_about(app.get_mysql_pool()).await?;
    Ok(ApiResponse::success(Some(value!(value_map))))
}
