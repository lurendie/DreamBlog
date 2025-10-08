use crate::error::AppError;
use crate::middleware::AppClaims;
use crate::model::User;
use crate::service::UserService;
use crate::{app::AppState, model::ApiResponse};
use actix_jwt_session::Authenticated;
use actix_web::{routes, web};
use rbs::{value, Value};

#[routes]
#[post("/account")]
pub async fn change_account(
    auth: Authenticated<AppClaims>,
    app: web::Data<AppState>,
    user_from: web::Json<User>,
) -> Result<ApiResponse<Value>, AppError> {
    let db = app.get_mysql_pool();
    // 通过TOKEN 查询当前用户
    let username = auth.subject.clone();
    UserService::update(user_from.into_inner(), &username, db).await?;
    Ok(ApiResponse::<Value>::success(Some(value!(
        "用户信息,更新成功!"
    ))))
}
