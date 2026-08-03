use crate::app::AppState;
use crate::common::{IpRegion, ParamUtils};
use crate::error::AppError;
use crate::error::WebErrorCode;
use crate::model::ApiResponse;
use crate::model::SearchRequest;
use crate::service::{MomentService, RedisService};
use actix_web::web::Path;
use actix_web::{routes, web};
use actix_web::{web::Query, HttpRequest, Responder};
use rbs::value;
use rbs::Value;

//动态
#[routes]
#[get("/moments")]
pub(crate) async fn moments(
    mut query: Query<SearchRequest>,
    app: web::Data<AppState>,
) -> Result<ApiResponse<Value>, AppError> {
    query.0.set_page_size(Some(5));
    let query = ParamUtils::validate_request_params(&query.0).await?;
    //查询所有moments
    let data =
        MomentService::get_public_moments(query.page_num, query.page_size, app.get_mysql_pool())
            .await?;
    Ok(ApiResponse::success(Some(value!(data))))
}

#[routes]
#[post("/moment/like/{id}")]
pub async fn moment_like(
    id: Path<i64>,
    app: web::Data<AppState>,
    req: HttpRequest,
) -> Result<impl Responder, AppError> {
    let id = id.into_inner();
    if id <= 0 {
        return Ok(ApiResponse::<String>::error_with_code(
            WebErrorCode::VALIDATION_ERROR,
            "参数有误!",
        )
        .respond());
    }
    // 点赞限频：同一 IP 10 秒内最多 1 次（Redis 关闭时跳过）
    let ip = IpRegion::get_real_client_ip(
        &req,
        crate::app::CONFIG.get_server_config().trust_proxy,
    );
    let rate_key = format!("moment:like:rate:{}", ip);
    if !RedisService::check_rate_limit(&rate_key, 10).await {
        return Ok(ApiResponse::<String>::error_with_code(
            WebErrorCode::VALIDATION_ERROR,
            "点赞过于频繁，请稍后再试",
        )
        .respond());
    }
    MomentService::moment_like(id, app.get_mysql_pool()).await?;
    Ok(ApiResponse::<String>::success_with_msg("点赞成功", None).respond())
}
