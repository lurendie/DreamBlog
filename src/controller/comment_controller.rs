/*
 * @Author: lurendie
 * @Date: 2024-02-24 22:58:03
 * @LastEditors: lurendie
 * @LastEditTime: 2024-04-21 23:26:00
 */
use crate::models::vo::{page_request::SearchRequest, result::Result};
use crate::service::CommentService;
use actix_web::web::Query;
use actix_web::{get, Responder};
use rbatis::{IPage, IPageRequest};
use rbs::to_value;
use rbs::value::map::ValueMap;

#[get("/comments")]
pub(crate) async fn get_comments(data: Option<Query<SearchRequest>>) -> impl Responder {
    //println!("page:{:?}", data);
    if data.is_none() {
        return Result::error("获取数据失败!".to_string()).error_json();
    }
    let mut comments = ValueMap::new();
    let page_request = data.unwrap();

    let list = CommentService::get_comments_page(
        page_request.get_page_num() as usize,
        page_request.get_page(),
        page_request.get_blog_id(),
    )
    .await;
    comments.insert("list".into(), to_value!(list.get_records()));
    comments.insert("totalPage".into(), rbs::Value::U64(list.pages()));
    let mut data = ValueMap::new();
    data.insert("comments".into(), to_value!(comments));
    data.insert(
        "allComment".into(),
        to_value!(rbs::Value::String(
            CommentService::get_all_comments(page_request.get_blog_id()).await
        )),
    );
    data.insert(
        "closeComment".into(),
        to_value!(CommentService::get_close_comments(page_request.get_blog_id()).await),
    );
    Result::ok("获取成功!".to_string(), Some(to_value!(data))).ok_json()
}
