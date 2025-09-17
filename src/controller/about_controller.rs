use crate::models::vo::result::Result;
use crate::service::AboutService;
use actix_web::{get, Responder};
use rbs::to_value;

//关于我
#[get("/about")]
pub(crate) async fn about() -> impl Responder {
    let about = AboutService::get_about().await;
    Result::ok("请求成功".to_string(), Some(to_value!(about))).ok_json()
}
