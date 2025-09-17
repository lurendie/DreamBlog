use crate::app_state::AppState;
use crate::error::ErrorCode;
use crate::model::SearchRequest;
use crate::model::ApiResponse;
use crate::service::CommentService;
use actix_web::web::{self, Query};
use actix_web::{get, Responder};
use rbs::to_value;
use rbs::value::map::ValueMap;

#[get("/comments")]
pub(crate) async fn get_comments(
    data: Option<Query<SearchRequest>>,
    app: web::Data<AppState>,
) -> impl Responder {
    if data.is_none() {
        return ApiResponse::<String>::error_with_code(ErrorCode::VALIDATION_ERROR, "获取数据失败!".to_string()).json();
    }

    let page_request = match data {
        Some(page_request) => page_request,
        None => return ApiResponse::<String>::error_with_code(ErrorCode::VALIDATION_ERROR, "获取数据失败!".to_string()).json(),
    };
    let connect = app.get_mysql_pool();
    let list = match CommentService::find_by_id_comments(
        page_request.get_page_num(),
        page_request.get_blog_id(),
        connect,
    )
    .await
    {
        Ok(list) => list,
        Err(e) => {
            return ApiResponse::<String>::error_with_code(ErrorCode::DATABASE_ERROR, e.to_string()).json();
        }
    };

    let mut data = ValueMap::new();
    data.insert("comments".into(), to_value!(list));

    match CommentService::get_all_count(page_request.get_blog_id(), connect).await {
        Ok(close_comment) => {
            data.insert("allComment".into(), to_value!(close_comment));
        }
        Err(e) => {
            return ApiResponse::<String>::error_with_code(ErrorCode::DATABASE_ERROR, e.to_string()).json();
        }
    }
    match CommentService::get_close_count(page_request.get_blog_id(), connect).await {
        Ok(close_comment) => {
            data.insert("closeComment".into(), to_value!(close_comment));
        }
        Err(e) => {
            return ApiResponse::<String>::error_with_code(ErrorCode::DATABASE_ERROR, e.to_string()).json();
        }
    }

    ApiResponse::success_with_msg("获取成功!".to_string(), Some(to_value!(data))).json()
}
