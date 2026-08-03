use crate::app::AppState;
use crate::error::AppError;
use crate::middleware::AppClaims;
use crate::model::{
    ApiResponse, DeletePathRequest, GithubContentsQuery, GithubDeleteRequest, GithubTokenRequest,
    PathQuery, TxyunConfig, UpyunConfig,
};
use crate::service::PictureHostingService;
use actix_jwt_session::Authenticated;
use actix_multipart::Multipart;
use actix_web::{delete, get, post, web};
use futures_util::StreamExt;
use rbs::{value, Value};
use std::collections::HashMap;

#[get("/pictureHosting/configs")]
pub async fn get_configs(
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
) -> Result<ApiResponse<Value>, AppError> {
    let data = PictureHostingService::get_configs(app.get_mysql_pool()).await?;
    Ok(ApiResponse::success(Some(data)))
}

#[post("/pictureHosting/github/user")]
pub async fn github_user(
    _: Authenticated<AppClaims>,
    request: web::Json<GithubTokenRequest>,
) -> Result<ApiResponse<Value>, AppError> {
    let user = PictureHostingService::github_user(&request.token).await?;
    Ok(ApiResponse::success(Some(value!(user))))
}

#[post("/pictureHosting/config/github")]
pub async fn save_github_config(
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
    request: web::Json<GithubTokenRequest>,
) -> Result<ApiResponse<Value>, AppError> {
    let user =
        PictureHostingService::save_github_config(app.get_mysql_pool(), request.token.clone())
            .await?;
    Ok(ApiResponse::success_with_msg(
        "保存成功",
        Some(value!(user)),
    ))
}

#[post("/pictureHosting/config/upyun")]
pub async fn save_upyun_config(
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
    request: web::Json<UpyunConfig>,
) -> Result<ApiResponse<Value>, AppError> {
    PictureHostingService::save_upyun_config(app.get_mysql_pool(), request.into_inner()).await?;
    Ok(ApiResponse::<Value>::success_with_msg("保存成功", None))
}

#[post("/pictureHosting/config/txyun")]
pub async fn save_txyun_config(
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
    request: web::Json<TxyunConfig>,
) -> Result<ApiResponse<Value>, AppError> {
    PictureHostingService::save_txyun_config(app.get_mysql_pool(), request.into_inner()).await?;
    Ok(ApiResponse::<Value>::success_with_msg("保存成功", None))
}

#[delete("/pictureHosting/config/{provider}")]
pub async fn delete_config(
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
    provider: web::Path<String>,
) -> Result<ApiResponse<Value>, AppError> {
    PictureHostingService::delete_config(app.get_mysql_pool(), &provider.into_inner()).await?;
    Ok(ApiResponse::<Value>::success_with_msg("清除成功", None))
}

#[get("/pictureHosting/github/repos")]
pub async fn github_repos(
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
) -> Result<ApiResponse<Value>, AppError> {
    let data = PictureHostingService::github_repos(app.get_mysql_pool()).await?;
    Ok(ApiResponse::success(Some(data)))
}

#[get("/pictureHosting/github/contents")]
pub async fn github_contents(
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
    query: web::Query<GithubContentsQuery>,
) -> Result<ApiResponse<Value>, AppError> {
    let data =
        PictureHostingService::github_contents(app.get_mysql_pool(), &query.repos, &query.path)
            .await?;
    Ok(ApiResponse::success(Some(data)))
}

#[delete("/pictureHosting/github/file")]
pub async fn github_delete(
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
    request: web::Json<GithubDeleteRequest>,
) -> Result<ApiResponse<Value>, AppError> {
    let data = PictureHostingService::github_delete(
        app.get_mysql_pool(),
        &request.repos,
        &request.path,
        &request.sha,
    )
    .await?;
    Ok(ApiResponse::success_with_msg("删除成功", Some(data)))
}

#[post("/pictureHosting/github/upload")]
pub async fn github_upload(
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
    payload: Multipart,
) -> Result<ApiResponse<Value>, AppError> {
    let form = read_upload_form(payload).await?;
    let repos = form.field("repos")?.to_string();
    let path = form.field("path")?.to_string();
    let file_name = form.field("fileName")?.to_string();
    let data = PictureHostingService::github_upload(
        app.get_mysql_pool(),
        &repos,
        &path,
        &file_name,
        form.file_bytes()?,
    )
    .await?;
    Ok(ApiResponse::success_with_msg("上传成功", Some(data)))
}

