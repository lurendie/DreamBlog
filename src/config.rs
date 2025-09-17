/*
 * @Author: lurendie
 * @Date: 2024-02-24 22:58:03
 * @LastEditors: lurendie
 * @LastEditTime: 2024-05-17 12:18:04
 */
use crate::enums::DataBaseError;
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::{fs, panic, sync::LazyLock};

//配置文件结构体
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Config {
    server: ServerConfig,
    mysql: MysqlConfig, //Mysql链接
    redis: ReidsConfig, //Redis
    log: Option<LogConfig>,
}
/**
 * Redis 连接信息结构体
 */
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ReidsConfig {
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
    // let envs = env::var_os("zero_config");
    // let path = match envs {
    //     Some(yaml_path) => {
    //         let mut new = yaml_path.clone();
    //         new.push("/config.yaml");
    //         new
    //     }
    //     None => OsString::from("./config/config.yaml"),
    // };
    match Config::build_config("./config/config.yaml".to_string()) {
        Ok(mut config) => {
            config.log = Some(LogConfig::new().init());
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
    pub fn new() -> Self {
        Self::default()
    }
    pub fn init(self) -> Self {
        // let envs = env::var_os("zero_config");
        // match envs {
        //     Some(yaml_path) => {
        //         let mut new = yaml_path.clone();
        //         new.push("/log4rs.yaml");
        //         let _ = log4rs::init_file(new, Default::default()).unwrap();
        //     }
        //     None => {
        //         let _ = log4rs::init_file("./config/log4rs.yaml", Default::default()).unwrap();
        //     }
        // };
        let _ = log4rs::init_file("./config/log4rs.yaml", Default::default()).unwrap();
        //let _ = log4rs::init_file("./config/log4rs.yaml", Default::default()).unwrap();
        log::info!("日志初始化完成, 时间为:[{}]...", Self::get_date_time());
        //修改日志等级ERROR 非ERROR日志不记录
        //log::set_max_level(log::LevelFilter::Error.to_level().unwrap().to_level_filter());
        self
    }
    pub const FMT_Y_M_D_H_M_S: &str = "%Y-%m-%d %H:%M:%S";

    pub fn get_date_time() -> String {
        let date_time = Local::now();
        date_time.format(Self::FMT_Y_M_D_H_M_S).to_string()
    }
}

impl Config {
    pub fn get_mysql_config(&self) -> MysqlConfig {
        self.mysql.clone()
    }

    pub fn get_redis_config(&self) -> ReidsConfig {
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
