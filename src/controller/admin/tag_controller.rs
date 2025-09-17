use std::collections::HashMap;

use crate::{
    app_state::AppState,
    middleware::AppClaims,
    model::{ResponseResult, SearchRequest, TagDTO},
    service::TagService,
};
use actix_jwt_session::Authenticated;
use actix_web::{routes, web, Responder};
use rbs::to_value;

#[routes]
#[get("/tags")]
pub async fn get_all_tags(
    _: Authenticated<AppClaims>,
    params: web::Query<SearchRequest>,
    app: web::Data<AppState>,
) -> impl Responder {
    if params.get_page_num() <= 0 || params.get_page_size() <= 0 {
        return ResponseResult::error("参数有误!".to_string()).json();
    }

    let tags_result = TagService::get_tags_by_page(
        params.get_page_num(),
        params.get_page_size(),
        app.get_mysql_pool(),
    )
    .await;
    match tags_result {
        Ok(value_map) => {
            ResponseResult::ok("请求成功！".to_string(), Some(to_value!(value_map))).json()
        }
        Err(e) => ResponseResult::error(e.to_string()).json(),
    }
}

#[routes]
#[put("/tag")]
#[post("/tag")]
pub async fn insert_or_update(
    _: Authenticated<AppClaims>,
    tag: web::Json<TagDTO>,
    app: web::Data<AppState>,
) -> impl Responder {
    let tag_result = TagService::insert_or_update(tag.into_inner(), app.get_mysql_pool()).await;
    match tag_result {
        Ok(_) => ResponseResult::ok_no_data("操作成功！".to_string()).json(),
        Err(e) => ResponseResult::error(e.to_string()).json(),
    }
}

#[routes]
#[delete("/tag")]
pub async fn delete_by_id(
    _: Authenticated<AppClaims>,
    query: web::Query<HashMap<String, i64>>,
    app: web::Data<AppState>,
) -> impl Responder {
    let id = {
        match query.get("id") {
            Some(id) => id.to_owned(),
            None => return ResponseResult::error("参数有误!".to_string()).json(),
        }
    };
    match TagService::delete_by_id(id, app.get_mysql_pool()).await {
        Ok(_) => ResponseResult::ok_no_data("操作成功！".to_string()).json(),
        Err(e) => ResponseResult::error(e.to_string()).json(),
    }
}
