/*
 * @Author: lurendie
 * @Date: 2024-05-03 23:58:25
 * @LastEditors: lurendie
 * @LastEditTime: 2024-05-17 18:23:36
 *
 */

use crate::app::AppState;
use crate::error::AppError;
use crate::model::{ApiResponse, LoginUser};
use crate::service::UserService;
use actix_jwt_session::{JwtTtl, RefreshTtl, SessionStorage, JWT_HEADER_NAME};
use actix_web::{
    routes,
    web::{Data, Json},
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
) -> Result<ApiResponse<Value>, AppError> {
    match UserService::verify_logined_user(&user_form, &store).await {
        Ok(data) => {
            let result = ApiResponse::<Value>::success_with_msg(
                format!("登录成功!,欢迎用户{}回来", user_form.username).as_str(),
                Some(value!(&data.0)),
            );
            result
                .http_response_builder()
                .append_header((JWT_HEADER_NAME, data.1.to_string()));
            return Ok(result);
        }
        Err(e) => {
            log::warn!("用户名{}尝试登录，错误信息{e}", user_form.username);
            // return Ok(ApiResponse::<String>::error("用户名或密码错误！".to_string()).json());
        }
    }
    //验证账号 密码是否正确
    let data = UserService::get_user_info(
        &user_form,
        app.get_mysql_pool(),
        jwt_ttl,
        refresh_ttl,
        store,
    )
    .await?;
    let result = ApiResponse::<Value>::success_with_msg(
        format!("登录成功!,欢迎用户{}回来", user_form.username).as_str(),
        Some(value!(&data.0)),
    );
    result
        .http_response_builder()
        .append_header((JWT_HEADER_NAME, data.1.to_string()));
    return Ok(result);
}
