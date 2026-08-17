use std::collections::{HashMap, HashSet};
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
        // 正在执行的任务集合，防止任务耗时超过轮询周期时被重复调度（重叠执行）
        let running: Arc<Mutex<HashSet<i64>>> = Arc::new(Mutex::new(HashSet::new()));
        // 轮询间隔：秒级 cron 的触发精度受此限制
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
                    tracing::error!("定时任务获取失败: {}", e);
                    continue;
                }
            };

            for job in jobs {
                let cron = job.cron.clone().unwrap_or_default();
                if cron.trim().is_empty() {
                    continue;
                }

                let job_id = job.job_id;
                // 任务正在执行则跳过本轮，避免重叠
                {
                    let mut run_set = running.lock().await;
                    if run_set.contains(&job_id) {
                        continue;
                    }
                    run_set.insert(job_id);
                }

                let last_run = {
                    let map = last_runs.lock().await;
                    map.get(&job_id)
                        .copied()
                        .unwrap_or_else(|| DateTime::<Utc>::from(std::time::UNIX_EPOCH))
                };

                let next_run = match ScheduleJobService::next_run_after(&cron, last_run) {
                    Ok(value) => value,
                    Err(e) => {
                        tracing::error!("定时任务Cron解析失败 job_id={}: {}", job_id, e);
                        running.lock().await.remove(&job_id);
                        continue;
                    }
                };
                let Some(next_run) = next_run else {
                    running.lock().await.remove(&job_id);
                    continue;
                };
                let now = Utc::now();
                if next_run > now {
                    running.lock().await.remove(&job_id);
                    continue;
                }

                let run_result = tokio::time::timeout(
                    Duration::from_secs(300),
                    ScheduleJobService::execute_job_model(job, db),
                )
                .await;
                match run_result {
                    Ok(Err(e)) => tracing::error!("定时任务执行失败: {}", e),
                    Ok(Ok(_)) => {}
                    Err(_) => {
                        tracing::error!("定时任务执行超时 job_id={}", job_id);
                    }
                }
                // 无论成功、失败还是超时，都必须从 running 集合中移除该任务，避免后续被跳过
                running.lock().await.remove(&job_id);

                let mut map = last_runs.lock().await;
                map.insert(job_id, now);
            }
        }
    }
}
