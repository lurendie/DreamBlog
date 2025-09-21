use crate::entity::user;
use crate::middleware::AppClaims;
use crate::service::UserService;
use crate::{app::AppState, model::ApiResponse};
use actix_jwt_session::Authenticated;
use actix_web::{routes, web, Responder};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AccountUpdate {
    pub username: Option<String>,
    pub password: Option<String>,
    pub nickname: Option<String>,
    pub avatar: Option<String>,
    pub email: Option<String>,
    pub old_password: Option<String>,
}

#[routes]
#[post("/account")]
pub async fn change_account(
    auth: Authenticated<AppClaims>,
    app: web::Data<AppState>,
    account_update: web::Json<AccountUpdate>,
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
            if let Some(username) = &account_update.username {
                // 检查用户名是否已被其他用户使用
                if let Ok(existing_user) = UserService::get_by_username(username, db).await {
                    if existing_user.get_username() == *username {
                        return ApiResponse::<String>::error("用户名已被使用".to_string()).json();
                    }
                }
                active_user.username = Set(username.clone());
            }

            if let Some(nickname) = &account_update.nickname {
                active_user.nickname = Set(nickname.clone());
            }

            if let Some(avatar) = &account_update.avatar {
                active_user.avatar = Set(avatar.clone());
            }

            if let Some(email) = &account_update.email {
                active_user.email = Set(email.clone());
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
