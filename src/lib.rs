//配置项
pub mod config;
//Rbatis
pub mod rbatis;
//模型
pub mod models;
// 应用
pub mod app;
//常量
mod constant;
//控制
pub mod controller;
//数据访问
mod dao;
//枚举
mod enums;
//日志
pub mod log4rs;
//服务
pub mod service;
//工具
mod utils;
//redis
mod redis;
//中间件
mod middleware;

pub use config::CONFIG;
