use actix_web::HttpRequest;
use chrono::{Local, NaiveDateTime};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait,
    PaginatorTrait, QueryFilter, QueryOrder,
};

use crate::{
    app::CONFIG,
    common::{IpRegion, UserAgent},
    entity::login_log,
    error::DataBaseError,
    model::{LoginLog, LoginLogQuery},
};

pub struct LoginLogService;

impl LoginLogService {
    /// 记录一次登录行为（成功/失败），写日志失败不影响登录流程
    pub async fn save_login_log(
        db: &DatabaseConnection,
        username: &str,
        req: &HttpRequest,
        status: bool,
        description: String,
    ) -> Result<(), DataBaseError> {
        let ip = IpRegion::get_real_client_ip(req, CONFIG.get_server_config().trust_proxy);
        let user_agent_str = req
            .headers()
            .get("User-Agent")
            .and_then(|h| h.to_str().ok())
            .unwrap_or("")
            .to_string();
        let user_agent = UserAgent::parse_user_agent(&user_agent_str).await;
        let model = login_log::ActiveModel {
            username: Set(username.to_string()),
            ip: Set(Some(ip.clone())),
            ip_source: Set(Some(
                IpRegion::search_by_ip::<&str>(&ip).unwrap_or_default(),
            )),
            os: Set(Some(user_agent.os.name)),
            browser: Set(Some(user_agent.browser.name)),
            status: Set(Some(status)),
            description: Set(Some(description)),
            create_time: Set(Local::now().naive_local()),
            user_agent: Set(Some(user_agent.user_agent)),
            ..Default::default()
        };
        model.save(db).await?;
        Ok(())
    }

    pub async fn get_login_log_list(
        query: LoginLogQuery,
        db: &DatabaseConnection,
    ) -> Result<(Vec<LoginLog>, u64), DataBaseError> {
        let page_num = query.page_num.unwrap_or(1).max(1);
        let page_size = query.page_size.unwrap_or(10).max(1);
        let mut query_builder = login_log::Entity::find();

        if let Some(date) = query.date.as_deref() {
            if let Some((start, end)) = parse_date_range(date) {
                query_builder =
                    query_builder.filter(login_log::Column::CreateTime.between(start, end));
            }
        }

        let paginator = query_builder
            .order_by_desc(login_log::Column::Id)
            .paginate(db, page_size as u64);
        let total = paginator.num_items().await.unwrap_or(0);
        let models = paginator.fetch_page((page_num - 1) as u64).await?;

        let mut logs = vec![];
        models.into_iter().for_each(|item| {
            logs.push(LoginLog::from(item));
        });
        Ok((logs, total))
    }

    pub async fn delete_by_id(db: &DatabaseConnection, id: i64) -> Result<(), DataBaseError> {
        login_log::Entity::delete_by_id(id).exec(db).await?;
        Ok(())
    }
}

fn parse_date_range(date: &str) -> Option<(NaiveDateTime, NaiveDateTime)> {
    let mut parts = date.split(',');
    let start = parts.next()?;
    let end = parts.next()?;
    let start_dt = NaiveDateTime::parse_from_str(start, "%Y-%m-%d %H:%M:%S").ok()?;
    let end_dt = NaiveDateTime::parse_from_str(end, "%Y-%m-%d %H:%M:%S").ok()?;
    Some((start_dt, end_dt))
}
