/*
 * @Author: lurendie
 * @Date: 2026-03-12
 * @LastEditors: Codex
 * @LastEditTime: 2026-03-12
 *
 */
use std::{
    future::{ready, Future, Ready},
    pin::Pin,
    rc::Rc,
    sync::Arc,
};

use actix_http::{BoxedPayloadStream, Payload};
use actix_jwt_session::Authenticated;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::Method,
    web::Data,
    Error, HttpMessage,
};
use bytes::{Bytes, BytesMut};
use chrono::Local;
use futures_util::StreamExt;
use parking_lot::Mutex;

use crate::{
    app::AppState,
    common::{IpRegion, UserAgent},
    middleware::AppClaims,
    service::OperationLogService,
};

/**
 * 记录后台操作日志
 */
#[derive(Default, Debug)]
pub struct OperationLog;

impl<S, B> Transform<S, ServiceRequest> for OperationLog
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = OperationLogMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(OperationLogMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct OperationLogMiddleware<S> {
    service: Rc<S>,
}

type LocalBoxFuture<T> = Pin<Box<dyn Future<Output = T> + 'static>>;

impl<S, B> Service<ServiceRequest> for OperationLogMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let method = req.method().clone();
        let uri = req.uri().path().to_string();
        let query_string = req.query_string().to_string();
        let user_agent_str = req
            .headers()
            .get("User-Agent")
            .and_then(|h| h.to_str().ok())
            .unwrap_or("")
            .to_string();
        let ip = IpRegion::get_real_client_ip(
            &req.request(),
            crate::app::CONFIG.get_server_config().trust_proxy,
        );
        let app_state = req.app_data::<Data<AppState>>().cloned();
        let auth_username = req
            .extensions()
            .get::<Authenticated<AppClaims>>()
            .map(|v| v.subject.clone());

        let service = self.service.clone();

        Box::pin(async move {
            let start_time = Local::now().naive_local();
            let should_log_request = should_log(&method, &uri);
            // 记录 Content-Type 中的 multipart 标志，用于脱敏时跳过 body 记录
            let is_multipart = req
                .headers()
                .get("Content-Type")
                .and_then(|v| v.to_str().ok())
                .map(|v| v.to_lowercase().contains("multipart/"))
                .unwrap_or(false);
            // 需要记录时，把请求体 tee 一份到有界缓冲（仅保留前 64KB 用于日志），
            // 完整 body 原样透传给下游 handler。修复未认证超大请求体被无限缓冲的内存 DoS。
            let log_buffer: Option<Arc<Mutex<BytesMut>>> = if should_log_request {
                let buffer = Arc::new(Mutex::new(BytesMut::new()));
                let tee = Arc::clone(&buffer);
                let payload = req.take_payload();
                let stream = payload.map(move |chunk| {
                    if let Ok(bytes) = &chunk {
                        let mut buf = tee.lock();
                        if buf.len() < BODY_LOG_LIMIT {
                            let room = BODY_LOG_LIMIT - buf.len();
                            buf.extend_from_slice(&bytes[..bytes.len().min(room)]);
                        }
                    }
                    chunk
                });
                req.set_payload(Payload::from(Box::pin(stream) as BoxedPayloadStream));
                Some(buffer)
            } else {
                None
            };

            let res = service.call(req).await?;

            if should_log_request {
                if let Some(app_state) = app_state {
                    let end_time = Local::now().naive_local();
                    let duration = end_time.signed_duration_since(start_time);
                    let times = duration.num_milliseconds() as i32;

                    let username = auth_username
                        .clone()
                        .unwrap_or_else(|| "Unknown".to_string());

                    let user_agent = UserAgent::parse_user_agent(&user_agent_str).await;
                    let ip_source = IpRegion::search_by_ip::<&str>(&ip).unwrap_or_default();

                    let body_bytes = log_buffer
                        .map(|buf| {
                            let guard = buf.lock();
                            Bytes::copy_from_slice(&guard[..])
                        })
                        .unwrap_or_default();
                    let param = build_param(&query_string, &body_bytes, is_multipart);
                    let description = resolve_description(&method, &uri);

                    let db = app_state.get_mysql_pool();
                    OperationLogService::save_operation_log(
                        &db,
                        username,
                        uri.clone(),
                        method.to_string(),
                        param,
                        description,
                        Some(ip),
                        Some(ip_source),
                        Some(user_agent.os.name),
                        Some(user_agent.browser.name),
                        times,
                        end_time,
                        Some(user_agent.user_agent),
                    )
                    .await
                    .unwrap_or_else(|e| tracing::error!("保存操作日志失败{e}"));
                }
            }

            Ok(res)
        })
    }
}

/// 日志请求体保留上限（64KB 远超 build_param 的 1900 字符截断，仅用于防止无限缓冲）
const BODY_LOG_LIMIT: usize = 64 * 1024;

fn should_log(method: &Method, uri: &str) -> bool {
    if method == Method::OPTIONS || method == Method::GET {
        return false;
    }
    if !uri.starts_with("/admin") {
        return false;
    }
    if uri == "/admin/login"
        || uri.ends_with("/login")
        || uri == "/admin/operationLogs"
        || uri == "/admin/operationLog"
    {
        return false;
    }
    true
}

fn build_param(query: &str, body: &Bytes, is_multipart: bool) -> Option<String> {
    let query = query.trim();
    // multipart 请求（如文件上传）body 为二进制/敏感内容，只记录 query
    let body_str = if is_multipart {
        String::new()
    } else {
        String::from_utf8_lossy(body).trim().to_string()
    };

    let param = match (query.is_empty(), body_str.is_empty()) {
        (true, true) => return None,
        (false, true) => sanitize_plain(&query),
        (true, false) => sanitize_body(&body_str),
        (false, false) => {
            let value = serde_json::json!({
                "query": sanitize_plain(&query),
                "body": sanitize_body(&body_str),
            });
            value.to_string()
        }
    };

    Some(truncate_param(param, 1900))
}

