use std::collections::HashMap;

use crate::constant::RedisKeyConstant;
use crate::error::DataBaseError;
use crate::model::Visitor;
use crate::service::VisitorService;
use crate::{
    app::AppState,
    common::{IpRegion, UserAgentInfo},
    constant::{VisitBehavior, VisitBehaviorType},
    entity::visit_log,
    service::{BlogService, RedisService},
};
use actix_web::web::{Data, Query};
use chrono::Local;
use sea_orm::{ActiveModelTrait, ActiveValue::Set};

pub struct VisitService;

impl VisitService {
    /***
     * 获取访问行为及请求参数
     */
    pub async fn get_behavior(
        path: &VisitBehaviorType,
        parameter: &Query<HashMap<String, String>>,
        app_state: &Option<Data<AppState>>,
    ) -> (VisitBehavior, HashMap<String, String>) {
        let mut map = HashMap::new();
        let behavior = {
            match path {
                VisitBehaviorType::INDEX => {
                    let mut behavior = VisitBehavior::from(VisitBehaviorType::INDEX);
                    if let Some((key, value)) = parameter.0.get_key_value("pageNum") {
                        map.insert(key.to_string(), value.to_string());

                        behavior.set_remark(format!("第{value}页"));
                    }
                    if let Some((key, value)) = parameter.0.get_key_value("pageSize") {
                        map.insert(key.to_string(), value.to_string());
                    }
                    behavior
                }
                VisitBehaviorType::ARCHIVE => VisitBehavior::from(VisitBehaviorType::ARCHIVE),
                VisitBehaviorType::MOMENT => VisitBehavior::from(VisitBehaviorType::MOMENT),
                VisitBehaviorType::FRIEND => VisitBehavior::from(VisitBehaviorType::FRIEND),
                VisitBehaviorType::ABOUT => VisitBehavior::from(VisitBehaviorType::ABOUT),
                VisitBehaviorType::CATEGORY => {
                    let mut behavior = VisitBehavior::from(VisitBehaviorType::CATEGORY);
                    if let Some((key, value)) = parameter.0.get_key_value("categoryName") {
                        map.insert(key.to_string(), value.to_string());
                        behavior.set_content(value.to_string());
                        if let Some(page_num) = parameter.0.get("pageNum") {
                            behavior.set_remark(format!("分类名称：{value},第{page_num}页"));
                        } else {
                            behavior.set_remark(format!("分类名称：{value},第1页"));
                        };
                    }
                    behavior
                }
                VisitBehaviorType::TAG => {
                    let mut behavior = VisitBehavior::from(VisitBehaviorType::TAG);
                    if let Some((key, value)) = parameter.0.get_key_value("tagName") {
                        map.insert(key.to_string(), value.to_string());
                        behavior.set_content(value.to_string());
                        if let Some(page_num) = parameter.0.get("pageNum") {
                            behavior.set_remark(format!("标签名称：{value},第{page_num}页"));
                        } else {
                            behavior.set_remark(format!("标签名称：{value},第1页"));
                        };
                    }
                    behavior
                }
                VisitBehaviorType::BLOG => {
                    let mut behavior = VisitBehavior::from(VisitBehaviorType::BLOG);
                    if let Some(id) = parameter.0.get("id") {
                        if let Some(app) = app_state.as_ref() {
                            let blog = BlogService::find_blog_id_and_title(
                                app.get_mysql_pool(),
                                id.parse().unwrap_or(0),
                            )
                            .await
                            .unwrap_or_default();
                            map.insert("id".to_string(), id.to_string());
                            behavior.set_remark(format!("文章标题：{:?}", blog.title));
                            behavior.set_content(blog.title);
                        }
                    }
                    behavior
                }
                VisitBehaviorType::SEARCH => {
                    let mut behavior = VisitBehavior::from(VisitBehaviorType::SEARCH);
                    if let Some((key, value)) = parameter.0.get_key_value("query") {
                        map.insert(key.to_string(), value.to_string());
                        behavior.set_content(value.to_string());
                        behavior.set_remark(format!("搜索内容：{value}"));
                    }
                    behavior
                }
                VisitBehaviorType::ClickFriend => {
                    let mut behavior = VisitBehavior::from(VisitBehaviorType::ClickFriend);
                    if let Some((key, value)) = parameter.0.get_key_value("nickname") {
                        map.insert(key.to_string(), value.to_string());
                        behavior.set_content(value.to_string());
                        behavior.set_remark(format!("友链名称：{value}"));
                    }
                    VisitBehavior::from(VisitBehaviorType::ClickFriend)
                }

                VisitBehaviorType::LikeMoment => VisitBehavior::from(VisitBehaviorType::LikeMoment),
                VisitBehaviorType::CheckPassword => {
                    VisitBehavior::from(VisitBehaviorType::CheckPassword)
                }
                _ => VisitBehavior::from(VisitBehaviorType::UNKNOWN),
            }
        };
        (behavior, map)
    }

