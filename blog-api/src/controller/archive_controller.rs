/*
 * @Author: lurendie 549700459@qq.com
 * @Date: 2024-03-26 00:08:12
 * @LastEditors: lurendie
 * @LastEditTime: 2024-05-18 09:58:55
 */
use crate::app::AppState;
use crate::error::AppError;
use crate::model::ApiResponse;
use crate::service::BlogService;
use actix_web::{get, web};
use rbs::value::map::ValueMap;
use rbs::{value, Value};

#[get("/archives")]
pub(crate) async fn archives(app: web::Data<AppState>) -> Result<ApiResponse<Value>, AppError> {
    let mut data = ValueMap::new();
    let connection = app.get_mysql_pool();
    let result = BlogService::find_archives(connection).await?;
    let count = BlogService::find_archives_count(connection).await;
    data.insert(value!("blogMap"), value!(result));
    data.insert(value!("count"), value!(count.unwrap_or_default()));
    Ok(ApiResponse::success(Some(value!(data))))
}