/// 对单段文本做脱敏：先尝试按 JSON 解析递归处理，否则用查询串正则替换敏感字段值
fn sanitize_body(body: &str) -> String {
    if let Ok(value) = serde_json::from_str::<serde_json::Value>(body) {
        let value = redact_json(&value);
        return value.to_string();
    }
    sanitize_plain(body)
}

/// 对 a=b&c=d 形式的查询串（或非 JSON 文本）做敏感字段值替换
fn sanitize_plain(text: &str) -> String {
    let re = regex::Regex::new(r"(?i)(password|token|secret|authorization)=([^&\s]+)").unwrap();
    re.replace_all(text, "$1=***").into_owned()
}

/// 递归地把 key 命中敏感关键字的值替换为 "***"
fn redact_json(v: &serde_json::Value) -> serde_json::Value {
    let sensitive =
        regex::Regex::new(r"(?i)^(.*password.*|.*token.*|.*secret.*|.*authorization.*|.*auth.*)$")
            .unwrap();
    match v {
        serde_json::Value::Object(map) => {
            let mut out = serde_json::Map::new();
            for (k, val) in map {
                if sensitive.is_match(k) {
                    out.insert(k.clone(), serde_json::Value::String("***".to_string()));
                } else {
                    out.insert(k.clone(), redact_json(val));
                }
            }
            serde_json::Value::Object(out)
        }
        serde_json::Value::Array(arr) => {
            serde_json::Value::Array(arr.iter().map(redact_json).collect())
        }
        other => other.clone(),
    }
}

fn truncate_param(value: String, max_len: usize) -> String {
    if value.len() <= max_len {
        return value;
    }
    let mut trimmed = value;
    let mut truncate_at = max_len;
    while !trimmed.is_char_boundary(truncate_at) {
        truncate_at -= 1;
    }
    trimmed.truncate(truncate_at);
    trimmed.push_str("...");
    trimmed
}

fn resolve_description(method: &Method, uri: &str) -> Option<String> {
    let description = match uri {
        "/admin/blog" if method == Method::POST => "发布博客",
        "/admin/blog" if method == Method::PUT => "更新博客",
        "/admin/blog" if method == Method::DELETE => "删除博客",
        "/admin/blog/recommend" if method == Method::PUT => "更新博客推荐状态",
        "/admin/blog/top" if method == Method::PUT => "更新博客置顶状态",
        "/admin/moment" if method == Method::POST || method == Method::PUT => "更新动态",
        "/admin/moment" if method == Method::DELETE => "删除动态",
        "/admin/moment/published" if method == Method::PUT => "更新动态发布状态",
        "/admin/category" if method == Method::PUT => "更新分类",
        "/admin/category" if method == Method::DELETE => "删除分类",
        "/admin/category" if method == Method::POST => "新增分类",
        "/admin/tag" if method == Method::POST => "新增标签",
        "/admin/tag" if method == Method::PUT => "更新标签",
        "/admin/tag" if method == Method::DELETE => "删除标签",
        "/admin/comment" if method == Method::PUT => "更新评论",
        "/admin/comment" if method == Method::DELETE => "删除评论",
        "/admin/comment/published" if method == Method::PUT => "更新评论发布状态",
        "/admin/comment/notice" if method == Method::PUT => "更新评论通知状态",
        "/admin/friend" if method == Method::POST => "新增友链",
        "/admin/friend" if method == Method::PUT => "更新友链",
        "/admin/friend" if method == Method::DELETE => "删除友链",
        "/admin/friend/published" if method == Method::PUT => "更新友链发布状态",
        "/admin/friendInfo/commentEnabled" if method == Method::PUT => "更新友链评论状态",
        "/admin/friendInfo/content" if method == Method::PUT => "更新友链内容",
        "/admin/job" if method == Method::POST => "新增定时任务",
        "/admin/job" if method == Method::PUT => "更新定时任务",
        "/admin/job" if method == Method::DELETE => "删除定时任务",
        "/admin/job/status" if method == Method::PUT => "更新定时任务状态",
        "/admin/job/run" if method == Method::POST => "执行定时任务",
        "/admin/job/log" if method == Method::DELETE => "删除定时任务日志",
        "/admin/siteSettings" if method == Method::POST => "更新站点设置",
        "/admin/about" if method == Method::PUT || method == Method::POST => "更新关于我",
        "/admin/account" if method == Method::POST => "更新账号信息",
        "/admin/visitor" if method == Method::DELETE => "删除访客",
        "/admin/visitLog" if method == Method::DELETE => "删除访问日志",
        "/admin/loginLog" if method == Method::DELETE => "删除登录日志",
        "/admin/exceptionLog" if method == Method::DELETE => "删除异常日志",
        "/admin/operationLog" if method == Method::DELETE => "删除操作日志",
        _ => {
            if uri.starts_with("/admin/blog/")
                && uri.ends_with("/visibility")
                && method == Method::PUT
            {
                "更新博客可见性状态"
            } else {
                return None;
            }
        }
    };

    Some(description.to_string())
}

#[cfg(test)]
mod tests {
    use super::truncate_param;

    #[test]
    fn truncate_param_handles_multibyte_boundary() {
        let value = format!("{}你", "a".repeat(1899));
        let truncated = truncate_param(value, 1900);

        assert_eq!(truncated, format!("{}...", "a".repeat(1899)));
    }
}
