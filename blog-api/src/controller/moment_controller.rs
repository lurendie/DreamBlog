use crate::app::AppState;
use crate::common::ParamUtils;
use crate::error::AppError;
use crate::error::WebErrorCode;
use crate::model::ApiResponse;
use crate::model::SearchRequest;
use crate::service::MomentService;
use actix_web::web::Path;
use actix_web::{routes, web};
use actix_web::{web::Query, Responder};
use rbs::value;
use rbs::Value;

//动态
#[routes]
#[get("/moments")]
pub(crate) async fn moments(
    mut query: Query<SearchRequest>,
    app: web::Data<AppState>,
) -> Result<ApiResponse<Value>, AppError> {
    ParamUtils::validate_request_params(&query.0).await?;
    //查询所有moments
    query.0.set_page_size(Some(5));
    let data = MomentService::get_public_moments(
        query.0.get_page_num(),
        query.0.get_page_size(),
        app.get_mysql_pool(),
    )
    .await?;
    Ok(ApiResponse::success(Some(value!(data))))
}

#[routes]
#[post("/moment/like/{id}")]
pub async fn moment_like(
    id: Path<i64>,
    app: web::Data<AppState>,
) -> Result<impl Responder, AppError> {
    let id = id.into_inner();
    if id <= 0 {
        return Ok(ApiResponse::<String>::error_with_code(
            WebErrorCode::VALIDATION_ERROR,
            "参数有误!",
        )
        .respond());
    }
    MomentService::moment_like(id, app.get_mysql_pool()).await?;
    Ok(ApiResponse::<String>::success_with_msg("点赞成功", None).respond())
}
