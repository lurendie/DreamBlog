use std::collections::HashMap;

use actix_jwt_session::Authenticated;
use actix_web::{
    routes,
    web::{self, Data},
    Responder,
};
use rbs::value;

use crate::{
    app::AppState,
    error::WebErrorCode,
    middleware::AppClaims,
    model::ApiResponse,
    model::{CommentDTO, SearchRequest},
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
            ApiResponse::success_with_msg("请求成功！".to_string(), Some(value!(comments))).json()
        }
        Err(e) => ApiResponse::<String>::error(e.to_string()).json(),
    }
}

#[routes]
#[get("/blogIdAndTitle")]
pub async fn find_blog_id_and_title(
    _: Authenticated<AppClaims>,
    app: Data<AppState>,
) -> impl Responder {
    match BlogService::find_blogs_and_title(app.get_mysql_pool()).await {
        Ok(comments) => ApiResponse::success(Some(value!(comments))).json(),
        Err(e) => {
            ApiResponse::<String>::error_with_code(WebErrorCode::DATABASE_ERROR, e.to_string())
                .json()
        }
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
    match CommentService::save_comment(comment.into_inner(), app.get_mysql_pool()).await {
        Ok(_) => ApiResponse::<String>::success_with_msg("更新成功！".to_string(), None).json(),
        Err(e) => {
            ApiResponse::<String>::error_with_code(WebErrorCode::DATABASE_ERROR, e.to_string())
                .json()
        }
    }
}

//删除评论
#[routes]
#[delete("/comment")]
pub async fn delete_comment(
    _: Authenticated<AppClaims>,
    app: Data<AppState>,
    parameter: web::Query<HashMap<String, i64>>,
) -> impl Responder {
    let id = *parameter.get("id").unwrap_or(&0);
    match CommentService::delete_comment_recursive(id, app.get_mysql_pool()).await {
        Ok(_) => ApiResponse::<String>::success_with_msg("删除成功！".to_string(), None).json(),
        Err(e) => {
            ApiResponse::<String>::error_with_code(WebErrorCode::DATABASE_ERROR, e.to_string())
                .json()
        }
    }
}
