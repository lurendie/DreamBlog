/*
* @Author: lurendie
* @Date: 2024-04-30 00:04:06
 * @LastEditors: lurendie
 * @LastEditTime: 2024-05-15 19:10:17

*/
use crate::app::AppState;
use crate::constant::VisitBehaviorType;
use crate::service::VisitService;
use actix_jwt_session::Uuid;
use actix_web::web;
use actix_web::{self, web::Data};
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::header::{HeaderName, HeaderValue},
    Error,
};
use chrono::Local;

use crate::common::{IpRegion, UserAgent};
use rbs::value;
use std::collections::HashMap;
use std::{
    future::{ready, Future, Ready},
    pin::Pin,
    str::FromStr,
};

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
        let uri = req.uri().path().to_string();
        let user_agent_str = req
            .headers()
            .get("User-Agent")
            .and_then(|h| h.to_str().ok())
            .unwrap_or("")
            .to_string();
        let ip = IpRegion::get_real_client_ip(&req);
        let app_state = req.app_data::<Data<AppState>>().cloned();
        let fut = self.service.call(req);
        Box::pin(async move {
            // 记录请求开始时间
            let start_time = Local::now().naive_local();
            // 调用下一个服务
            let mut res: ServiceResponse<B> = fut.await?;
            // 服务调用完成,处理访问日志
            let query = res.request().query_string();
            let visit_behavior_type = VisitBehaviorType::from(uri.as_str());
            if !(res.request().method().to_string() == "OPTIONS") {
                match visit_behavior_type {
                    VisitBehaviorType::BLOG
                    | VisitBehaviorType::CATEGORY
                    | VisitBehaviorType::ARCHIVE
                    | VisitBehaviorType::MOMENT
                    | VisitBehaviorType::FRIEND
                    | VisitBehaviorType::ClickFriend
                    | VisitBehaviorType::ABOUT
                    | VisitBehaviorType::TAG
                    | VisitBehaviorType::INDEX
                    | VisitBehaviorType::SEARCH
                    | VisitBehaviorType::LikeMoment
                    | VisitBehaviorType::CheckPassword => {
                        //获取参数
                        let parameter = web::Query::<HashMap<String, String>>::from_query(query)
                            .unwrap_or_else(|_| {
                                web::Query::<HashMap<String, String>>::from_query("blog=zero")
                                    .unwrap()
                            });
                        let (visit_behavior, map) = VisitService::get_behavior(
                            &visit_behavior_type,
                            &parameter,
                            &app_state,
                        )
                        .await;
                        let uuid = Uuid::new_v4();
                        let uuid_str = uuid.to_string();
                        //1.检测访客标识码是否存在
                        let req_headers = res.request().headers();
                        let identification = req_headers.get("Identification");
                        let visitor_uuid = if let Some(uuid) = identification {
                            uuid.to_str().unwrap_or("").to_string()
                        } else {
                            let resp_headers = res.response_mut().headers_mut();
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
                        log::info!(
                                "访客UUID:{:?} , 访问路径:{:?},访问参数:{:?}, 访问IP:{:?}, 访问行为:{:?},访问内容:{:?}",
                                &visitor_uuid,
                                uri,
                                &map,
                                &ip,
                                &visit_behavior.get_behavior(),
                                &visit_behavior.get_content()
                            );

                        // 解析用户代理
                        let user_agent = UserAgent::parse_user_agent(&user_agent_str).await;
                        // 计算请求处理时间
                        let end_time: chrono::NaiveDateTime = Local::now().naive_local();
                        let duration = end_time.signed_duration_since(start_time);
                        let times = duration.num_milliseconds() as i32;
                        let param = match map.is_empty() {
                            true => "".to_string(),
                            false => value!(map).to_string(),
                        };
                        //保存访问日志
                        VisitService::save_visit(
                            &app_state,
                            &visitor_uuid,
                            &uri,
                            &method,
                            &param,
                            &ip,
                            user_agent,
                            times,
                            end_time,
                            visit_behavior,
                        )
                        .await;
                    }
                    _ => (),
                };
            }
            Ok(res)
        })
    }
}
