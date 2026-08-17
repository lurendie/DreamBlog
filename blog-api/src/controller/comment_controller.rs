use crate::app::AppState;
use crate::common::IpRegion;
use crate::common::ParamUtils;
use crate::error::{AppError, WebError};
use crate::middleware::AppClaims;
use crate::model::ApiResponse;
use crate::model::CommentDTO;
use crate::model::SearchRequest;
use crate::service::{CommentService, UserService};
use actix_jwt_session::MaybeAuthenticated;
use actix_web::get;
use actix_web::routes;
use actix_web::web::{self, Query};
use actix_web::HttpRequest;
use rbs::value;
use rbs::value::map::ValueMap;
use rbs::Value;

#[get("/comments")]
pub(crate) async fn get_comments(
    query: Query<SearchRequest>,
    app: web::Data<AppState>,
) -> Result<ApiResponse<Value>, AppError> {
    let query = ParamUtils::validate_request_params(&query.0).await?;
    let connect = app.get_mysql_pool();
    let list =
        CommentService::find_by_id_comments(query.page_num, query.blog_id, query.page, connect)
            .await?;
    let mut data = ValueMap::new();
    data.insert("comments".into(), value!(list));

    let all_comment = CommentService::get_all_count(query.blog_id, query.page, connect).await?;
    data.insert("allComment".into(), value!(all_comment));
    let close_count = CommentService::get_close_count(query.blog_id, query.page, connect).await?;
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
    req: HttpRequest,
    auth: Option<MaybeAuthenticated<AppClaims>>,
) -> Result<ApiResponse<Value>, AppError> {
    let authenticated_username = auth
        .and_then(|value| value.into_option())
        .map(|claims| claims.subject.clone());
    let is_admin_comment = match authenticated_username {
        Some(username) => UserService::is_admin_username(&username, &state.mysql_connection)
            .await
            .unwrap_or(false),
        None => false,
    };
    if !is_admin_comment {
        validate_comment_input(&comment_dto)?;
    } else if comment_dto.content.trim().is_empty() {
        return Err(WebError::Validation("评论内容不能为空".to_string()).into());
    }
    let ip = IpRegion::get_real_client_ip(&req, crate::app::CONFIG.get_server_config().trust_proxy);
    CommentService::save_comment(comment_dto.0, &state.mysql_connection, ip, is_admin_comment)
        .await?;
    Ok(ApiResponse::<Value>::success(None))
}

fn validate_comment_input(comment: &CommentDTO) -> Result<(), WebError> {
    if comment.nickname.trim().is_empty() {
        return Err(WebError::Validation("昵称不能为空".to_string()));
    }
    if comment.email.trim().is_empty() {
        return Err(WebError::Validation("邮箱不能为空".to_string()));
    }
    if !is_basic_email(&comment.email) {
        return Err(WebError::Validation("邮箱格式不正确".to_string()));
    }
    // 网站地址校验：允许为空；非空时必须是以 http:// 或 https:// 开头的简单字符串
    let website = comment.website.trim();
    if !website.is_empty() && !is_basic_website(website) {
        return Err(WebError::Validation("网站地址格式不正确".to_string()));
    }
    if comment.content.trim().is_empty() {
        return Err(WebError::Validation("评论内容不能为空".to_string()));
    }
    if comment.content.chars().count() > 250 {
        return Err(WebError::Validation(
            "评论内容不可多于250个字符".to_string(),
        ));
    }
    Ok(())
}

/// 简单的网站地址校验：以 http:// 或 https:// 开头、长度不超过 255，
/// 且排除包含 javascript: / data: / vbscript: 的危险字符串（避免新增 url 依赖）。
fn is_basic_website(website: &str) -> bool {
    if website.len() > 255 {
        return false;
    }
    if !(website.starts_with("http://") || website.starts_with("https://")) {
        return false;
    }
    let lower = website.to_lowercase();
    if lower.contains("javascript:") || lower.contains("data:") || lower.contains("vbscript:") {
        return false;
    }
    true
}

fn is_basic_email(email: &str) -> bool {
    let email = email.trim();
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return false;
    }
    let (local, domain) = (parts[0], parts[1]);
    if local.is_empty() || domain.is_empty() {
        return false;
    }
    domain.contains('.') && !domain.starts_with('.') && !domain.ends_with('.')
}