    pub async fn save_visit(
        app_state: &Option<Data<AppState>>,
        visitor_uuid: &str,
        uri: &str,
        method: &str,
        param: &str,
        ip: &str,
        user_agent: UserAgentInfo,
        times: i32,
        end_time: chrono::NaiveDateTime,
        visit_behavior: VisitBehavior,
    ) -> Result<(), DataBaseError> {
        if let Some(app) = app_state.as_ref() {
            let db = app.get_mysql_pool();
            // 记录访问日志
            let new_visit_log = visit_log::ActiveModel {
                uuid: Set(Some(visitor_uuid.to_string())),
                uri: Set(uri.to_string()),
                method: Set(method.to_string()),
                param: Set(param.to_string()),
                ip: Set(Some(ip.to_string())),
                ip_source: Set(Some(
                    IpRegion::search_by_ip::<&str>(&ip).unwrap_or_default(),
                )),
                os: Set(Some(user_agent.os.name.to_string())),
                browser: Set(Some(user_agent.browser.name.to_string())),
                times: Set(times),
                create_time: Set(end_time),
                user_agent: Set(Some(user_agent.user_agent.to_string())),
                behavior: Set(Some(visit_behavior.get_behavior().to_string())),
                content: Set(Some(visit_behavior.get_content().to_string())),
                remark: Set(Some(visit_behavior.get_remark().to_string())),
                ..Default::default()
            };
            if let Err(e) = new_visit_log.save(db).await {
                log::error!("保存访问日志失败: {}", e);
            }
            //记录访客Redis
            match RedisService::get_hash_key::<String>(
                RedisKeyConstant::IDENTIFICATION_SET.to_string(),
                visitor_uuid.to_string(),
            )
            .await
            {
                Ok(pv) => {
                    let mut pv = pv.parse::<u32>().unwrap_or_else(|e| {
                        log::error!("pv转换失败:{}", e);
                        0
                    });
                    pv += 1;
                    RedisService::set_hash_key::<u32>(
                        RedisKeyConstant::IDENTIFICATION_SET.to_string(),
                        visitor_uuid.to_string(),
                        &pv,
                    )
                    .await?;
                }
                Err(e) => {
                    //查询UUID是否存在
                    log::warn!("缓存中UUID不存在:{e},尝试从数据库中查询");
                    let pv = match VisitorService::get_by_uuid(visitor_uuid, db).await {
                        Some(mut visitor) => {
                            let pv = visitor.pv.unwrap_or(0) + 1;
                            visitor.pv = Some(pv);
                            visitor.last_time = Local::now().naive_local();
                            VisitorService::save_visitor(visitor, db).await?;
                            pv
                        }
                        None => {
                            let visitor = Visitor::new(
                                0,
                                visitor_uuid.to_string(),
                                Some(ip.to_string()),
                                Some(IpRegion::search_by_ip::<&str>(&ip).unwrap_or_default()),
                                Some(user_agent.os.name.to_string()),
                                Some(user_agent.browser.name.to_string()),
                                Local::now().naive_local(),
                                Local::now().naive_local(),
                                Some(1),
                                Some(user_agent.user_agent.to_string()),
                            );
                            VisitorService::save_visitor(visitor, db).await?;
                            0
                        }
                    };

                    RedisService::set_hash_key::<u32>(
                        RedisKeyConstant::IDENTIFICATION_SET.to_string(),
                        visitor_uuid.to_string(),
                        &(pv as u32),
                    )
                    .await?;
                }
            }
        } else {
            log::error!("保存访问日志失败: AppState is None");
        }
        Ok(())
    }
}
