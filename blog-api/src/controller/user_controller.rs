/*
 * @Author: lurendie
 * @Date: 2024-05-03 23:58:25
 * @LastEditors: lurendie
 * @LastEditTime: 2024-05-17 18:23:36
 *
 */

use crate::app::AppState;
use crate::app::CONFIG;
use crate::common::UserBcrypt;
use crate::model::ApiResponse;
use crate::{middleware::AppClaims, service::UserService};
use actix_jwt_session::JWT_COOKIE_NAME;
use actix_jwt_session::{
    JwtTtl, OffsetDateTime, RefreshTtl, SessionStorage, Uuid, JWT_HEADER_NAME, REFRESH_HEADER_NAME,
};
use actix_web::{
    routes,
    web::{Data, Json},
    HttpResponse, Responder,
};
use rbs::value::map::ValueMap;
use rbs::{value, Value};
use serde::Deserialize;

#[derive(Deserialize)]
struct SignInPayload {
    username: String,
    password: String,
}

#[routes]
#[post("/login")]
pub async fn login(
    user_form: Json<SignInPayload>,
    store: Data<SessionStorage>,
    jwt_ttl: Data<JwtTtl>,
    refresh_ttl: Data<RefreshTtl>,
    app: Data<AppState>,
) -> impl Responder {
    //TODO BUG 前端页面token没有过期时间 登录过一次后会一直保持   actix-jwt-session 发现请求头带有token 会进行验证 token过期无法找到session 拦截
    //验证账号 密码是否正确
    let mut user = UserService::get_by_username(&user_form.username, app.get_mysql_pool()).await;
    if let Ok(user) = user.as_mut() {
        //验证账号密码是否正确,排除非Admin账号登录
        let password_flag = UserBcrypt::verify_password(&user.get_password(), &user_form.password)
            .unwrap_or_default();
        if !password_flag || user.get_role() != "ROLE_admin" {
            //密码错误或者非Admin账号登录
            log::error!(
                "用户{}登录失败，密码错误或者非Admin账号登录",
                user_form.username
            );
            return ApiResponse::<Value>::error("用户名或密码错误！".to_string()).json();
        }
        //密码正确并且权限正确，登录成功返回token
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
            jwt_id: Uuid::parse_str(uuid.to_string().as_str()).unwrap(),
            account_id: user.get_id() as i32,
            not_before: 0,
        };
        let pair = match store
            .store(claims, *jwt_ttl.into_inner(), *refresh_ttl.into_inner())
            .await
        {
            Ok(pair) => pair,
            Err(e) => {
                log::error!("用户{}登录失败，创建认证数据失败:{}", user_form.username, e);
                return ApiResponse::<String>::error("登录失败！".to_string()).json();
            }
        };

        user.set_password("".to_string());
        //获取jwt 如果失败则返回
        let jwt_token = pair.jwt.encode().unwrap_or_default();
        if jwt_token.is_empty() {
            log::warn!("用户{}登录失败，获取jwt为空", user_form.username);
            return ApiResponse::<String>::error("登录失败！".to_string()).json();
        }
        //获取refresh 如果失败则返回
        let refresh_token = pair.refresh.encode().unwrap_or_default();
        if refresh_token.is_empty() {
            log::warn!("用户{}登录失败，获取refresh为空", user_form.username);
            return ApiResponse::<String>::error("登录失败！".to_string()).json();
        }
        map.insert(value!("user"), value!(user));
        map.insert(value!("token"), value!(&jwt_token));
        map.insert(
            value!("expires"),
            value!(CONFIG.get_server_config().token_expires),
        );
        let result =
            ApiResponse::<Value>::success_with_msg("请求成功".to_string(), Some(value!(map)));
        return HttpResponse::Ok()
            .append_header((JWT_HEADER_NAME, jwt_token.to_string()))
            .append_header((REFRESH_HEADER_NAME, refresh_token))
            .cookie(actix_web::cookie::Cookie::build(JWT_COOKIE_NAME, jwt_token).finish())
            .json(result);
    }
    log::warn!("用户名{}尝试登录，未找到用户", user_form.username);
    ApiResponse::<String>::error("用户名或密码错误！".to_string()).json()
}
