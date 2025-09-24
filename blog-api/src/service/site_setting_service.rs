use crate::constant::RedisKeyConstant;
use crate::constant::SiteSettingConstant;
use crate::entity::site_setting;
use crate::error::DataBaseError;
use crate::model::SiteSetting;
use crate::model::{Badge, Copyright, Favorite, Introduction};
use crate::service::RedisService;
use rbs::value;
use rbs::value::map::ValueMap;
use rbs::Value;
use sea_orm::DatabaseConnection;
use sea_orm::EntityTrait;
use std::collections::HashMap;

pub struct SiteSettingService;

impl SiteSettingService {
    pub async fn find_site_info(db: &DatabaseConnection) -> Result<ValueMap, DataBaseError> {
        //查询缓存
        let cache_result =
            RedisService::get_value_map(RedisKeyConstant::SITE_INFO_MAP.to_string()).await;
        if let Ok(cache_result) = cache_result {
            log::info!(
                "reids KEY:{} 获取缓存数据成功",
                RedisKeyConstant::SITE_INFO_MAP
            );
            return Ok(cache_result);
        }

        //查询数据库
        let site_setting_list = site_setting::Entity::find().all(db).await?; // 假设这是一个 Vec 或其他可迭代集合
        let mut map = ValueMap::new();
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
                            if name_en.contains(SiteSettingConstant::COPYRIGHT) {
                                let copyright: Copyright =
                                    serde_json::from_str(v.value.unwrap_or_default().as_str())?;
                                site_info.insert(name_en, value!(copyright));
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
                        SiteSettingConstant::AVATAR => {
                            introduction.avatar = v.value.unwrap_or_default()
                        }
                        SiteSettingConstant::NAME => {
                            introduction.name = v.value.unwrap_or_default()
                        }
                        SiteSettingConstant::GITHUB => {
                            introduction.github = v.value.unwrap_or_default()
                        }
                        SiteSettingConstant::TELEGRAM => {
                            introduction.telegram = v.value.unwrap_or_default()
                        }
                        SiteSettingConstant::QQ => introduction.qq = v.value.unwrap_or_default(),
                        SiteSettingConstant::BILIBILI => {
                            introduction.bilibili = v.value.unwrap_or_default()
                        }
                        SiteSettingConstant::NETEASE => {
                            introduction.netease = v.value.unwrap_or_default()
                        }
                        SiteSettingConstant::EMAIL => {
                            introduction.email = v.value.unwrap_or_default()
                        }
                        SiteSettingConstant::FAVORITE => {
                            let favorite =
                                serde_json::from_str(v.value.unwrap_or_default().as_str())?;
                            favorites.push(favorite);
                        }
                        SiteSettingConstant::ROLL_TEXT => {
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
        map.insert(value!("introduction"), value!(introduction));
        map.insert(value!("siteInfo"), value!(site_info));
        map.insert(value!("badges"), value!(badges));
        //缓存数据
        RedisService::set_value_map(RedisKeyConstant::SITE_INFO_MAP.to_string(), &map).await?;
        log::info!("redis KEY:{} 缓存数据成功", RedisKeyConstant::SITE_INFO_MAP);
        Ok(map)
    }

    pub async fn get_site_info(
        db: &DatabaseConnection,
    ) -> Result<HashMap<String, Value>, DataBaseError> {
        //查询数据库
        let site_setting_list = site_setting::Entity::find().all(db).await?; // 假设这是一个 Vec 或其他可迭代集合
        let mut map = HashMap::new();
        let mut site_type = vec![];
        let mut site_type2 = vec![];
        let mut site_type3 = vec![];
        for item in site_setting_list {
            match item.r#type {
                Some(1) => {
                    site_type.push(SiteSetting::from(item));
                }
                Some(2) => {
                    site_type2.push(SiteSetting::from(item));
                }
                Some(3) => {
                    site_type3.push(SiteSetting::from(item));
                }
                _ => (),
            }
        }

        map.insert("type1".to_string(), value!(site_type));
        map.insert("type2".to_string(), value!(site_type2));
        map.insert("type3".to_string(), value!(site_type3));
        Ok(map)
    }
}
