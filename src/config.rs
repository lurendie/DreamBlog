/*
 * @Author: lurendie
 * @Date: 2024-02-24 22:58:03
 * @LastEditors: lurendie
 * @LastEditTime: 2024-05-17 12:18:04
 */
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::fs;
//配置文件结构体
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub server: Server,
    pub mysql: MysqlCon, //Mysql链接
    pub redis: ReidsCon, //Redis
}
/**
 * Redis 连接信息结构体
 */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ReidsCon {
    pub port: u16,    //端口
    pub host: String, //IP地址
    pub db: u16,
    pub username: String,
    pub password: String,
    pub ttl: String,
}
/**
 * MySQL 配置信息结构体
 */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MysqlCon {
    pub port: u16,    //端口
    pub host: String, //IP地址
    pub data_base: String,
    pub user_name: String,
    pub password: String,
}
/**
 * Server 配置信息结构体
 */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Server {
    pub port: u16,             //端口
    pub host: String,          //IP地址
    pub front_adderss: String, //前端页面地址
}
pub static CONFIG: OnceCell<Config> = OnceCell::new();

pub fn default() -> &'static Config {
    CONFIG.get_or_init(|| {
        let yaml_str =
            fs::read_to_string("./config/config.yaml").expect("Failed to read config.yaml");
        let config: Config = serde_yaml::from_str(&yaml_str).expect("Failed to parse config.yaml");
        config
    })
}
