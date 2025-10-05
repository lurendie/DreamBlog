/*
 * @Author: lurendie
 * @Date: 2024-05-03 23:58:25
 * @LastEditors: lurendie
 * @LastEditTime: 2024-05-17 18:23:36
 *
 */

use crate::app::AppState;
use crate::model::{ApiResponse, LoginUser};
use crate::service::UserService;
use actix_jwt_session::{JwtTtl, RefreshTtl, SessionStorage, JWT_HEADER_NAME};
use actix_web::{
    routes,
    web::{Data, Json},
    HttpResponse, Responder,
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
) -> Result<impl Responder, actix_web::Error> {
    match UserService::verify_logined_user(&user_form, &store).await {
        Ok(map) => {
            let result = ApiResponse::<Value>::success_with_msg(
                format!("登录成功!,欢迎用户{}回来", user_form.username).as_str(),
                Some(value!(&map.0)),
            );
            return Ok(HttpResponse::Ok()
                .append_header((JWT_HEADER_NAME, map.1.to_string()))
                .content_type("application/json; charset=utf-8")
                .json(result));
        }
        Err(e) => {
            log::warn!("用户名{}尝试登录，错误信息{e}", user_form.username);
            // return Ok(ApiResponse::<String>::error("用户名或密码错误！".to_string()).json());
        }
    }
    //验证账号 密码是否正确
    match UserService::get_user_info(
        &user_form,
        app.get_mysql_pool(),
        jwt_ttl,
        refresh_ttl,
        store,
    )
    .await
    {
        Ok(map) => {
            let result = ApiResponse::<Value>::success_with_msg(
                format!("用户{},登录成功!", user_form.username).as_str(),
                Some(value!(&map.0)),
            );
            return Ok(HttpResponse::Ok()
                .append_header((JWT_HEADER_NAME, map.1.to_string()))
                .content_type("application/json; charset=utf-8")
                .json(result));
        }
        Err(e) => {
            log::warn!("用户名{}尝试登录，错误信息{e}", user_form.username);
            return Ok(ApiResponse::<String>::error("用户名或密码错误！").json());
        }
    }
}
