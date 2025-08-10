use crate::constant::redis_key_constants;
use crate::constant::site_setting_constants;
use crate::entity::site_setting;
use crate::enums::DataBaseError;
use crate::model::{Badge, Copyright, Favorite, Introduction};
use crate::service::RedisService;
use rbs::to_value;
use rbs::Value;
use sea_orm::DatabaseConnection;
use sea_orm::EntityTrait;
use std::collections::HashMap;

pub struct SiteSettingService;

impl SiteSettingService {
    pub async fn find_site_info(
        db: &DatabaseConnection,
    ) -> Result<HashMap<String, Value>, DataBaseError> {
        //查询缓存
        let cache_result =
            RedisService::get_value_map(redis_key_constants::SITE_INFO_MAP.to_string()).await;
        if let Ok(cache_result) = cache_result {
            log::info!(
                "reids KEY:{} 获取缓存数据成功",
                redis_key_constants::SITE_INFO_MAP
            );
            return Ok(cache_result);
        }

        //查询数据库
        let site_setting_list = site_setting::Entity::find().all(db).await?; // 假设这是一个 Vec 或其他可迭代集合
        let mut map: HashMap<String, Value> = HashMap::new();
        let mut introduction = Introduction::new();
        let mut site_info: HashMap<String, Value> = HashMap::new();
        let mut badges = vec![];
        let mut favorites: Vec<Favorite> = vec![];
        for v in site_setting_list {
            match v.r#type {
                //类型1
                Some(1) => {
                    match v.name_en {
                        Some(name_en) => {
                            if name_en.contains(site_setting_constants::COPYRIGHT) {
                                let copyright: Copyright =
                                    serde_json::from_str(v.value.unwrap_or_default().as_str())?;
                                site_info.insert(name_en, to_value!(copyright));
                            } else {
                                site_info
                                    .insert(name_en, Value::String(v.value.unwrap_or_default()));
                            }
                        }
                        None => {
                            return Err(DataBaseError::Custom("类型1的name_en 是Null".to_string()))
                        }
                    };
                }
                //类型2
                Some(2) => match v.name_en {
                    Some(name_en) => match name_en.as_str() {
                        site_setting_constants::AVATAR => {
                            introduction.avatar = v.value.unwrap_or_default()
                        }
                        site_setting_constants::NAME => {
                            introduction.name = v.value.unwrap_or_default()
                        }
                        site_setting_constants::GITHUB => {
                            introduction.github = v.value.unwrap_or_default()
                        }
                        site_setting_constants::TELEGRAM => {
                            introduction.telegram = v.value.unwrap_or_default()
                        }
                        site_setting_constants::QQ => introduction.qq = v.value.unwrap_or_default(),
                        site_setting_constants::BILIBILI => {
                            introduction.bilibili = v.value.unwrap_or_default()
                        }
                        site_setting_constants::NETEASE => {
                            introduction.netease = v.value.unwrap_or_default()
                        }
                        site_setting_constants::EMAIL => {
                            introduction.email = v.value.unwrap_or_default()
                        }
                        site_setting_constants::FAVORITE => {
                            let favorite =
                                serde_json::from_str(v.value.unwrap_or_default().as_str())?;
                            favorites.push(favorite);
                        }
                        site_setting_constants::ROLL_TEXT => {
                            let arr = v
                                .value
                                .unwrap_or_default()
                                .split(",")
                                .map(String::from)
                                .collect();
                            introduction.roll_text = arr;
                        }
                        _ => (),
                    },
                    None => {
                        return Err(DataBaseError::Custom("类型2的 name_en 是Null".to_string()))
                    }
                },
                //类型3
                Some(3) => match v.name_en {
                    Some(_) => {
                        let badge: Badge =
                            serde_json::from_str(v.value.unwrap_or_default().as_str())?;
                        badges.push(badge);
                    }
                    None => return Err(DataBaseError::Custom("类型3的name_en 是Null".to_string())),
                },
                _ => (),
            }
            //类型3
        }
        introduction.favorites = favorites;
        map.insert("introduction".to_string(), to_value!(introduction));
        map.insert("siteInfo".to_string(), to_value!(site_info));
        map.insert("badges".to_string(), to_value!(badges));
        //缓存数据
        RedisService::set_value_map(redis_key_constants::SITE_INFO_MAP.to_string(), &map).await?;
        log::info!("redis KEY:{} 写入缓存数据成功", redis_key_constants::SITE_INFO_MAP);
        Ok(map)
    }
}
