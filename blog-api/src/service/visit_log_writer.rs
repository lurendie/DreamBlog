/*
 * 访问日志异步写入器：
 * - 请求路径只做轻量入队（mpsc），不再同步解析 UA/查询 IP 归属地/写库
 * - 后台 worker 每 5 秒或积累 200 条批量落库，避免每次访问 2~3 次同步 DB 写
 * - 队列满时 send 会等待（背压），不会无限堆积内存
 */
use chrono::NaiveDateTime;
use sea_orm::DatabaseConnection;
use tokio::sync::mpsc;
use tokio::time::{interval, Duration};

use crate::common::{IpRegion, UserAgent};
use crate::constant::VisitBehavior;
use crate::error::DataBaseError;
use crate::model::Visitor;
use crate::service::{VisitService, VisitorService};

/// 一次访问日志所需的全部原始数据（重量级解析与落库在 worker 中完成）
pub struct VisitLogEvent {
    pub visitor_uuid: String,
    pub ip: String,
    pub user_agent: String,
    pub uri: String,
    pub method: String,
    pub param: String,
    pub times: i32,
    pub end_time: NaiveDateTime,
    pub behavior: VisitBehavior,
}

#[derive(Clone)]
pub struct VisitLogWriter {
    tx: mpsc::Sender<VisitLogEvent>,
}

impl VisitLogWriter {
    /// 启动后台写入任务，返回可克隆的发送端
    pub fn start(db: DatabaseConnection) -> Self {
        let (tx, mut rx) = mpsc::channel::<VisitLogEvent>(1024);
        tokio::spawn(async move {
            let mut buffer: Vec<VisitLogEvent> = Vec::with_capacity(256);
            let mut ticker = interval(Duration::from_secs(5));
            loop {
                tokio::select! {
                    _ = ticker.tick() => {
                        if !buffer.is_empty() {
                            flush(&db, &mut buffer).await;
                        }
                    }
                    event = rx.recv() => match event {
                        Some(event) => {
                            buffer.push(event);
                            if buffer.len() >= 200 {
                                flush(&db, &mut buffer).await;
                            }
                        }
                        // 所有发送端关闭：把剩余数据刷完后退场
                        None => {
                            flush(&db, &mut buffer).await;
                            break;
                        }
                    }
                }
            }
        });
        Self { tx }
    }

    /// 将一次访问事件入队（channel 已关闭时返回错误）
    pub async fn send(&self, event: VisitLogEvent) -> Result<(), DataBaseError> {
        self.tx
            .send(event)
            .await
            .map_err(|_| DataBaseError::Custom("访问日志队列已关闭".to_string()))
    }
}

async fn flush(db: &DatabaseConnection, buffer: &mut Vec<VisitLogEvent>) {
    if buffer.is_empty() {
        return;
    }
    let events = std::mem::take(buffer);
    for event in events {
        // 重量级解析（UA 解析 + IP 归属地）与原有落库逻辑整体搬入 worker
        let user_agent = UserAgent::parse_user_agent(&event.user_agent).await;
        let visitor = Visitor::new(
            0,
            event.visitor_uuid.clone(),
            Some(event.ip.clone()),
            Some(IpRegion::search_by_ip::<&str>(&event.ip).unwrap_or_default()),
            Some(user_agent.os.name.to_string()),
            Some(user_agent.browser.name.to_string()),
            event.end_time,
            event.end_time,
            Some(1),
            Some(user_agent.user_agent.to_string()),
        );
        if let Err(e) = VisitorService::save_visitor(visitor, db).await {
            tracing::error!("保存访客失败: {e}");
        }
        if let Err(e) = VisitService::save_visit(
            db,
            &event.visitor_uuid,
            &event.uri,
            &event.method,
            &event.param,
            &event.ip,
            user_agent,
            event.times,
            event.end_time,
            event.behavior,
        )
        .await
        {
            tracing::error!("保存访问日志失败: {e}");
        }
    }
}