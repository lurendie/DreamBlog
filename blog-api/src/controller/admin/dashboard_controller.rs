use crate::{app_state::AppState, model::ApiResponse};
use crate::middleware::AppClaims;
use crate::service::DashboardService;
use actix_jwt_session::Authenticated;
use actix_web::{routes, web, Responder};
use rbs::{value, value::map::ValueMap};

#[routes]
#[get("/dashboard")]
pub async fn dashboard(_: Authenticated<AppClaims>, app: web::Data<AppState>) -> impl Responder {
    let mut map = ValueMap::new();
    let today_pv = 0;
    let today_uv = 0;
    let blog_count = DashboardService::get_blog_count(app.get_mysql_pool()).await;
    let comment_count = DashboardService::get_comment_count(app.get_mysql_pool()).await;
    let category_blog_count_map = DashboardService::get_categorys_count(app.get_mysql_pool()).await;
    let tag_blog_count_map = DashboardService::get_tags_count(app.get_mysql_pool()).await;
    let visit_record_map = ValueMap::new();
    let city_visitor_list = ValueMap::new();
    map.insert( value!("pv"), value!(today_pv));
    map.insert( value!("uv"), value!(today_uv));
    map.insert( value!("blogCount"), value!(blog_count));
    map.insert( value!("commentCount"), value!(comment_count));
    map.insert( value!("category"), value!(category_blog_count_map));
    map.insert( value!("tag"), value!(tag_blog_count_map));
    map.insert( value!("visitRecord"), value!(visit_record_map));
    map.insert( value!("cityVisitor"), value!(city_visitor_list));
    ApiResponse::success_with_msg("请求成功!".to_string(), Some(value!(map))).json()
}
