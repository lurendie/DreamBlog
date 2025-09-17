use crate::constant::redis_key_constants;
use crate::constant::site_setting_constants;
use crate::dao;
use crate::models::vo::{badge::Badge, copyright::Copyright, favorite::Favorite, introduction};
use crate::service::RedisService;
use rbs::to_value;
use rbs::Value;
use std::collections::HashMap;

pub struct SiteSettingService;

impl SiteSettingService {
    pub async fn get_site_info() -> HashMap<String, Value> {
        //查询缓存
        let cache_result =
            RedisService::get_value_map(redis_key_constants::SITE_INFO_MAP.to_string()).await;
        if let Some(cache_result) = cache_result {
            log::info!("key:{}数据存在", redis_key_constants::SITE_INFO_MAP);
            return cache_result;
        }

        //查询数据库
        let site_setting_list = dao::site_setting_dao::get_list().await; // 假设这是一个 Vec 或其他可迭代集合
        let mut map: HashMap<String, Value> = HashMap::new();
        let mut introduction = introduction::Introduction::new();
        let mut site_info: HashMap<String, Value> = HashMap::new();
        let mut badges = vec![];
        let mut favorites: Vec<Favorite> = vec![];
        for v in site_setting_list {
            match v.r#type {
                //类型1
                1 => {
                    if  site_setting_constants::COPYRIGHT == v.name_en{
                       let copyright:Copyright=serde_json::from_str(v.value.as_str()).unwrap();
                        site_info.insert(v.name_en, to_value!(copyright));
                    }else{
                        site_info.insert(v.name_en, Value::String(v.value));
                    }
                },
                //类型2
                2 =>
                    match v.name_en.as_str() {
                        site_setting_constants::AVATAR => introduction.avatar = v.value,
                        site_setting_constants::NAME => introduction.name = v.value,
                        site_setting_constants::GITHUB => introduction.github = v.value,
                        site_setting_constants::TELEGRAM => introduction.telegram = v.value,
                        site_setting_constants::QQ => introduction.qq = v.value,
                        site_setting_constants::BILIBILI => introduction.bilibili = v.value,
                        site_setting_constants::NETEASE => introduction.netease = v.value,
                        site_setting_constants::EMAIL => introduction.email = v.value,
                        site_setting_constants::FAVORITE => {
                            let favorite=serde_json::from_str(v.value.as_str()).unwrap();
                            favorites.push(favorite);
                        },
                        site_setting_constants::ROLL_TEXT => {
                            let arr  =v.value.split(",").map(String::from).collect();
                            introduction.roll_text=arr;
                        }
                        ,
                        _ => ()
                    }
                ,
                //类型3
                3 =>{
                    let badge:Badge=serde_json::from_str(v.value.as_str()).expect("异常");
                    badges.push(badge);
                },
                //不存在,返回单元类型
                _ => (),
            }
        }
        introduction.favorites = favorites;
        map.insert("introduction".to_string(), to_value!(introduction));
        map.insert("siteInfo".to_string(), to_value!(site_info));
        map.insert("badges".to_string(), to_value!(badges));
        //缓存数据
        log::info!("key:{}数据不存在", redis_key_constants::SITE_INFO_MAP);
        RedisService::set_value_map(redis_key_constants::SITE_INFO_MAP.to_string(), &map).await;
        map
    }
}
