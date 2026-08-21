use crate::entity::site_setting;
use crate::error::{AppError, WebError};
use crate::model::{GithubConfig, TxyunConfig, UpyunConfig};
use base64::{engine::general_purpose, Engine as _};
use hmac::{Hmac, Mac};
use rbs::value;
use rbs::Value;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE, HOST};
use reqwest::{Client, StatusCode};
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
};
use serde_json::json;
use sha1::{Digest, Sha1};
use std::collections::BTreeMap;
use std::sync::LazyLock;
use std::time::{SystemTime, UNIX_EPOCH};

const SETTING_TYPE: i32 = 9;
const GITHUB_KEY: &str = "pictureHosting.github";
const UPYUN_KEY: &str = "pictureHosting.upyun";
const TXYUN_KEY: &str = "pictureHosting.txyun";

type HmacSha1 = Hmac<Sha1>;

/// 模块级共享的 HTTP 客户端，统一携带超时，防止出站请求长时间挂起
static HTTP_CLIENT: LazyLock<Client> = LazyLock::new(|| {
    Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .connect_timeout(std::time::Duration::from_secs(10))
        .build()
        .unwrap_or_else(|_| Client::new())
});

fn http_client() -> &'static Client {
    &HTTP_CLIENT
}

pub struct PictureHostingService;

impl PictureHostingService {
    pub async fn get_configs(db: &DatabaseConnection) -> Result<Value, AppError> {
        let github = Self::get_config::<GithubConfig>(db, GITHUB_KEY).await?;
        let upyun = Self::get_config::<UpyunConfig>(db, UPYUN_KEY).await?;
        let txyun = Self::get_config::<TxyunConfig>(db, TXYUN_KEY).await?;

        let data = json!({
            "github": github.map(|config| {
                json!({
                    "configured": true,
                    "userInfo": config.user_info,
                })
            }).unwrap_or_else(|| json!({"configured": false})),
            "upyun": upyun.map(|config| {
                json!({
                    "configured": true,
                    "bucketName": config.bucket_name,
                    "domain": config.domain,
                })
            }).unwrap_or_else(|| json!({"configured": false})),
            "txyun": txyun.map(|config| {
                json!({
                    "configured": true,
                    "bucketName": config.bucket_name,
                    "region": config.region,
                    "domain": config.domain,
                })
            }).unwrap_or_else(|| json!({"configured": false})),
        });
        Ok(value!(data))
    }

    pub async fn github_user(token: &str) -> Result<serde_json::Value, AppError> {
        Self::github_json_request(
            token,
            http_client().get("https://api.github.com/user"),
            "GitHub用户信息",
        )
        .await
    }

    pub async fn save_github_config(
        db: &DatabaseConnection,
        token: String,
    ) -> Result<serde_json::Value, AppError> {
        let user_info = Self::github_user(&token).await?;
        let config = GithubConfig {
            token,
            user_info: Some(user_info.clone()),
        };
        Self::save_config(db, GITHUB_KEY, "GitHub图床配置", &config).await?;
        Ok(user_info)
    }

    pub async fn save_upyun_config(
        db: &DatabaseConnection,
        config: UpyunConfig,
    ) -> Result<(), AppError> {
        Self::save_config(db, UPYUN_KEY, "又拍云图床配置", &config).await
    }

    pub async fn save_txyun_config(
        db: &DatabaseConnection,
        config: TxyunConfig,
    ) -> Result<(), AppError> {
        Self::save_config(db, TXYUN_KEY, "腾讯云图床配置", &config).await
    }

    pub async fn delete_config(db: &DatabaseConnection, provider: &str) -> Result<(), AppError> {
        let key = match provider {
            "github" => GITHUB_KEY,
            "upyun" => UPYUN_KEY,
            "txyun" => TXYUN_KEY,
            _ => return Err(AppError::Custom("未知图床类型".to_string())),
        };
        site_setting::Entity::delete_many()
            .filter(site_setting::Column::NameEn.eq(key))
            .exec(db)
            .await?;
        Ok(())
    }

