use actix_jwt_session::Authenticated;
use actix_web::{
    routes,
    web::{self, Data},
    Responder,
};
use rbs::to_value;

use crate::{
    app_state::AppState,
    middleware::AppClaims,
    model::{CommentDTO, ResponseResult, SearchRequest},
    service::{BlogService, CommentService},
};

#[routes]
#[get("/comments")]
pub async fn find_comments(
    _: Authenticated<AppClaims>,
    app: Data<AppState>,
    query: web::Query<SearchRequest>,
) -> impl Responder {
    let page_num = query.get_page_num();
    let page_size = query.get_page_size();
    match CommentService::find_comment_dto(page_num, page_size, app.get_mysql_pool()).await {
        Ok(comments) => {
            ResponseResult::ok("请求成功！".to_string(), Some(to_value!(comments))).json()
        }
        Err(e) => ResponseResult::error(e.to_string()).json(),
    }
}

#[routes]
#[get("/blogIdAndTitle")]
pub async fn find_blog_id_and_title(
    _: Authenticated<AppClaims>,
    app: Data<AppState>,
) -> impl Responder {
    match BlogService::find_blogs_and_title(app.get_mysql_pool()).await {
        Ok(comments) => {
            ResponseResult::ok("请求成功！".to_string(), Some(to_value!(comments))).json()
        }
        Err(e) => ResponseResult::error(e.to_string()).json(),
    }
}

//更新评论
#[routes]
#[put("/comment")]
pub async fn update_comment(
    _: Authenticated<AppClaims>,
    app: Data<AppState>,
    comment: web::Json<CommentDTO>,
) -> impl Responder {
    match CommentService::add_and_update_comment(comment.into_inner(), app.get_mysql_pool()).await {
        Ok(_) => ResponseResult::ok_no_data("更新成功！".to_string()).json(),
        Err(e) => ResponseResult::error(e.to_string()).json(),
    }
}

//删除评论
#[routes]
#[delete("/comment/{id}")]
pub async fn delete_comment(
    _: Authenticated<AppClaims>,
    app: Data<AppState>,
    id: web::Path<i64>,
) -> impl Responder {
    match CommentService::delete_comment(id.into_inner(), app.get_mysql_pool()).await {
        Ok(_) => ResponseResult::ok("删除成功！".to_string(), None).json(),
        Err(e) => ResponseResult::error(e.to_string()).json(),
    }
}
