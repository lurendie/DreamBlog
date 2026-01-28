/*
 * @Author: lurendie
 * @Date: 2024-02-24 22:58:03
 * @LastEditors: lurendie
 * @LastEditTime: 2024-05-17 12:18:04
 */
use crate::error::DataBaseError;
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::{env, fs, sync::LazyLock};

//配置文件结构体
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    server: ServerConfig,
    mysql: MysqlConfig, //Mysql链接
    redis: RedisConfig, //Redis
    log: Option<LogConfig>,
    email: EmailConfig,
}
/**
 * Redis 连接信息结构体
 */
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RedisConfig {
    pub(crate) port: u16,    //端口
    pub(crate) host: String, //IP地址
    pub(crate) db: u16,
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) ttl: i64,
}
/**
 * MySQL 配置信息结构体
 */
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MysqlConfig {
    pub(crate) port: u16,    //端口
    pub(crate) host: String, //IP地址
    pub(crate) data_base: String,
    pub(crate) user_name: String,
    pub(crate) password: String,
}
/**
 * Server 配置信息结构体
 */
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ServerConfig {
    pub(crate) port: u16,          //端口
    pub(crate) host: String,       //IP地址
    pub(crate) view_url: String,   //前端页面地址
    pub(crate) cms_url: String,    //前端页面地址
    pub(crate) token_expires: i64, //token 过期时间
}

pub static CONFIG: LazyLock<AppConfig> = LazyLock::new(|| {
    // 尝试获取 配置目录 命令行参数：如没有指定则默认 ./config
    let config_dir = env::args().nth(1).unwrap_or_else(|| "./config".to_string());
    let config_dir = PathBuf::from(config_dir);

    // 加载配置：优先使用本地覆盖文件（不应提交到 git）
    let local_config_path = config_dir.join("app_config.local.yaml");
    let default_config_path = config_dir.join("app_config.yaml");
    let app_config_path = if local_config_path.exists() {
        local_config_path
    } else {
        default_config_path
    };

    let log_yaml_path = config_dir.join("log_config.yaml");

    match AppConfig::build_config(&app_config_path) {
        Ok(mut config) => {
            let log_config =
                LogConfig::init_path(log_yaml_path.to_string_lossy().into_owned()).unwrap();
            config.log = Some(log_config);
            log::info!(
                "Loaded config from: {}",
                app_config_path.to_string_lossy().into_owned()
            );
            config
        }
        Err(e) => {
            panic!("{e}")
        }
    }
});

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct LogConfig;
impl LogConfig {
    // pub fn new() -> Self {
    //     Self::default()
    // }

    pub fn init_path(path: String) -> Result<Self, log4rs::config::InitError> {
        let _ = log4rs::init_file(path, Default::default())
            .expect("初始化日志配置失败，请检查 log_config.yaml 配置文件是否正确！");
        log::info!("Blog API初始化完成, 时间为:[{}]...", Self::get_date_time());
        //修改日志等级ERROR 非ERROR日志不记录
        //log::set_max_level(log::LevelFilter::Error.to_level().unwrap().to_level_filter());
        Ok(Self)
    }

    pub const FMT_Y_M_D_H_M_S: &str = "%Y-%m-%d %H:%M:%S";

    pub fn get_date_time() -> String {
        let date_time = Local::now().naive_local();
        date_time.format(Self::FMT_Y_M_D_H_M_S).to_string()
    }
}

impl AppConfig {
    pub fn get_mysql_config(&self) -> MysqlConfig {
        self.mysql.clone()
    }

    pub fn get_redis_config(&self) -> RedisConfig {
        self.redis.clone()
    }

    pub fn get_server_config(&self) -> ServerConfig {
        self.server.clone()
    }

    pub fn get_email_config(&self) -> EmailConfig {
        self.email.clone()
    }

    fn build_config(path: &Path) -> Result<AppConfig, DataBaseError> {
        let yaml_str = match fs::read_to_string(path) {
            Ok(str) => str,
            Err(_) => {
                return Err(DataBaseError::Custom(format!(
                    "无法从路径:{:?} 中加载配置，请检查！",
                    path.display()
                )));
            }
        };
        Ok(serde_yaml::from_str::<AppConfig>(&yaml_str)?)
    }
}

/**
 * 获取配置信息
 */
pub fn _get_app_config() -> AppConfig {
    CONFIG.clone()
}
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct EmailConfig {
    pub(crate) host: String,
    pub(crate) port: u16,
    pub(crate) username: String,
    pub(crate) password: String,
}
