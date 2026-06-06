use actix_jwt_session::{JwtTtl, OffsetDateTime, RefreshTtl, SessionStorage, Uuid};
use actix_web::web::{Data, Json};
use chrono::Utc;
use rbs::value;
use rbs::value::map::ValueMap;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::common::UserBcrypt;
use crate::constant::RedisKeyConstant;
use crate::entity::user;
use crate::error::{AppError, DataBaseError};
use crate::middleware::AppClaims;
use crate::model::{CacheUserInfo, LoginUser, LoginedCacheUser, User};
use crate::service::RedisService;

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

    pub async fn get_cache_user_info(username: &str) -> Result<LoginedCacheUser, DataBaseError> {
        let redis_key = RedisKeyConstant::LOGIN_USER_INFO.to_string();
        let logined_user =
            RedisService::get_string::<LoginedCacheUser>(format!("{}_{}", redis_key, username))
                .await?;
        Ok(logined_user)
    }

    pub async fn verify_logined_user(
        user_form: &Json<LoginUser>,
        store: &Data<SessionStorage>,
    ) -> Result<(ValueMap, String), DataBaseError> {
        //获取用户信息
        let login_user_info_result = Self::get_cache_user_info(&user_form.username).await;
        if let Err(e) = login_user_info_result {
            return Err(e);
        }
        match login_user_info_result {
            Ok(login_user) => {
                let uuid = Uuid::parse_str(&login_user.uuid).unwrap_or_default();
                let flag = Self::verify_logined_info(user_form, &login_user).await;
                if !flag
                    && uuid
                        .hyphenated()
                        .to_string()
                        .contains("00000000-0000-0000-0000-000000000000")
                {
                    return Err(DataBaseError::Custom(format!(
                        "用户:{}密码验证失败或者UUID是NULL",
                        user_form.username
                    )));
                }
                let app_claims = store.find_jwt::<AppClaims>(uuid).await.unwrap_or_default();
                if app_claims.subject.contains(&user_form.username) {
                    //将用户信息存入map封装
                    let mut map: ValueMap = ValueMap::new();
                    map.insert(value!("user"), value!(&login_user.cache_info.user));
                    map.insert(value!("token"), value!(&login_user.cache_info.token));
                    //密码正确并且权限正确，登录成功返回token,检测用户是否已经登录
                    return Ok((map, login_user.cache_info.token));
                }

                Err(DataBaseError::Custom(format!(
                    "登录用户{},Redis缓存中的TOKEN验证失败",
                    user_form.username
                )))
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    pub async fn verify_logined_info(user_form: &Json<LoginUser>, user: &LoginedCacheUser) -> bool {
        let password_flag =
            UserBcrypt::verify_password(&user.password, &user_form.password).unwrap_or_default();
        if !password_flag
            || !user.cache_info.user.get_role().contains("ROLE_admin")
            || user.uuid.to_string().is_empty()
            || user.cache_info.token.is_empty()
        {
            //密码错误或者非Admin账号登录
            return false;
        }
        true
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
        //创建认证数据
        let claims = AppClaims {
            issues_at: OffsetDateTime::now_utc().unix_timestamp() as usize,
            subject: user.get_username(),
            expiration_time: jwt_ttl.0.as_seconds_f64() as u64,
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
        let user_password = user.get_password();
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

        //将用户信息存入redis
        let cache_user = LoginedCacheUser::new(
            CacheUserInfo::new(user.clone(), &jwt_token),
            &user_password,
            &uuid.hyphenated().to_string(),
        );
        let _ = RedisService::set_string(
            format!(
                "{}_{}",
                RedisKeyConstant::LOGIN_USER_INFO,
                user_form.username
            ),
            &cache_user,
        )
        .await;
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
        RedisService::_del_key(&format!(
            "{}_{}",
            RedisKeyConstant::LOGIN_USER_INFO,
            username
        ))
        .await?;
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
}
