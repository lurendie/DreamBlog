use crate::service::AboutService;
use crate::{app::AppState, error::ErrorCode, model::ApiResponse};
use actix_web::{get, web, Responder};
use rbs::value;

//关于我
#[get("/about")]
pub(crate) async fn about(app: web::Data<AppState>) -> impl Responder {
    match AboutService::get_about(app.get_mysql_pool()).await {
        Ok(value_map) => {
            ApiResponse::success(Some(value!(value_map))).json()
        }
        Err(e) => {
            ApiResponse::<String>::error_with_code(ErrorCode::DATABASE_ERROR, e.to_string()).json()
        }
    }
}
