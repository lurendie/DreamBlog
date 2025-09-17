/*
 * @Author: lurendie 549700459@qq.com
 * @Date: 2024-03-26 00:08:12
 * @LastEditors: lurendie
 * @LastEditTime: 2024-05-18 09:58:55
 */
use crate::models::vo::result::Result;
use crate::service::BlogService;
use actix_web::{get, Responder};
use rbs::to_value;
use rbs::value::map::ValueMap;

#[get("/archives")]
pub(crate) async fn archives() -> impl Responder {
    let mut data = ValueMap::new();
    let blog_map = BlogService::get_archives().await;
    let count = BlogService::get_archives_count().await;
    data.insert(to_value!("blogMap"), to_value!(blog_map));
    data.insert(to_value!("count"), to_value!(count));
    Result::ok("请求成功".to_string(), Some(to_value!(data))).ok_json()
}
