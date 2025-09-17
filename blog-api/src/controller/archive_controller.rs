/*
 * @Author: lurendie 549700459@qq.com
 * @Date: 2024-03-26 00:08:12
 * @LastEditors: lurendie
 * @LastEditTime: 2024-05-18 09:58:55
 */
use crate::app_state::AppState;
use crate::error::ErrorCode;
use crate::model::ApiResponse;
use crate::service::BlogService;
use actix_web::{get, web, Responder};
use rbs::to_value;
use rbs::value::map::ValueMap;

#[get("/archives")]
pub(crate) async fn archives(app: web::Data<AppState>) -> impl Responder {
    let mut data = ValueMap::new();
    let connection = app.get_mysql_pool();
    let result = BlogService::find_archives(connection).await;
    match result {
        Ok(blog_map) => {
            let count = BlogService::find_archives_count(connection).await;
            data.insert(to_value!("blogMap"), to_value!(blog_map));
            data.insert(to_value!("count"), to_value!(count.unwrap_or_default()));
            ApiResponse::success(Some(to_value!(data))).json()
        }
        Err(e) => {
            ApiResponse::<String>::error_with_code(ErrorCode::DATABASE_ERROR, e.to_string()).json()
        }
    }
}
