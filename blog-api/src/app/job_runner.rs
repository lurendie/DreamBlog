use std::collections::HashMap;
use std::sync::Arc;

use chrono::{DateTime, Utc};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use tokio::sync::Mutex;
use tokio::time::{interval, Duration};

use crate::app::AppState;
use crate::entity::schedule_job;
use crate::service::ScheduleJobService;

pub struct JobRunner;

impl JobRunner {
    pub async fn start(app_state: AppState) {
        let last_runs: Arc<Mutex<HashMap<i64, DateTime<Utc>>>> =
            Arc::new(Mutex::new(HashMap::new()));
        let mut ticker = interval(Duration::from_secs(30));

        loop {
            ticker.tick().await;
            let db = app_state.get_mysql_pool();
            let jobs = schedule_job::Entity::find()
                .filter(schedule_job::Column::Status.eq(true))
                .all(db)
                .await;

            let jobs = match jobs {
                Ok(jobs) => jobs,
                Err(e) => {
                    log::error!("定时任务获取失败: {}", e);
                    continue;
                }
            };

            for job in jobs {
                let cron = job.cron.clone().unwrap_or_default();
                if cron.trim().is_empty() {
                    continue;
                }

                let last_run = {
                    let map = last_runs.lock().await;
                    map.get(&job.job_id)
                        .copied()
                        .unwrap_or_else(|| DateTime::<Utc>::from(std::time::UNIX_EPOCH))
                };

                let next_run = match ScheduleJobService::next_run_after(&cron, last_run) {
                    Ok(value) => value,
                    Err(e) => {
                        log::error!("定时任务Cron解析失败 job_id={}: {}", job.job_id, e);
                        continue;
                    }
                };
                let Some(next_run) = next_run else { continue };
                let now = Utc::now();
                if next_run > now {
                    continue;
                }

                let job_id = job.job_id;
                if let Err(e) = ScheduleJobService::execute_job_model(job, db).await {
                    log::error!("定时任务执行失败: {}", e);
                }

                let mut map = last_runs.lock().await;
                map.insert(job_id, now);
            }
        }
    }
}