    pub async fn github_repos(db: &DatabaseConnection) -> Result<Value, AppError> {
        let config = Self::require_github(db).await?;
        let url = "https://api.github.com/user/repos?visibility=all&affiliation=owner&per_page=100";
        Self::github_request(&config.token, http_client().get(url), "GitHub仓库列表").await
    }

    pub async fn github_contents(
        db: &DatabaseConnection,
        repos: &str,
        path: &str,
    ) -> Result<Value, AppError> {
        let config = Self::require_github(db).await?;
        let login = config
            .user_info
            .as_ref()
            .and_then(|user| user.get("login"))
            .and_then(|login| login.as_str())
            .ok_or_else(|| AppError::Custom("GitHub用户信息缺失".to_string()))?;
        let url = Self::github_contents_url(login, repos, path);
        let target = format!("GitHub仓库内容 {}/{}", repos, path.trim_start_matches('/'));
        Self::github_request(&config.token, http_client().get(url), &target).await
    }

    pub async fn github_delete(
        db: &DatabaseConnection,
        repos: &str,
        path: &str,
        sha: &str,
    ) -> Result<Value, AppError> {
        let config = Self::require_github(db).await?;
        let login = config
            .user_info
            .as_ref()
            .and_then(|user| user.get("login"))
            .and_then(|login| login.as_str())
            .ok_or_else(|| AppError::Custom("GitHub用户信息缺失".to_string()))?;
        let url = Self::github_contents_url(login, repos, path);
        let body = json!({
            "message": "Delete file via PictureHosting",
            "sha": sha,
        });
        let target = format!("GitHub删除文件 {}/{}", repos, path.trim_start_matches('/'));
        Self::github_request(
            &config.token,
            http_client().delete(url).json(&body),
            &target,
        )
        .await
    }

    pub async fn github_upload(
        db: &DatabaseConnection,
        repos: &str,
        path: &str,
        file_name: &str,
        bytes: Vec<u8>,
    ) -> Result<Value, AppError> {
        let config = Self::require_github(db).await?;
        let login = config
            .user_info
            .as_ref()
            .and_then(|user| user.get("login"))
            .and_then(|login| login.as_str())
            .ok_or_else(|| AppError::Custom("GitHub用户信息缺失".to_string()))?;
        let path = Self::join_path(path, file_name);
        let url = Self::github_contents_url(login, repos, &path);
        let body = json!({
            "message": "Add files via PictureHosting",
            "content": general_purpose::STANDARD.encode(bytes),
        });
        let target = format!("GitHub上传文件 {}/{}", repos, path.trim_start_matches('/'));
        Self::github_request(&config.token, http_client().put(url).json(&body), &target).await
    }

    pub async fn upyun_contents(db: &DatabaseConnection, path: &str) -> Result<Value, AppError> {
        let config = Self::require_upyun(db).await?;
        let path = Self::normalize_slash(path);
        let url = format!("https://v0.api.upyun.com/{}{}", config.bucket_name, path);
        Self::upyun_request(&config, http_client().get(url)).await
    }

    pub async fn upyun_delete(db: &DatabaseConnection, path: &str) -> Result<Value, AppError> {
        let config = Self::require_upyun(db).await?;
        let path = Self::normalize_slash(path);
        let url = format!("https://v0.api.upyun.com/{}{}", config.bucket_name, path);
        Self::upyun_request(&config, http_client().delete(url)).await
    }

    pub async fn upyun_upload(
        db: &DatabaseConnection,
        path: &str,
        file_name: &str,
        bytes: Vec<u8>,
    ) -> Result<Value, AppError> {
        let config = Self::require_upyun(db).await?;
        let path = Self::join_path(path, file_name);
        let url = format!("https://v0.api.upyun.com/{}{}", config.bucket_name, path);
        Self::upyun_request(&config, http_client().put(url).body(bytes)).await
    }

