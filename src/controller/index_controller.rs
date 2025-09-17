/*
 * @Author: lurendie 
 * @Date: 2024-03-26 00:08:12
 * @LastEditors: lurendie
 * @LastEditTime: 2024-05-02 22:02:04
 * @FilePath: \zero_blog\src\controller\index_controller.rs
 */
use crate::models::vo::result::Result;

use crate::service::{BlogService, CategoryService, SiteSettingService, TagService};
use actix_web::{routes, HttpResponse, Responder};
use rbs::to_value;
use rbs::Value;
use std::collections::HashMap;

/**
   Site 数据
*/
#[routes]
#[get("/site")]
#[options("/site")]
pub async fn site() -> impl Responder {
    let mut map: HashMap<String, Value> = SiteSettingService::get_site_info().await;
    let category_list = CategoryService::get_list().await;
    let random_list = BlogService::get_blog_list_random().await;
    let new_list = BlogService::get_blog_list_new().await;
    let tag_list = TagService::get_tags().await;
    map.insert("newBlogList".to_string(), to_value!(new_list));
    map.insert("categoryList".to_string(), to_value!(category_list));
    map.insert("tagList".to_string(), to_value!(tag_list));
    map.insert("randomBlogList".to_string(), to_value!(random_list));
    let result: Result<HashMap<String, Value>> =
        Result::new(200, String::from("请求成功！"), Some(map));
    HttpResponse::Ok()
        .json(result)
}

pub async fn default() -> impl Responder {
    //error!("404,找不到页面");
    HttpResponse::Found()
        .content_type(mime::TEXT_HTML_UTF_8)
        .body("404,找不到页面")
}
