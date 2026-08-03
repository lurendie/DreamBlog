/*
 * 异常日志中间件：记录未被转换为响应的服务端错误（HTTP 5xx 级别）。
 * 业务错误（参数校验、404 等）以 200 + body code 返回，不属于异常日志范畴。
 */
use std::future::{ready, Future, Ready};
use std::pin::Pin;
use std::rc::Rc;

use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::web::Data;
use actix_web::Error;
use chrono::Local;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;

use crate::app::AppState;
use crate::common::{IpRegion, UserAgent};
use crate::entity::exception_log;

#[derive(Default, Debug)]
pub struct ExceptionLog;

impl<S, B> Transform<S, ServiceRequest> for ExceptionLog
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = ExceptionLogMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ExceptionLogMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct ExceptionLogMiddleware<S> {
    service: Rc<S>,
}

type LocalBoxFuture<T> = Pin<Box<dyn Future<Output = T> + 'static>>;

impl<S, B> Service<ServiceRequest> for ExceptionLogMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let method = req.method().to_string();
        let uri = req.uri().path().to_string();
        let query = req.query_string().to_string();
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
        let service = self.service.clone();

        Box::pin(async move {
            let res = service.call(req).await;
            // 仅记录 5xx 服务端错误，避免无效 token、404 等常见请求刷屏
            if let Err(e) = &res {
                if e.as_response_error().status_code().is_server_error() {
                    if let Some(app_state) = app_state {
                        let user_agent = UserAgent::parse_user_agent(&user_agent_str).await;
                        let param = if query.is_empty() {
                            None
                        } else {
                            Some(query.clone())
                        };
                        let model = exception_log::ActiveModel {
                            uri: Set(uri.clone()),
                            method: Set(method.clone()),
                            param: Set(param),
                            description: Set(None),
                            error: Set(Some(e.to_string())),
                            ip: Set(Some(ip.clone())),
                            ip_source: Set(Some(
                                IpRegion::search_by_ip::<&str>(&ip).unwrap_or_default(),
                            )),
                            os: Set(Some(user_agent.os.name)),
                            browser: Set(Some(user_agent.browser.name)),
                            create_time: Set(Local::now().naive_local()),
                            user_agent: Set(Some(user_agent.user_agent)),
                            ..Default::default()
                        };
                        let db = app_state.get_mysql_pool();
                        if let Err(save_err) = model.save(db).await {
                            log::error!("保存异常日志失败: {save_err}");
                        }
                    }
                }
            }
            res
        })
    }
}
