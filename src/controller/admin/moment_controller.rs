//创建动态
use crate::models::dto::moment_dto::MomentDTO;
use crate::models::Result;
use crate::service::MomentService;
use actix_web::{routes, web, Responder};

/**
 * 创建动态
 */
#[routes]
#[post("/moment")]
pub async fn create_moment(moment: web::Json<MomentDTO>) -> impl Responder {
    let moment = MomentService::create_moment(moment.into_inner()).await;
    match moment {
        Ok(_) => Result::ok_no_data("更新成功".to_string()).ok_json(),
        Err(e) => Result::error(e.to_string()).error_json(),
    }
}
