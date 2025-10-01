use crate::common::UserBcrypt;
use crate::entity::user;
use crate::middleware::AppClaims;
use crate::model::User;
use crate::{app::AppState, model::ApiResponse};
use actix_jwt_session::Authenticated;
use actix_web::{routes, web, Responder};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

#[routes]
#[post("/account")]
pub async fn change_account(
    auth: Authenticated<AppClaims>,
    app: web::Data<AppState>,
    user_from: web::Json<User>,
) -> impl Responder {
    let db = app.get_mysql_pool();
    let username = auth.subject.clone();
    // 查询当前用户
    let result = user::Entity::find()
        .filter(user::Column::Username.eq(username))
        .one(db)
        .await;

    match result {
        Ok(Some(user_model)) => {
            let mut active_user: user::ActiveModel = user_model.into();
            let now = Utc::now().naive_utc();

            // 更新字段
            if !user_from.get_username().is_empty() {
                // 检查用户名是否已被其他用户使用
                active_user.username = Set(user_from.get_username());
            }

            if !user_from.get_nickname().is_empty() {
                active_user.nickname = Set(user_from.get_nickname());
            }

            if !user_from.get_avatar().is_empty() {
                active_user.avatar = Set(user_from.get_avatar());
            }

            if !user_from.get_email().is_empty() {
                active_user.email = Set(user_from.get_email());
            }
            if !user_from.get_password().is_empty() {
                let password = match UserBcrypt::hash_password(&user_from.get_password()) {
                    Ok(password) => password,
                    Err(e) => {
                        return ApiResponse::<String>::error(format!("密码加密失败: {}", e)).json()
                    }
                };
                active_user.password = Set(password);
            }

            active_user.update_time = Set(now);

            match active_user.update(db).await {
                Ok(_) => {
                    ApiResponse::<String>::success_with_msg("用户信息更新成功".to_string(), None)
                        .json()
                }
                Err(e) => ApiResponse::<String>::error(format!("用户信息更新失败: {}", e)).json(),
            }
        }
        Ok(None) => ApiResponse::<String>::error("用户不存在".to_string()).json(),
        Err(e) => ApiResponse::<String>::error(format!("查询用户失败: {}", e)).json(),
    }
}
