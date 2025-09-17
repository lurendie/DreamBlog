use crate::models::vo::result::Result;
use crate::service::FriendService;
use actix_web::get;
use actix_web::{HttpResponse, Responder};

//获取友链信息
#[get("/friends")]
pub(crate) async fn get_friend() -> impl Responder {
    let friend = FriendService::get_friend().await;
    let result = Result::ok("请求成功".to_string(), Some(friend));
    HttpResponse::Ok().json(result)
}
