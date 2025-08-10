/*
 * @Author: lurendie 549700459@qq.com
 * @Date: 2024-03-26 00:08:12
 * @LastEditors: lurendie
 * @LastEditTime: 2024-05-18 09:58:55
 */
use crate::app_state::AppState;
use crate::model::ResponseResult;
use crate::service::BlogService;
use actix_web::{get, web, Responder};
use rbs::to_value;
use rbs::value::map::ValueMap;

#[get("/archives")]
pub(crate) async fn archives(app: web::Data<AppState>) -> impl Responder {
    let mut data = ValueMap::new();
    let connection =app.get_mysql_pool();
    let blog_map = BlogService::find_archives(connection).await.unwrap();
    let count = BlogService::find_archives_count(connection).await;
    data.insert(to_value!("blogMap"), to_value!(blog_map));
    data.insert(to_value!("count"), to_value!(count.unwrap_or_default()));
    ResponseResult::ok("请求成功".to_string(), Some(to_value!(data))).json()
}
