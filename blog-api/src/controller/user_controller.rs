/*
 * @Author: lurendie
 * @Date: 2024-05-03 23:58:25
 * @LastEditors: lurendie
 * @LastEditTime: 2024-05-17 18:23:36
 *
 */

use crate::app::AppState;
use crate::common::IpRegion;
use crate::error::AppError;
use crate::middleware::AppClaims;
use crate::model::{ApiResponse, LoginUser};
use crate::service::{LoginLogService, RedisService, UserService};
use actix_jwt_session::{Authenticated, JwtTtl, RefreshTtl, SessionStorage};
use actix_web::cookie::{self, Cookie, SameSite};
use actix_web::{
    routes,
    web::{Data, Json},
    HttpRequest, HttpResponse,
};
use rbs::{value, Value};

#[routes]
#[post("/login")]
pub async fn login(
    user_form: Json<LoginUser>,
    store: Data<SessionStorage>,
    jwt_ttl: Data<JwtTtl>,
    refresh_ttl: Data<RefreshTtl>,
    app: Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let ip = IpRegion::get_real_client_ip(&req, crate::app::CONFIG.get_server_config().trust_proxy);
    // 登录失败锁定：同一 IP 连续失败 5 次后锁定 10 分钟（Redis 关闭时跳过）
    let fail_key = format!("login:fail:{}", ip);
    if let Ok(count) = RedisService::get_string::<i64>(fail_key.clone()).await {
        if count >= 5 {
            let _ = LoginLogService::save_login_log(
                app.get_mysql_pool(),
                &user_form.username,
                &req,
                false,
                "登录失败次数过多，账号已临时锁定".to_string(),
            )
            .await;
            return Err(AppError::Custom(
                "登录失败次数过多，请 10 分钟后再试".to_string(),
            ));
        }
    }

    let result = UserService::get_user_info(
        &user_form,
        app.get_mysql_pool(),
        jwt_ttl,
        refresh_ttl,
        store,
    )
    .await;
    let description = match &result {
        Ok(_) => {
            RedisService::try_del_key(&fail_key).await;
            "登录成功".to_string()
        }
        Err(e) => {
            RedisService::incr_with_ttl(&fail_key, 600).await;
            e.to_string()
        }
    };
    // 记录登录日志（写日志失败不影响登录流程）
    let _ = LoginLogService::save_login_log(
        app.get_mysql_pool(),
        &user_form.username,
        &req,
        result.is_ok(),
        description,
    )
    .await;

    let data = result?;
    let mut resp = ApiResponse::<Value>::success_with_msg(
        format!("登录成功!,欢迎用户{}!", user_form.username).as_str(),
        Some(value!(&data.0)),
    )
    .respond();
    // JWT 同时写入 httpOnly Cookie（SameSite=Lax 防 CSRF），前端不再持有 token 明文
    let cookie = Cookie::build("token", &data.1)
        .http_only(true)
        .same_site(SameSite::Lax)
        .path("/")
        .max_age(cookie::time::Duration::days(
            crate::app::CONFIG.get_server_config().token_expires,
        ))
        .secure(crate::app::CONFIG.get_server_config().cookie_secure)
        .finish();
    if let Err(e) = resp.add_cookie(&cookie) {
        return Err(AppError::Custom(format!("设置登录 Cookie 失败: {e}")));
    }
    Ok(resp)
}

#[routes]
#[post("/logout")]
pub async fn logout(
    auth: Authenticated<AppClaims>,
    store: Data<SessionStorage>,
) -> Result<HttpResponse, AppError> {
    // 吊销会话（Redis 存储启用时生效；无状态降级模式下忽略）
    if let Err(e) = store.erase::<AppClaims>(auth.claims.jwt_id).await {
        tracing::debug!("退出登录吊销会话失败（可能为无状态模式）: {e}");
    }
    let mut resp = ApiResponse::<Value>::success_with_msg("退出登录成功", None).respond();
    // 清除 httpOnly 会话 Cookie（与登录时相同的 Path/SameSite 属性）
    let removal = Cookie::build("token", "")
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .finish();
    if let Err(e) = resp.add_removal_cookie(&removal) {
        return Err(AppError::Custom(format!("清除登录 Cookie 失败: {e}")));
    }
    Ok(resp)
}
