/*
 * @Author: lurendie 
 * @Date: 2024-03-26 00:08:12
 * @LastEditors: lurendie
 * @LastEditTime: 2024-04-26 00:07:05
 * @FilePath: \zero_blog\src\log4rs.rs
 */
use chrono::Local;
#[derive(Default)]
pub struct Log4rs;
impl Log4rs {
    pub fn new() -> Log4rs {
        Log4rs::default().init()
    }
    fn init(self) -> Self {
        let _ = log4rs::init_file("./config/log4rs.yaml", Default::default()).unwrap();
        log::info!("开始加载项目, 时间为:[{}]...", get_date_time());
        //修改日志等级ERROR 非ERROR日志不记录
        //log::set_max_level(log::LevelFilter::Error.to_level().unwrap().to_level_filter());
        self
    }
}

pub const FMT_Y_M_D_H_M_S: &str = "%Y-%m-%d %H:%M:%S";

pub fn get_date_time() -> String {
    let date_time = Local::now();
    date_time.format(FMT_Y_M_D_H_M_S).to_string()
}
