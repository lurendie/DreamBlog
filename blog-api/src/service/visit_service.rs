use std::collections::HashMap;

use actix_web::web::{Data, Query};
use sea_orm::{ActiveModelTrait, ActiveValue::Set};

use crate::{
    app::AppState,
    common::{IpRegion, UserAgentInfo},
    constant::{VisitBehavior, VisitBehaviorType},
    entity::visit_log,
    service::BlogService,
};

pub struct VisitService;

impl VisitService {
    /***
     * 获取访问行为及请求参数
     */
    pub async fn get_behavior(
        path: &str,
        parameter: &Query<HashMap<String, String>>,
        app_state: &Option<Data<AppState>>,
    ) -> (VisitBehavior, HashMap<String, String>) {
        let mut map = HashMap::new();
        let behavior = {
            match path {
                "/blogs" => {
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
                "/archives" => VisitBehavior::from(VisitBehaviorType::ARCHIVE),
                "/moments" => VisitBehavior::from(VisitBehaviorType::MOMENT),
                "/friends" => VisitBehavior::from(VisitBehaviorType::FRIEND),
                "/about" => VisitBehavior::from(VisitBehaviorType::ABOUT),
                "/category" => {
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
                "/tag" => {
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
                "/blog" => {
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
                "/searchBlog" => {
                    let mut behavior = VisitBehavior::from(VisitBehaviorType::SEARCH);
                    if let Some((key, value)) = parameter.0.get_key_value("query") {
                        map.insert(key.to_string(), value.to_string());
                        behavior.set_content(value.to_string());
                        behavior.set_remark(format!("搜索内容：{value}"));
                    }
                    behavior
                }
                "/friend" => {
                    let mut behavior = VisitBehavior::from(VisitBehaviorType::ClickFriend);
                    if let Some((key, value)) = parameter.0.get_key_value("nickname") {
                        map.insert(key.to_string(), value.to_string());
                        behavior.set_content(value.to_string());
                        behavior.set_remark(format!("友链名称：{value}"));
                    }
                    VisitBehavior::from(VisitBehaviorType::ClickFriend)
                }

                "/moment/like/" => VisitBehavior::from(VisitBehaviorType::LikeMoment),
                "/checkBlogPassword" => VisitBehavior::from(VisitBehaviorType::CheckPassword),
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
    ) {
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
                os: Set(Some(user_agent.os.name)),
                browser: Set(Some(user_agent.browser.name)),
                times: Set(times),
                create_time: Set(end_time),
                user_agent: Set(Some(user_agent.user_agent)),
                behavior: Set(Some(visit_behavior.get_behavior().to_string())),
                content: Set(Some(visit_behavior.get_content().to_string())),
                remark: Set(Some(visit_behavior.get_remark().to_string())),
                ..Default::default()
            };
            if let Err(e) = new_visit_log.save(db).await {
                log::error!("保存访问日志失败: {}", e);
            };
        } else {
            log::error!("保存访问日志失败: AppState is None");
        }
    }
}