    pub async fn txyun_contents(db: &DatabaseConnection, path: &str) -> Result<Value, AppError> {
        let config = Self::require_txyun(db).await?;
        let path = Self::normalize_cos_prefix(path);
        let query = if path.is_empty() {
            "delimiter=%2F".to_string()
        } else {
            format!("delimiter=%2F&prefix={}", urlencoding::encode(&path))
        };
        let response = Self::cos_request(&config, "GET", "", Some(&query), None).await?;
        Ok(Self::parse_cos_list(&response, &path, &config.domain))
    }

    pub async fn txyun_delete(db: &DatabaseConnection, path: &str) -> Result<Value, AppError> {
        let config = Self::require_txyun(db).await?;
        Self::cos_request(&config, "DELETE", path, None, None).await?;
        Ok(value!({}))
    }

    pub async fn txyun_upload(
        db: &DatabaseConnection,
        path: &str,
        file_name: &str,
        bytes: Vec<u8>,
    ) -> Result<Value, AppError> {
        let config = Self::require_txyun(db).await?;
        let key = Self::join_cos_path(path, file_name);
        Self::cos_request(&config, "PUT", &key, None, Some(bytes)).await?;
        Ok(value!({}))
    }

    async fn require_github(db: &DatabaseConnection) -> Result<GithubConfig, AppError> {
        Self::get_config(db, GITHUB_KEY)
            .await?
            .ok_or_else(|| AppError::Custom("请先配置GitHub图床".to_string()))
    }

    async fn require_upyun(db: &DatabaseConnection) -> Result<UpyunConfig, AppError> {
        Self::get_config(db, UPYUN_KEY)
            .await?
            .ok_or_else(|| AppError::Custom("请先配置又拍云图床".to_string()))
    }

    async fn require_txyun(db: &DatabaseConnection) -> Result<TxyunConfig, AppError> {
        Self::get_config(db, TXYUN_KEY)
            .await?
            .ok_or_else(|| AppError::Custom("请先配置腾讯云图床".to_string()))
    }

    async fn get_config<T: serde::de::DeserializeOwned>(
        db: &DatabaseConnection,
        key: &str,
    ) -> Result<Option<T>, AppError> {
        let model = site_setting::Entity::find()
            .filter(site_setting::Column::NameEn.eq(key))
            .one(db)
            .await?;
        match model.and_then(|model| model.value) {
            Some(value) if !value.trim().is_empty() => {
                // 新数据为 v1: 加密格式；历史明文数据兼容读取
                let json_str = match crate::common::decrypt_secret(&value) {
                    Ok(plain) => plain,
                    Err(e) if e == "非加密数据" => value,
                    Err(e) => return Err(AppError::Custom(format!("图床配置解密失败: {e}"))),
                };
                Ok(Some(serde_json::from_str(&json_str)?))
            }
            _ => Ok(None),
        }
    }

    async fn save_config<T: serde::Serialize>(
        db: &DatabaseConnection,
        key: &str,
        name_zh: &str,
        config: &T,
    ) -> Result<(), AppError> {
        let plain = serde_json::to_string(config)?;
        // 凭据静态加密后再入库，防止数据库泄露直接暴露 token/密码/SecretKey
        let value = crate::common::encrypt_secret(&plain).map_err(AppError::Custom)?;
        let model = site_setting::Entity::find()
            .filter(site_setting::Column::NameEn.eq(key))
            .one(db)
            .await?;
        if let Some(model) = model {
            let mut active = model.into_active_model();
            active.value = Set(Some(value));
            active.update(db).await?;
        } else {
            site_setting::Entity::insert(site_setting::ActiveModel {
                name_en: Set(Some(key.to_string())),
                name_zh: Set(Some(name_zh.to_string())),
                value: Set(Some(value)),
                r#type: Set(Some(SETTING_TYPE)),
                ..Default::default()
            })
            .exec(db)
            .await?;
        }
        Ok(())
    }

    async fn github_request(
        token: &str,
        builder: reqwest::RequestBuilder,
        target: &str,
    ) -> Result<Value, AppError> {
        let response = Self::github_json_request(token, builder, target).await?;
        Ok(value!(response))
    }

