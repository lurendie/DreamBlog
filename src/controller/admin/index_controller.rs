use actix_web::{ HttpResponse, Responder};
use log::error;

pub async fn default() -> impl Responder {
    error!("404,找不到页面");
    HttpResponse::Found().content_type(mime::TEXT_HTML_UTF_8).body("404,找不到页面")
}