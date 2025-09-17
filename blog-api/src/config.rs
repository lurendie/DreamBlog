/*
 * @Author: lurendie
 * @Date: 2024-02-24 22:58:03
 * @LastEditors: lurendie
 * @LastEditTime: 2024-05-17 12:18:04
 */
use crate::enums::DataBaseError;
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::{env, fs, panic, sync::LazyLock};

//配置文件结构体
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Config {
    server: ServerConfig,
    mysql: MysqlConfig, //Mysql链接
    redis: RedisConfig, //Redis
    log: Option<LogConfig>,
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
    pub(crate) port: u16,             //端口
    pub(crate) host: String,          //IP地址
    pub(crate) front_adderss: String, //前端页面地址
    pub(crate) token_expires: i64,    //token 过期时间
}
pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    let args: Vec<String> = env::args().collect();
    //尝试获取 配置路径 命令行参数 如没有指定配置文件路径则默认路径是./config
    let config_path = args.get(1).unwrap_or(&"./config".to_string()).to_string();
    let mut server_config_path = config_path.clone();
    let mut log_yaml_path = config_path;
    //加载配置
    server_config_path.push_str("/server_config.yaml");
    log_yaml_path.push_str("/log_config.yaml");
    match Config::build_config(server_config_path.clone()) {
        Ok(mut config) => {
            let log_config = LogConfig::init_path(log_yaml_path).unwrap();
            config.log = Some(log_config);
            return config;
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
        log::info!("Zero Blog初始化完成, 时间为:[{}]...", Self::get_date_time());
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

impl Config {
    pub fn get_mysql_config(&self) -> MysqlConfig {
        self.mysql.clone()
    }

    pub fn get_redis_config(&self) -> RedisConfig {
        self.redis.clone()
    }

    pub fn get_server_config(&self) -> ServerConfig {
        self.server.clone()
    }

    fn build_config(path: String) -> Result<Config, DataBaseError> {
        let yaml_str = match fs::read_to_string(path.clone()) {
            Ok(str) => str,
            Err(_) => {
                return Err(DataBaseError::Custom(format!(
                    "无法从路径:{:?} 中加载配置，请检查！",
                    path
                )));
            }
        };
        Ok(serde_yaml::from_str::<Config>(&yaml_str)?)
    }
}

/**
 * 获取配置信息
 */
pub fn _get_app_config() -> Config {
    CONFIG.clone()
}
