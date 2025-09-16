/*
* @Author: lurendie
* @Date: 2024-04-30 00:04:06
 * @LastEditors: lurendie
 * @LastEditTime: 2024-05-15 19:10:17

*/
use std::{
    future::{ready, Future, Ready},
    pin::Pin,
    str::FromStr,
    sync::LazyLock,
};

use crate::app_state::AppState;
use crate::entity::visit_log;
use actix_jwt_session::Uuid;
use actix_web;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::header::{HeaderName, HeaderValue},
    Error,
};
use chrono::Local;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;
use user_agent_parser::UserAgentParser;
// 全局UserAgent解析器
static USER_AGENT_PARSER: LazyLock<UserAgentParser> = LazyLock::new(|| {
    // 尝试从文件加载，如果失败则使用默认解析器
    match std::fs::read_to_string("./data/regexes.yaml") {
        Ok(content) => UserAgentParser::from_str(&content).unwrap_or_else(|_| {
            log::warn!("无法解析UserAgent正则表达式文件");
            panic!("无法解析UserAgent正则表达式文件")
        }),
        Err(_) => {
            log::warn!("找不到UserAgent正则表达式文件");
            panic!("无法解析UserAgent正则表达式文件")
        }
    }
});

/**
 * 校验访客标识码并记录访问日志
 */
#[derive(Default, Debug)]
pub struct VisiLog;

impl<S, B> Transform<S, ServiceRequest> for VisiLog
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = VisitLogMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(VisitLogMiddleware { service }))
    }
}

pub struct VisitLogMiddleware<S> {
    /// The next service to call
    service: S,
}

// This future doesn't have the requirement of being `Send`.
// See: futures_util::future::LocalBoxFuture
type LocalBoxFuture<T> = Pin<Box<dyn Future<Output = T> + 'static>>;

// `S`: type of the wrapped service
// `B`: type of the body - try to be generic over the body where possible
impl<S, B> Service<ServiceRequest> for VisitLogMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<Result<Self::Response, Self::Error>>;

    // This service is ready when its next service is ready
    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // 在调用服务之前获取所有需要的信息
        let method = req.method().to_string();
        let uri = req.uri().to_string();
        let user_agent = req
            .headers()
            .get("User-Agent")
            .and_then(|h| h.to_str().ok())
            .unwrap_or("")
            .to_string();
        let ip = get_real_client_ip(&req);
        let params = req.query_string().to_string();
        let app_state = req.app_data::<AppState>().cloned();

        let fut = self.service.call(req);

        Box::pin(async move {
            // 记录请求开始时间
            let start_time = Local::now().naive_local();

            // 调用下一个服务
            let mut res: ServiceResponse<B> = fut.await?;

            // 如果不是admin路径，记录访问日志
            if !(res.request().uri().path().to_string().contains("admin")) {
                let uuid = Uuid::new_v4();
                let uuid_str = uuid.to_string();

                //1.检测访客标识码是否存在
                let req_headers = res.request().headers();
                let identification = req_headers.get("Identification");
                let visitor_uuid = if let Some(uuid) = identification {
                    log::info!("访客UUID:{:?}", uuid);
                    uuid.to_str().unwrap_or("").to_string()
                } else {
                    let resp = res.response_mut();
                    let resp_headers = resp.headers_mut();
                    //添加访客标识码UUID至响应头
                    resp_headers.insert(
                        HeaderName::from_str("Identification").unwrap(),
                        HeaderValue::from_str(uuid_str.as_str()).unwrap(),
                    );
                    resp_headers.insert(
                        HeaderName::from_str("access-control-expose-headers").unwrap(),
                        HeaderValue::from_str("Identification").unwrap(),
                    );
                    uuid_str
                };

                // 解析用户代理
                let browser = match USER_AGENT_PARSER.parse_engine(&user_agent).name {
                    Some(name) => name.to_string(),
                    None => {
                        log::warn!("解析user_agent中的browser异常");
                        "未知browser".to_string()
                    }
                };

                let os = match USER_AGENT_PARSER.parse_os(&user_agent).name {
                    Some(name) => name.to_string(),
                    None => {
                        log::warn!("解析user_agent中的os异常");
                        "未知os".to_string()
                    }
                };
                // 计算请求处理时间
                let end_time = Local::now().naive_local();
                let duration = end_time.signed_duration_since(start_time);
                let times = duration.num_milliseconds() as i32;

                // 记录访问日志
                if let Some(app) = app_state {
                    let db = app.get_mysql_pool();
                    let new_visit_log = visit_log::ActiveModel {
                        uuid: Set(Some(visitor_uuid)),
                        uri: Set(uri),
                        method: Set(method),
                        param: Set(params),
                        ip: Set(Some(ip.clone())),
                        ip_source: Set(Some("request".to_string())),
                        os: Set(Some(os)),
                        browser: Set(Some(browser)),
                        times: Set(times),
                        create_time: Set(end_time),
                        user_agent: Set(Some(user_agent)),
                        ..Default::default()
                    };
                    if let Err(e) = new_visit_log.save(db).await {
                        log::error!("保存访问日志失败: {}", e);
                    };
                } else {
                    log::error!("获取app_state失败");
                }
            }
            Ok(res)
        })
    }
}

/// 获取真实的客户端IP地址，考虑代理和转发的情况
fn get_real_client_ip(req: &ServiceRequest) -> String {
    // 按优先级尝试获取IP地址
    let headers = req.headers();

    // 1. 首先检查 X-Forwarded-For 头
    if let Some(x_forwarded_for) = headers.get("X-Forwarded-For") {
        if let Ok(x_forwarded_for_str) = x_forwarded_for.to_str() {
            // X-Forwarded-For 可能包含多个IP，第一个是真实的客户端IP
            let ips: Vec<&str> = x_forwarded_for_str.split(',').collect();
            if !ips.is_empty() {
                let ip = ips[0].trim();
                if !ip.is_empty() {
                    return ip.to_string();
                }
            }
        }
    }

    // 2. 检查 X-Real-IP 头
    if let Some(x_real_ip) = headers.get("X-Real-IP") {
        if let Ok(x_real_ip_str) = x_real_ip.to_str() {
            let ip = x_real_ip_str.trim();
            if !ip.is_empty() {
                return ip.to_string();
            }
        }
    }

    // 3. 检查 Proxy-Client-IP 头
    if let Some(proxy_client_ip) = headers.get("Proxy-Client-IP") {
        if let Ok(proxy_client_ip_str) = proxy_client_ip.to_str() {
            let ip = proxy_client_ip_str.trim();
            if !ip.is_empty() {
                return ip.to_string();
            }
        }
    }

    // 4. 检查 WL-Proxy-Client-IP 头
    if let Some(wl_proxy_client_ip) = headers.get("WL-Proxy-Client-IP") {
        if let Ok(wl_proxy_client_ip_str) = wl_proxy_client_ip.to_str() {
            let ip = wl_proxy_client_ip_str.trim();
            if !ip.is_empty() {
                return ip.to_string();
            }
        }
    }

    // 5. 最后从连接信息中获取IP
    let conn_info = req.connection_info();
    // 如果都无法获取，返回unknown
    //"unknown".to_string()
    return conn_info.peer_addr().unwrap_or("unknown").to_string();
}
