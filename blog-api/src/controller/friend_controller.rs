use crate::model::ApiResponse;
use crate::service::FriendService;
use crate::{app::AppState, error::AppError};
use actix_web::{get, web};
use rbs::{value, Value};

//获取友链信息
#[get("/friends")]
pub(crate) async fn get_friend(app: web::Data<AppState>) -> Result<ApiResponse<Value>, AppError> {
    let friend = FriendService::get_friend(app.get_mysql_pool()).await?;
    Ok(ApiResponse::<Value>::success(Some(value!(friend))))
}
