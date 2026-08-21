use super::app_config::CONFIG;
//use deadpool_redis::Pool;
use crate::service::VisitLogWriter;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

#[derive(Clone)]
pub struct AppState {
    pub(crate) mysql_connection: DatabaseConnection,
    /// 访问日志异步写入器（请求路径只入队，后台批量落库）
    pub(crate) visit_log_writer: Option<VisitLogWriter>,
    // pub(crate) redis_connection: Pool,
    // pub(crate) config: Config,
}

impl AppState {
    pub fn new(
        mysql_connection: DatabaseConnection,
        //redis_connection: Pool,
        // config: Config,
    ) -> Self {
        Self {
            mysql_connection,
            visit_log_writer: None,
            //  redis_connection,
            // config,
        }
    }

    // pub fn get_redis_pool(&self) -> &Pool {
    //     &self.redis_connection
    // }

    pub fn get_mysql_pool(&self) -> &DatabaseConnection {
        &self.mysql_connection
    }

    // pub fn get_redis_pool(&self) -> &Pool {
    //     &self.redis_connection
    // }

    // pub fn get_config(&self) -> &Config {
    //     &self.config
    // }
}

pub async fn get_connection() -> Result<DatabaseConnection, crate::error::DataBaseError> {
    let mysql_config = CONFIG.get_mysql_config();
    let opt = ConnectOptions::new(format!(
        "mysql://{}:{}@{}:{}/{}",
        mysql_config.user_name,
        mysql_config.password,
        mysql_config.host,
        mysql_config.port,
        mysql_config.data_base
    ))
    .max_connections(100)
    .min_connections(10)
    .sqlx_logging(false)
    .to_owned();
    Database::connect(opt)
        .await
        .map_err(crate::error::DataBaseError::MySQLError)
}
