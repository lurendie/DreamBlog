use std::collections::HashMap;
use crate::app_state::AppState;
use crate::middleware::AppClaims;
use crate::models::dto::moment_dto::MomentDTO;
use crate::models::{Result,vo::page_request::SearchRequest};
use crate::service::MomentService;
use actix_jwt_session::Authenticated;
use actix_web::{routes, web, Responder};
use rbs::to_value;

/**
 * 创建动态
 */
#[routes]
#[post("/moment")]
pub async fn create_moment(moment: web::Json<MomentDTO>,_:Authenticated<AppClaims>,app:web::Data<AppState>) -> impl Responder {
    let moment= MomentService::create_and_update_moment(moment.into_inner(),app.get_mysql_pool()).await;
    match moment {
        Ok(_) => Result::ok_no_data("更新成功".to_string()).ok_json(),
        Err(e) => Result::error(e.to_string()).ok_json(),
    }
}

#[routes]
#[get("/moments")]
pub async fn moments(_:Authenticated<AppClaims>,mut query:web::Query<SearchRequest>,app:web::Data<AppState>)-> impl Responder{
    //查询所有moments
    if query.0.get_page_num() ==0{
       return Result::error("参数有误！".to_string()).ok_json();
    }
    query.0.set_page_size(Some(5));
    //分页查询
  match  MomentService::get_moments(query.0.get_page_num(),query.0.get_page_size(),app.get_mysql_pool()).await{
    Ok(value_map)=> Result::ok("请求成功".to_string(), Some(to_value!(value_map))).ok_json(), // 返回一个包含map的JSON响应;
    Err(e)=> Result::error(e.to_string()).ok_json(),
  }
   
   
    
}

/**
 * 动态发布状态
 */

#[routes]
#[put("/moment/published")]
pub async fn moment_published(query:web::Query<HashMap<String, String>>, _:Authenticated<AppClaims>,app:web::Data<AppState>) -> impl Responder {
    let id = query.get("id").unwrap().parse::<i64>().unwrap_or(0);
    
    if id<=0{
        return Result::error("参数有误！".to_string()).ok_json();
    }
    let is_published = query.get("published").unwrap().parse::<bool>().unwrap();
   let row= MomentService::update_published(id, is_published,app.get_mysql_pool()).await;
    if let Err(e)  = row {
        return Result::error(e.to_string()).ok_json();
    }
    Result::ok_no_data("更新成功".to_string()).ok_json()
    
}

#[routes]
#[get("/moment")]
pub async fn get_moment_by_id(query:web::Query<HashMap<String, String>>, _:Authenticated<AppClaims>,app:web::Data<AppState>) -> impl Responder{
    let id = query.get("id").unwrap().parse::<i64>().unwrap_or(0);
    if id<=0{
        return Result::error("参数有误！".to_string()).ok_json();
    }
    let moment = MomentService::get_moment_by_id(id,app.get_mysql_pool()).await;
    match moment {
        Some(m) => Result::ok("请求成功".to_string(), Some(to_value!(m))),
        None => Result::error("未找到该动态！".to_string()),
    }.ok_json()
}

/**
 * 删除动态
 */

#[routes]
#[delete("/moment")]
pub async fn delete_moment(query:web::Query<HashMap<String, String>>, _:Authenticated<AppClaims>,app:web::Data<AppState>) -> impl Responder {
    let id = query.get("id").unwrap().parse::<i64>().unwrap_or(0);
    if id<=0{
        return Result::error("参数有误！".to_string()).ok_json();
    }
    let row= MomentService::delete_moment(id,app.get_mysql_pool()).await;
    if let Err(e)  = row {
        return Result::error(e.to_string()).ok_json();
    }
    Result::ok_no_data("删除成功".to_string()).ok_json()    
}

/**
 * 更新动态
 */
#[routes]
#[put("/moment")]
pub async fn update_moment(moment: web::Json<MomentDTO>,_:Authenticated<AppClaims>,app:web::Data<AppState>) -> impl Responder {
    let moment = MomentService::create_and_update_moment(moment.into_inner(),app.get_mysql_pool()).await;
    match moment {
        Ok(_) => Result::ok_no_data("更新成功".to_string()).ok_json(),
        Err(e) => Result::error(e.to_string()).ok_json(),
    }
}