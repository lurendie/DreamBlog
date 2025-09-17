use crate::service::DashboardService;
use crate::{middleware::AppClaims, models::Result};
use actix_jwt_session::Authenticated;
use actix_web::{routes, Responder};
use rbs::{to_value, value::map::ValueMap};

#[routes]
#[get("/dashboard")]
pub async fn dashboard(_: Authenticated<AppClaims>) -> impl Responder {
    let mut map = ValueMap::new();
    let today_pv = 0;
    let today_uv = 0;
    let blog_count = DashboardService::get_blog_count().await;
    let comment_count = DashboardService::get_comment_count().await;
    let category_blog_count_map = DashboardService::get_categorys_count().await;
    let tag_blog_count_map = DashboardService::get_tags_count().await;
    let visit_record_map = ValueMap::new();
    let city_visitor_list = ValueMap::new();
    map.insert(to_value!("pv"), to_value!(today_pv));
    map.insert(to_value!("uv"), to_value!(today_uv));
    map.insert(to_value!("blogCount"), to_value!(blog_count));
    map.insert(to_value!("commentCount"), to_value!(comment_count));
    map.insert(to_value!("category"), to_value!(category_blog_count_map));
    map.insert(to_value!("tag"), to_value!(tag_blog_count_map));
    map.insert(to_value!("visitRecord"), to_value!(visit_record_map));
    map.insert(to_value!("cityVisitor"), to_value!(city_visitor_list));
    Result::ok("请求成功!".to_string(), Some(to_value!(map))).ok_json()
}
