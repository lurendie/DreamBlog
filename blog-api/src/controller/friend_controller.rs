use crate::app::AppState;
use crate::error::WebErrorCode;
use crate::model::ApiResponse;
use crate::service::FriendService;
use actix_web::{get, web, Responder};

//获取友链信息
#[get("/friends")]
pub(crate) async fn get_friend(app: web::Data<AppState>) -> impl Responder {
    match FriendService::get_friend(app.get_mysql_pool()).await {
        Ok(friend) => {
            ApiResponse::success(Some(friend)).json()
        }
        Err(e) => {
            ApiResponse::<String>::error_with_code(WebErrorCode::DATABASE_ERROR, e.to_string()).json()
        }
    }
}
