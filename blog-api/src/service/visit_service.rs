use std::collections::HashMap;

use crate::error::DataBaseError;
use crate::model::{VisitLog, VisitLogQuery};
use crate::{
    app::AppState,
    common::{IpRegion, UserAgentInfo},
    constant::{VisitBehavior, VisitBehaviorType},
    entity::visit_log,
    service::BlogService,
};
use actix_web::web::{Data, Query};
use sea_orm::{ActiveModelTrait, ActiveValue::Set};
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
};

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
        db: &DatabaseConnection,
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
        Ok(())
    }

    pub async fn delete_by_id(db: &DatabaseConnection, id: i64) -> Result<(), DataBaseError> {
        visit_log::Entity::delete_by_id(id).exec(db).await?;
        Ok(())
    }

    pub async fn get_visit_log_list(
        query: VisitLogQuery,
        db: &DatabaseConnection,
        page_num: i64,
        page_size: i64,
    ) -> Result<(Vec<VisitLog>,u64), DataBaseError> {
        // 构建查询条件
        let mut query_builder = visit_log::Entity::find();

        if let Some(uri) = &query.uri {
            query_builder = query_builder.filter(visit_log::Column::Uri.contains(uri));
        }

        if let Some(ip) = &query.ip {
            query_builder = query_builder.filter(visit_log::Column::Ip.contains(ip));
        }

        if let Some(behavior) = &query.behavior {
            query_builder = query_builder.filter(visit_log::Column::Behavior.contains(behavior));
        }

        // 获取分页数据
        let paginator = query_builder
            .order_by_desc(visit_log::Column::Id)
            .paginate(db, page_size as u64);

        let total = paginator.num_items().await.unwrap_or(0);
        let log_models = paginator.fetch_page((page_num - 1) as u64).await?;
        
        let mut logs = vec![];
        log_models.into_iter().for_each(|item| {
            logs.push(VisitLog::from(item));
        });
        Ok((logs, total))
    }
}
