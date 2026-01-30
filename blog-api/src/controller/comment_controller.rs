use crate::app::AppState;
use crate::common::IpRegion;
use crate::common::ParamUtils;
use crate::error::{AppError, WebError};
use crate::model::ApiResponse;
use crate::model::CommentDTO;
use crate::model::SearchRequest;
use crate::service::CommentService;
use crate::service::UserService;
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
) -> Result<ApiResponse<Value>, AppError> {
    let is_admin_comment = {
        log::warn!("验证TOKEN 识别评论者身份!");
        let owenr_user = UserService::find_admin_role(state.get_mysql_pool()).await?;
        let cache_user_info = UserService::get_cache_user_info(&owenr_user.get_username()).await?;
        let token = {
            let authorization = req.headers().get("Authorization");
            if let Some(a) = authorization {
                a.to_str().unwrap_or_default()
            } else {
                ""
            }
        };

        if cache_user_info.cache_info.token.eq(token) {
            log::warn!("评论者身份:管理员");
            true
        } else {
            log::warn!("评论者身份:访客");
            false
        }
    };
    if !is_admin_comment {
        validate_comment_input(&comment_dto)?;
    } else if comment_dto.content.trim().is_empty() {
        return Err(WebError::Validation("评论内容不能为空".to_string()).into());
    }
    let ip = IpRegion::get_real_client_ip(&req);
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
    if comment.content.trim().is_empty() {
        return Err(WebError::Validation("评论内容不能为空".to_string()));
    }
    if comment.content.chars().count() > 250 {
        return Err(WebError::Validation("评论内容不可多于250个字符".to_string()));
    }
    Ok(())
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
