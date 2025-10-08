use crate::app::AppState;
use crate::common::ParamUtils;
use crate::error::AppError;
use crate::model::ApiResponse;
use crate::model::CommentDTO;
use crate::model::SearchRequest;
use crate::service::CommentService;
use actix_web::get;
use actix_web::routes;
use actix_web::web::{self, Query};
use rbs::value;
use rbs::value::map::ValueMap;
use rbs::Value;

#[get("/comments")]
pub(crate) async fn get_comments(
    data: Query<SearchRequest>,
    app: web::Data<AppState>,
) -> Result<ApiResponse<Value>, AppError> {
    let page_request = data.into_inner();
    ParamUtils::validate_request_params(&page_request).await?;
    let connect = app.get_mysql_pool();
    let list = CommentService::find_by_id_comments(
        page_request.get_page_num(),
        page_request.get_blog_id(),
        page_request.get_page(),
        connect,
    )
    .await?;
    let mut data = ValueMap::new();
    data.insert("comments".into(), value!(list));

    let all_comment =
        CommentService::get_all_count(page_request.get_blog_id(), page_request.get_page(), connect)
            .await?;
    data.insert("allComment".into(), value!(all_comment));
    let close_count = CommentService::get_close_count(
        page_request.get_blog_id(),
        page_request.get_page(),
        connect,
    )
    .await?;
    data.insert("closeComment".into(), value!(close_count));

    Ok(ApiResponse::success_with_msg(
        "获取成功!",
        Some(value!(data)),
    ))
}

#[routes]
#[post("/comment")]
pub async fn save_comment(
    state: web::Data<AppState>,
    comment_dto: web::Json<CommentDTO>,
) -> Result<ApiResponse<Value>, AppError> {
    CommentService::save_comment(comment_dto.0, &state.mysql_connection).await?;
    Ok(ApiResponse::<Value>::success(None))
}
