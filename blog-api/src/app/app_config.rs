/*
 * @Author: lurendie
 * @Date: 2024-02-24 22:58:03
 * @LastEditors: lurendie
 * @LastEditTime: 2024-05-17 12:18:04
 */
use crate::error::DataBaseError;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::{env, fs, sync::LazyLock};

//配置文件结构体
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    server: ServerConfig,
    mysql: MysqlConfig, //Mysql链接
    #[serde(default)]
    redis: Option<RedisConfig>, //Redis
    email: EmailConfig,
}
/**
 * Redis 连接信息结构体
 */
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RedisConfig {
    #[serde(default = "default_redis_enabled")]
    pub(crate) enabled: bool,
    pub(crate) port: u16,    //端口
    pub(crate) host: String, //IP地址
    pub(crate) db: u16,
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) ttl: i64,
}

fn default_redis_enabled() -> bool {
    true
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
    #[serde(default)]
    pub(crate) cors_enabled: bool, //是否开启 CORS
    #[serde(default)]
    pub(crate) trust_proxy: bool, //是否信任反向代理的转发头（X-Forwarded-For 等），仅部署在可信反代后时开启
    pub(crate) view_url: String,  //前端页面地址
    pub(crate) cms_url: String,   //前端页面地址
    pub(crate) token_expires: i64, //token 过期时间
    #[serde(default)]
    pub(crate) cookie_secure: bool, //登录 Cookie 是否仅 HTTPS 传输（生产建议 true）
}

impl ServerConfig {
    pub fn cors_origins(&self) -> Vec<String> {
        let mut origins = Vec::new();

        for origin in [&self.view_url, &self.cms_url] {
            let normalized = origin.trim().trim_end_matches('/').to_string();
            if !normalized.is_empty() && !origins.iter().any(|item| item == &normalized) {
                origins.push(normalized);
            }
        }

        origins
    }
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

    match AppConfig::build_config(&app_config_path) {
        Ok(config) => {
            tracing::info!(
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

impl AppConfig {
    pub fn get_mysql_config(&self) -> MysqlConfig {
        self.mysql.clone()
    }

    pub fn get_redis_config(&self) -> Option<RedisConfig> {
        self.redis.clone().filter(|config| config.enabled)
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