    async fn github_json_request(
        token: &str,
        builder: reqwest::RequestBuilder,
        target: &str,
    ) -> Result<serde_json::Value, AppError> {
        let response = builder
            .header(AUTHORIZATION, format!("token {}", token))
            .header("User-Agent", "Dream Blog")
            .send()
            .await
            .map_err(Self::external_error)?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(Self::github_status_error(status, target, &body));
        }

        let response = response
            .json::<serde_json::Value>()
            .await
            .map_err(Self::external_error)?;
        Ok(response)
    }

    async fn upyun_request(
        config: &UpyunConfig,
        builder: reqwest::RequestBuilder,
    ) -> Result<Value, AppError> {
        let token =
            general_purpose::STANDARD.encode(format!("{}:{}", config.username, config.password));
        let response = builder
            .header(ACCEPT, "application/json")
            .header(AUTHORIZATION, format!("Basic {}", token))
            .send()
            .await
            .map_err(Self::external_error)?
            .error_for_status()
            .map_err(Self::external_error)?;
        let text = response.text().await.map_err(Self::external_error)?;
        if text.trim().is_empty() {
            return Ok(value!({}));
        }
        Ok(value!(serde_json::from_str::<serde_json::Value>(&text)
            .unwrap_or_else(|_| json!({ "raw": text }))))
    }

    async fn cos_request(
        config: &TxyunConfig,
        method: &str,
        key: &str,
        query: Option<&str>,
        body: Option<Vec<u8>>,
    ) -> Result<String, AppError> {
        let host = format!("{}.cos.{}.myqcloud.com", config.bucket_name, config.region);
        let key = key.trim_start_matches('/');
        let uri = if key.is_empty() {
            "/".to_string()
        } else {
            format!(
                "/{}",
                key.split('/')
                    .map(urlencoding::encode)
                    .collect::<Vec<_>>()
                    .join("/")
            )
        };
        let url = if let Some(query) = query {
            format!("https://{}{}?{}", host, uri, query)
        } else {
            format!("https://{}{}", host, uri)
        };
        let authorization =
            Self::cos_authorization(config, method, &uri, query.unwrap_or(""), &host)?;
        let mut headers = HeaderMap::new();
        headers.insert(
            HOST,
            HeaderValue::from_str(&host).map_err(|e| AppError::Custom(e.to_string()))?,
        );
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&authorization).map_err(|e| AppError::Custom(e.to_string()))?,
        );
        if body.is_some() {
            headers.insert(
                CONTENT_TYPE,
                HeaderValue::from_static("application/octet-stream"),
            );
        }
        let client = http_client();
        let request = match method {
            "GET" => client.get(url),
            "PUT" => client.put(url),
            "DELETE" => client.delete(url),
            _ => return Err(AppError::Custom("不支持的COS方法".to_string())),
        }
        .headers(headers);
        let request = if let Some(body) = body {
            request.body(body)
        } else {
            request
        };
        let response = request
            .send()
            .await
            .map_err(Self::external_error)?
            .error_for_status()
            .map_err(Self::external_error)?
            .text()
            .await
            .map_err(Self::external_error)?;
        Ok(response)
    }

    fn cos_authorization(
        config: &TxyunConfig,
        method: &str,
        uri: &str,
        query: &str,
        host: &str,
    ) -> Result<String, AppError> {
        let start = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| AppError::Custom(e.to_string()))?
            .as_secs();
        let end = start + 600;
        let key_time = format!("{};{}", start, end);
        let sign_key = Self::hmac_sha1(&config.secret_key, &key_time)?;
        let http_method = method.to_lowercase();
        let http_uri = uri;
        let http_parameters = Self::normalize_query(query);
        let http_headers = format!("host={}", host.to_lowercase());
        let header_list = "host";
        let url_param_list = Self::query_keys(query);
        let http_string = format!(
            "{}\n{}\n{}\n{}\n",
            http_method, http_uri, http_parameters, http_headers
        );
        let string_to_sign = format!(
            "sha1\n{}\n{}\n",
            key_time,
            hex::encode(Sha1::digest(http_string.as_bytes()))
        );
        let signature = Self::hmac_sha1_hex(&sign_key, &string_to_sign)?;
        Ok(format!(
            "q-sign-algorithm=sha1&q-ak={}&q-sign-time={}&q-key-time={}&q-header-list={}&q-url-param-list={}&q-signature={}",
            config.secret_id, key_time, key_time, header_list, url_param_list, signature
        ))
    }

    fn hmac_sha1(key: &str, data: &str) -> Result<Vec<u8>, AppError> {
        let mut mac = HmacSha1::new_from_slice(key.as_bytes())
            .map_err(|e| AppError::Custom(e.to_string()))?;
        mac.update(data.as_bytes());
        Ok(mac.finalize().into_bytes().to_vec())
    }

    fn hmac_sha1_hex(key: &[u8], data: &str) -> Result<String, AppError> {
        let mut mac = HmacSha1::new_from_slice(key).map_err(|e| AppError::Custom(e.to_string()))?;
        mac.update(data.as_bytes());
        Ok(hex::encode(mac.finalize().into_bytes()))
    }

    fn normalize_query(query: &str) -> String {
        let mut map = BTreeMap::new();
        for pair in query.split('&').filter(|pair| !pair.is_empty()) {
            let mut parts = pair.splitn(2, '=');
            let key = parts.next().unwrap_or_default().to_lowercase();
            let value = parts.next().unwrap_or_default().to_string();
            map.insert(key, value);
        }
        map.into_iter()
            .map(|(key, value)| format!("{}={}", key, value))
            .collect::<Vec<_>>()
            .join("&")
    }

    fn query_keys(query: &str) -> String {
        let mut keys = query
            .split('&')
            .filter(|pair| !pair.is_empty())
            .filter_map(|pair| pair.split('=').next())
            .map(|key| key.to_lowercase())
            .collect::<Vec<_>>();
        keys.sort();
        keys.join(";")
    }

    fn parse_cos_list(xml: &str, prefix: &str, domain: &str) -> Value {
        let mut prefixes = vec![];
        let mut contents = vec![];
        for item in Self::xml_values(xml, "Prefix") {
            if item != prefix {
                let name = item
                    .trim_start_matches(prefix)
                    .trim_end_matches('/')
                    .to_string();
                if !name.is_empty() {
                    prefixes.push(json!({ "Prefix": item, "name": name }));
                }
            }
        }
        for key in Self::xml_values(xml, "Key") {
            if key.ends_with('/') {
                continue;
            }
            let name = key.trim_start_matches(prefix).to_string();
            contents.push(json!({
                "Key": key,
                "path": key,
                "name": name,
                "cdn_url": format!("{}{}", Self::ensure_trailing_slash(domain), key),
            }));
        }
        value!({ "CommonPrefixes": prefixes, "Contents": contents })
    }

    fn xml_values(xml: &str, tag: &str) -> Vec<String> {
        let open = format!("<{}>", tag);
        let close = format!("</{}>", tag);
        let mut values = vec![];
        let mut rest = xml;
        while let Some(start) = rest.find(&open) {
            let value_start = start + open.len();
            if let Some(end) = rest[value_start..].find(&close) {
                values.push(rest[value_start..value_start + end].to_string());
                rest = &rest[value_start + end + close.len()..];
            } else {
                break;
            }
        }
        values
    }

    fn normalize_slash(path: &str) -> String {
        let path = path.trim();
        if path.starts_with('/') {
            path.to_string()
        } else {
            format!("/{}", path)
        }
    }

    fn normalize_cos_prefix(path: &str) -> String {
        let path = path.trim_start_matches('/').to_string();
        if !path.is_empty() && !path.ends_with('/') {
            format!("{}/", path)
        } else {
            path
        }
    }

    fn join_path(path: &str, file_name: &str) -> String {
        let path = path.trim();
        if path.is_empty() || path == "/" {
            format!("/{}", file_name)
        } else {
            format!(
                "{}/{}",
                Self::normalize_slash(path).trim_end_matches('/'),
                file_name
            )
        }
    }

    fn join_cos_path(path: &str, file_name: &str) -> String {
        let path = path.trim_start_matches('/').trim_end_matches('/');
        if path.is_empty() {
            file_name.to_string()
        } else {
            format!("{}/{}", path, file_name)
        }
    }

    fn ensure_trailing_slash(domain: &str) -> String {
        if domain.ends_with('/') {
            domain.to_string()
        } else {
            format!("{}/", domain)
        }
    }

    fn external_error(error: reqwest::Error) -> AppError {
        AppError::Custom(format!("图床服务请求失败: {}", error))
    }

    fn github_contents_url(login: &str, repos: &str, path: &str) -> String {
        let path = Self::encode_github_path(path);
        if path.is_empty() {
            format!("https://api.github.com/repos/{}/{}/contents", login, repos)
        } else {
            format!(
                "https://api.github.com/repos/{}/{}/contents/{}",
                login, repos, path
            )
        }
    }

    fn encode_github_path(path: &str) -> String {
        path.trim_matches('/')
            .split('/')
            .filter(|part| !part.is_empty())
            .map(|part| urlencoding::encode(part).into_owned())
            .collect::<Vec<_>>()
            .join("/")
    }

    fn github_status_error(status: StatusCode, target: &str, body: &str) -> AppError {
        let message = serde_json::from_str::<serde_json::Value>(body)
            .ok()
            .and_then(|value| {
                value
                    .get("message")
                    .and_then(|message| message.as_str())
                    .map(str::to_string)
            })
            .filter(|message| !message.trim().is_empty())
            .unwrap_or_else(|| status.canonical_reason().unwrap_or("未知错误").to_string());

        let text = match status {
            StatusCode::UNAUTHORIZED => {
                format!("GitHub Token 无效或已过期: {message}")
            }
            StatusCode::FORBIDDEN => {
                format!("GitHub Token 权限不足或触发限流，无法访问 {target}: {message}")
            }
            StatusCode::NOT_FOUND => {
                format!("GitHub 仓库或路径不存在，或当前 Token 无权限访问 {target}: {message}")
            }
            StatusCode::CONFLICT => {
                format!("GitHub 文件版本冲突，请刷新后重试 {target}: {message}")
            }
            StatusCode::UNPROCESSABLE_ENTITY => {
                format!("GitHub 请求参数不合法或文件已存在 {target}: {message}")
            }
            _ => {
                format!(
                    "GitHub 服务请求失败({} {}) {target}: {message}",
                    status.as_u16(),
                    status.canonical_reason().unwrap_or("")
                )
            }
        };

        let web_error = match status {
            StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => WebError::Unauthorized(text),
            StatusCode::NOT_FOUND => WebError::NotFound(text),
            _ => WebError::Business(text),
        };
        AppError::WebError(web_error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::WebErrorCode;

    #[test]
    fn github_contents_url_uses_contents_endpoint_for_root() {
        let url = PictureHostingService::github_contents_url("lurendie", "DreamBlog", "");

        assert_eq!(
            url,
            "https://api.github.com/repos/lurendie/DreamBlog/contents"
        );
    }

    #[test]
    fn github_contents_url_encodes_each_path_segment() {
        let url = PictureHostingService::github_contents_url(
            "lurendie",
            "image-hosting",
            "/中文 目录/a#b.png",
        );

        assert_eq!(
            url,
            "https://api.github.com/repos/lurendie/image-hosting/contents/%E4%B8%AD%E6%96%87%20%E7%9B%AE%E5%BD%95/a%23b.png"
        );
    }

    #[test]
    fn github_404_is_returned_as_visible_not_found_error() {
        let error = PictureHostingService::github_status_error(
            StatusCode::NOT_FOUND,
            "GitHub仓库内容 image-hosting/",
            r#"{"message":"Not Found"}"#,
        );

        match error {
            AppError::WebError(web_error) => {
                assert_eq!(web_error.error_code(), WebErrorCode::NOT_FOUND);
                assert!(web_error.message().contains("image-hosting"));
                assert!(web_error.message().contains("无权限访问"));
            }
            other => panic!("expected WebError, got {other:?}"),
        }
    }
}