#[get("/pictureHosting/upyun/contents")]
pub async fn upyun_contents(
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
    query: web::Query<PathQuery>,
) -> Result<ApiResponse<Value>, AppError> {
    let data = PictureHostingService::upyun_contents(app.get_mysql_pool(), &query.path).await?;
    Ok(ApiResponse::success(Some(data)))
}

#[delete("/pictureHosting/upyun/file")]
pub async fn upyun_delete(
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
    request: web::Json<DeletePathRequest>,
) -> Result<ApiResponse<Value>, AppError> {
    let data = PictureHostingService::upyun_delete(app.get_mysql_pool(), &request.path).await?;
    Ok(ApiResponse::success_with_msg("删除成功", Some(data)))
}

#[post("/pictureHosting/upyun/upload")]
pub async fn upyun_upload(
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
    payload: Multipart,
) -> Result<ApiResponse<Value>, AppError> {
    let form = read_upload_form(payload).await?;
    let path = form.field("path")?.to_string();
    let file_name = form.field("fileName")?.to_string();
    let data = PictureHostingService::upyun_upload(
        app.get_mysql_pool(),
        &path,
        &file_name,
        form.file_bytes()?,
    )
    .await?;
    Ok(ApiResponse::success_with_msg("上传成功", Some(data)))
}

#[get("/pictureHosting/txyun/contents")]
pub async fn txyun_contents(
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
    query: web::Query<PathQuery>,
) -> Result<ApiResponse<Value>, AppError> {
    let data = PictureHostingService::txyun_contents(app.get_mysql_pool(), &query.path).await?;
    Ok(ApiResponse::success(Some(data)))
}

#[delete("/pictureHosting/txyun/file")]
pub async fn txyun_delete(
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
    request: web::Json<DeletePathRequest>,
) -> Result<ApiResponse<Value>, AppError> {
    let data = PictureHostingService::txyun_delete(app.get_mysql_pool(), &request.path).await?;
    Ok(ApiResponse::success_with_msg("删除成功", Some(data)))
}

#[post("/pictureHosting/txyun/upload")]
pub async fn txyun_upload(
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
    payload: Multipart,
) -> Result<ApiResponse<Value>, AppError> {
    let form = read_upload_form(payload).await?;
    let path = form.field("path")?.to_string();
    let file_name = form.field("fileName")?.to_string();
    let data = PictureHostingService::txyun_upload(
        app.get_mysql_pool(),
        &path,
        &file_name,
        form.file_bytes()?,
    )
    .await?;
    Ok(ApiResponse::success_with_msg("上传成功", Some(data)))
}

struct UploadForm {
    fields: HashMap<String, String>,
    file: Option<Vec<u8>>,
}

impl UploadForm {
    fn field(&self, key: &str) -> Result<&str, AppError> {
        self.fields
            .get(key)
            .map(String::as_str)
            .ok_or_else(|| AppError::Custom(format!("缺少上传字段: {}", key)))
    }

    fn file_bytes(self) -> Result<Vec<u8>, AppError> {
        self.file
            .ok_or_else(|| AppError::Custom("缺少上传文件".to_string()))
    }
}

async fn read_upload_form(mut payload: Multipart) -> Result<UploadForm, AppError> {
    // 上传文件与表单字段大小限制，防止超大请求耗尽内存
    const MAX_FILE_BYTES: usize = 20 * 1024 * 1024; // 20MB
    const MAX_FIELD_BYTES: usize = 4 * 1024; // 4KB

    let mut fields = HashMap::new();
    let mut file = None;
    while let Some(item) = payload.next().await {
        let mut field = item.map_err(|e| AppError::Custom(e.to_string()))?;
        let name = field
            .content_disposition()
            .get_name()
            .map(str::to_string)
            .unwrap_or_default();
        let mut bytes = Vec::new();
        while let Some(chunk) = field.next().await {
            bytes.extend_from_slice(&chunk.map_err(|e| AppError::Custom(e.to_string()))?);
            let limit = if name == "file" {
                MAX_FILE_BYTES
            } else {
                MAX_FIELD_BYTES
            };
            if bytes.len() > limit {
                return Err(AppError::Custom(if name == "file" {
                    "上传文件不能超过 20MB".to_string()
                } else {
                    "上传表单字段过大".to_string()
                }));
            }
        }
        if name == "file" {
            file = Some(bytes);
        } else {
            fields.insert(
                name,
                String::from_utf8(bytes).map_err(|e| AppError::Custom(e.to_string()))?,
            );
        }
    }
    Ok(UploadForm { fields, file })
}
