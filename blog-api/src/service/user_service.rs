use actix_jwt_session::{JwtTtl, OffsetDateTime, RefreshTtl, SessionStorage, Uuid};
use actix_web::web::{Data, Json};
use chrono::Utc;
use rbs::value;
use rbs::value::map::ValueMap;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::common::UserBcrypt;
use crate::entity::user;
use crate::error::{AppError, DataBaseError};
use crate::middleware::AppClaims;
use crate::model::{LoginUser, User};

pub struct UserService;

impl UserService {
    /**
     *根据Name获取User
     */
    pub async fn get_by_username(
        username: &str,
        db: &DatabaseConnection,
    ) -> Result<User, DataBaseError> {
        let user = user::Entity::find()
            .filter(user::Column::Username.eq(username))
            .one(db)
            .await?;
        if let Some(user) = user {
            return Ok(User::from(user));
        }
        Err(DataBaseError::Custom("没有检索到该用户".to_string()))
    }

    pub async fn get_user_info(
        user_form: &Json<LoginUser>,
        db: &DatabaseConnection,
        jwt_ttl: Data<JwtTtl>,
        refresh_ttl: Data<RefreshTtl>,
        store: Data<SessionStorage>,
    ) -> Result<(ValueMap, String), DataBaseError> {
        let mut user = UserService::get_by_username(&user_form.username, db).await?;
        //验证账号密码是否正确,排除非Admin账号登录
        let password_flag = UserBcrypt::verify_password(&user.get_password(), &user_form.password)
            .unwrap_or_default();
        if !password_flag || !user.get_role().eq("ROLE_admin") {
            //密码错误或者非Admin账号登录
            return Err(DataBaseError::Custom(format!(
                "用户{}登录失败，密码错误或者非Admin账号登录",
                user_form.username
            )));
        }
        let mut map: ValueMap = ValueMap::new();
        //登录成功
        log::info!("用户:{}登录成功", user_form.username);
        let uuid = Uuid::new_v4();
        let now = OffsetDateTime::now_utc();
        //创建认证数据
        let claims = AppClaims {
            issues_at: now.unix_timestamp() as usize,
            subject: user.get_username(),
            expiration_time: (now + jwt_ttl.0).unix_timestamp() as u64,
            //audience: Audience::Web,
            jwt_id: uuid.clone(),
            account_id: user.get_id() as i32,
            not_before: 0,
        };
        let pair = match store
            .store(claims, *jwt_ttl.into_inner(), *refresh_ttl.into_inner())
            .await
        {
            Ok(pair) => pair,
            Err(e) => {
                return Err(DataBaseError::Custom(format!(
                    "用户{}登录失败，创建认证数据失败:{}",
                    user_form.username, e
                )));
            }
        };
        //清空密码
        user.set_password("".to_string());
        //获取jwt 如果失败则返回
        let jwt_token = pair.jwt.encode().unwrap_or_default();
        if jwt_token.is_empty() {
            return Err(DataBaseError::Custom(format!(
                "用户{}登录失败，获取jwt为空",
                user_form.username
            )));
        }
        //将用户信息存入map封装
        map.insert(value!("user"), value!(&user));
        map.insert(value!("token"), value!(&jwt_token));

        map.insert(value!("user"), value!(&user));
        map.insert(value!("token"), value!(&jwt_token));
        Ok((map, jwt_token))
    }

    pub async fn update(
        user_from: User,
        username: &str,
        db: &DatabaseConnection,
    ) -> Result<(), AppError> {
        let user = UserService::get_by_username(&username, db).await?;
        let user_model = user::Model::from(user);
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
            let password = UserBcrypt::hash_password(&user_from.get_password())?;
            active_user.password = Set(password);
        }
        active_user.update_time = Set(now);
        if let Err(e) = active_user.update(db).await {
            return Err(DataBaseError::Custom(format!("更新用户信息失败:{}", e)).into());
        }
        Ok(())
    }

    pub async fn find_admin_role(db: &DatabaseConnection) -> Result<User, DataBaseError> {
        let user = user::Entity::find()
            .filter(user::Column::Role.eq("ROLE_admin"))
            .one(db)
            .await?;
        if let Some(user) = user {
            return Ok(User::from(user));
        }
        Err(DataBaseError::Custom("没有检索到该用户".to_string()))
    }

    pub async fn is_admin_username(
        username: &str,
        db: &DatabaseConnection,
    ) -> Result<bool, DataBaseError> {
        let user = user::Entity::find()
            .filter(user::Column::Username.eq(username))
            .filter(user::Column::Role.eq("ROLE_admin"))
            .one(db)
            .await?;
        Ok(user.is_some())
    }
}
