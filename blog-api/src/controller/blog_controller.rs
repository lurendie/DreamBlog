use crate::app::AppState;
use crate::common::ParamUtils;
use crate::error::AppError;
use crate::error::DataBaseError;
use crate::error::WebError;
use crate::error::WebErrorCode;
use crate::middleware::AppClaims;
use crate::model::ApiResponse;
use crate::model::SearchRequest;
use crate::service;
use crate::service::RedisService;
use crate::service::UserService;
use actix_jwt_session::MaybeAuthenticated;
use actix_jwt_session::Uuid;
use actix_web::routes;
use actix_web::web::{self, Json, Query};
use actix_web::HttpRequest;
use rbs::value;
use rbs::Value;
use sea_orm::EntityTrait;
use service::BlogService;
use std::collections::HashMap;

//按置顶、创建时间排序 分页查询博客简要信息列表
#[routes]
//#[options("/site")]
#[get("/blogs")]
pub async fn blogs(
    params: Query<SearchRequest>,
    app: web::Data<AppState>,
) -> Result<ApiResponse<Value>, AppError> {
    //提供默认值page_num
    let query = ParamUtils::validate_request_params(&params.0).await?;
    let db_conn = app.get_mysql_pool();

    let data = BlogService::find_list_by_page(query.page_num, db_conn).await?;
    Ok(ApiResponse::success(Some(value!(data))))
}
#[routes]
#[get("/blog")]
pub async fn blog(
    params: Query<HashMap<String, String>>,
    app: web::Data<AppState>,
    req: HttpRequest,
    auth: Option<MaybeAuthenticated<AppClaims>>,
) -> Result<ApiResponse<Value>, AppError> {
    //获取blog_id参数   不是必要参数，如果没有，则返回参数有误的错误信息
    let id = ParamUtils::get_i64_param(&params, "id")?;

    let db = app.get_mysql_pool();
    // 判断文章是否受密码保护：直接查库取 password 字段，不依赖详情缓存
    let is_protected = crate::entity::blog::Entity::find_by_id(id)
        .one(db)
        .await
        .map(|model| {
            model
                .map(|m| m.password.as_deref().map(|p| !p.is_empty()).unwrap_or(false))
                .unwrap_or(false)
        })
        .unwrap_or(false);

    let mut blog = BlogService::find_id_detail(id, db)
        .await
        .ok_or_else(|| WebError::Validation("blogid参数有误".to_string()))?;

    if is_protected {
        // 未解锁：管理员直接放行；非管理员需携带有效的解锁 token
        let unlocked = if is_admin(auth, &db).await {
            true
        } else {
            let token = req
                .headers()
                .get("Authorization")
                .and_then(|h| h.to_str().ok())
                .unwrap_or("")
                .to_string();
            let token = token.trim().to_string();
            if token.is_empty() {
                false
            } else {
                // Authorization 头可能带 "Bearer " 前缀，需剥离后匹配解锁 token
                let raw_token = token
                    .strip_prefix("Bearer ")
                    .or_else(|| token.strip_prefix("bearer "))
                    .map(|t| t.trim().to_string())
                    .unwrap_or(token);
                match RedisService::get_string::<i64>(format!("blog_unlock:{}", raw_token)).await {
                    Ok(unlocked_blog_id) => unlocked_blog_id == id,
                    Err(_) => false,
                }
            }
        };
        if !unlocked {
            blog.content = "该文章受密码保护，请输入密码后查看。".to_string();
        }
    }
    Ok(ApiResponse::success(Some(value!(blog))))
}

/// 判断当前请求是否为管理员
async fn is_admin(
    auth: Option<MaybeAuthenticated<AppClaims>>,
    db: &sea_orm::DatabaseConnection,
) -> bool {
    match auth.and_then(|v| v.into_option()) {
        Some(claims) => UserService::is_admin_username(&claims.subject, db)
            .await
            .unwrap_or(false),
        None => false,
    }
}

#[routes]
#[get("/category")]
pub async fn category(
    params: Query<HashMap<String, String>>,
    app: web::Data<AppState>,
) -> Result<ApiResponse<Value>, AppError> {
    let category_name = ParamUtils::get_string_param(&params, "categoryName")?;
    //使用新的分页参数验证方法
    let (page_num, _) = ParamUtils::validate_pagination_params(&params)?;
    let page =
        BlogService::find_by_categorya_name(category_name, page_num as usize, app.get_mysql_pool())
            .await;
    Ok(ApiResponse::success(Some(value!(page))))
}

#[routes]
#[get("/tag")]
pub async fn tag(
    params: Query<HashMap<String, String>>,
    app: web::Data<AppState>,
) -> Result<ApiResponse<Value>, AppError> {
    let tag_name = ParamUtils::get_string_param(&params, "tagName")?;
    //使用新的分页参数验证方法
    let (page_num, _) = ParamUtils::validate_pagination_params(&params)?;
    let page =
        BlogService::find_by_tag_name(tag_name, page_num as usize, app.get_mysql_pool()).await;
    Ok(ApiResponse::success(Some(value!(page))))
}

/**
 * 检测Blog PassWrod 的正确性
 */
#[routes]
#[post("/checkBlogPassword")]
pub async fn check_blog_password(
    query: Json<SearchRequest>,
    app: web::Data<AppState>,
) -> Result<ApiResponse<Value>, AppError> {
    let query = ParamUtils::validate_request_params(&query.0).await?;
    // 仅做存在性校验（该文章必须已发布且存在，方可提交密码）
    let _blog_info = BlogService::find_id_detail(query.blog_id, app.get_mysql_pool())
        .await
        .ok_or_else(|| WebError::NotFound(format!("BlogID:{}文章不存在", query.blog_id)))?;
    let password = query.password;
    // 密码比较直接查库（详情缓存中不保存密码字段）
    let db_password = crate::entity::blog::Entity::find_by_id(query.blog_id)
        .one(app.get_mysql_pool())
        .await
        .ok()
        .flatten()
        .and_then(|m| m.password)
        .unwrap_or_default();
    if db_password == password {
        // 密码正确：签发解锁 token 并写入 Redis，作为后续访问该文章的解锁凭证
        let token = Uuid::new_v4().to_string();
        let stored = RedisService::set_string_ttl(
            format!("blog_unlock:{}", token),
            &query.blog_id,
            86400,
        )
        .await;
        if !stored {
            return Err(AppError::from(DataBaseError::Custom(
                "密码验证服务不可用".to_string(),
            )));
        }
        Ok(ApiResponse::success_with_msg(
            "验证成功,密码正确!",
            Some(value!(token)),
        ))
    } else {
        Ok(ApiResponse::<Value>::error_with_code(
            WebErrorCode::JWT_ERROR,
            "密码错误",
        ))
    }
}

#[routes]
#[get("/searchBlog")]
pub async fn search_blog(
    query: Query<HashMap<String, String>>,
    app: web::Data<AppState>,
) -> Result<ApiResponse<Value>, AppError> {
    let blog_title = ParamUtils::get_string_param(&query, "query")?;
    //查找title内容的文章
    let result = BlogService::search_content(blog_title, app.get_mysql_pool()).await?;
    Ok(ApiResponse::success(Some(value!(result))))
}
