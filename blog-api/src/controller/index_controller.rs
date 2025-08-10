use crate::app_state::AppState;
/*
 * @Author: lurendie
 * @Date: 2024-03-26 00:08:12
 * @LastEditors: lurendie
 * @LastEditTime: 2024-05-02 22:02:04
 * @FilePath: \zero_blog\src\controller\index_controller.rs
 */
use crate::model::ResponseResult;

use crate::service::{BlogService, CategoryService, SiteSettingService, TagService};
use actix_web::{routes, web, HttpResponse, Responder};
use rbs::to_value;
use rbs::Value;
use std::collections::HashMap;

/**
   Site 数据
*/
#[routes]
#[get("/site")]
#[options("/site")]
pub async fn site(app: web::Data<AppState>) -> impl Responder {
    let connect = app.get_mysql_pool();
    let mut map: HashMap<String, Value> = match SiteSettingService::find_site_info(connect).await {
        Ok(data) => data,
        Err(e) => return ResponseResult::error(e.to_string()).json(),
    };
    let category_list =match  CategoryService::get_list(connect).await{
        Ok(data) => data,
        Err(e) => return ResponseResult::error(e.to_string()).json(),
    };
    let random_list = match BlogService::find_list_random(connect).await{
        Ok(data) => data,
        Err(e) => return ResponseResult::error(e.to_string()).json(),
    };
    let new_list = match BlogService::find_list_new(connect).await{
        Ok(data) => data,
        Err(e) => return ResponseResult::error(e.to_string()).json(),
    };
    let tag_list = match TagService::get_tags(connect).await{
        Ok(data) => data,
        Err(e) => return ResponseResult::error(e.to_string()).json(),
    };
    map.insert("newBlogList".to_string(), to_value!(new_list));
    map.insert("categoryList".to_string(), to_value!(category_list));
    map.insert("tagList".to_string(), to_value!(tag_list));
    map.insert("randomBlogList".to_string(), to_value!(random_list));
    let result: ResponseResult<HashMap<String, Value>> =
        ResponseResult::new(200, String::from("请求成功！"), Some(map));
    HttpResponse::Ok().json(result)
}

pub async fn default() -> impl Responder {
    //error!("404,找不到页面");
    HttpResponse::Ok().json(to_value!(ResponseResult::<String>::error(String::from(
        "Error Not Found"
    ))))
}
