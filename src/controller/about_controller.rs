use crate::service::AboutService;
use crate::{app_state::AppState, model::ResponseResult};
use actix_web::{get, web, Responder};
use rbs::to_value;

//关于我
#[get("/about")]
pub(crate) async fn about(app: web::Data<AppState>) -> impl Responder {
    match AboutService::get_about(app.get_mysql_pool()).await {
        Ok(value_map) => {
            ResponseResult::ok("请求成功".to_string(), Some(to_value!(value_map))).json()
        }
        Err(e) => ResponseResult::error(e.to_string()).json(),
    }
}
