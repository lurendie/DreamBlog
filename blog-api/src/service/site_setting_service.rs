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
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, IntoActiveModel,
    QueryFilter, TransactionTrait,
};
use std::collections::HashMap;

pub struct SiteSettingService;

impl SiteSettingService {
    pub async fn find_site_info(db: &DatabaseConnection) -> Result<ValueMap, DataBaseError> {
        //查询缓存
        let cache_result =
            RedisService::get_string(RedisKeyConstant::SITE_INFO_MAP.to_string()).await;
        if let Ok(cache_result) = cache_result {
            log::info!(
                "reids KEY:{} 获取缓存数据成功",
                RedisKeyConstant::SITE_INFO_MAP
            );
            return Ok(cache_result);
        }

        //查询数据库
        let site_setting_list = site_setting::Entity::find().all(db).await?;
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
                                .split(',')
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
        }
        introduction.favorites = favorites;
        map.insert(value!("introduction"), value!(introduction));
        map.insert(value!("siteInfo"), value!(site_info));
        map.insert(value!("badges"), value!(badges));

        //缓存数据
        if RedisService::try_set_string(RedisKeyConstant::SITE_INFO_MAP.to_string(), &map).await {
            log::info!("redis KEY:{} 缓存数据成功", RedisKeyConstant::SITE_INFO_MAP);
        }
        Ok(map)
    }

    pub async fn get_site_info(
        db: &DatabaseConnection,
    ) -> Result<HashMap<String, Value>, DataBaseError> {
        let site_setting_list = site_setting::Entity::find().all(db).await?;
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

    pub async fn update_site_settings(
        db: &DatabaseConnection,
        settings: Vec<SiteSetting>,
        delete_ids: Vec<i64>,
    ) -> Result<(), DataBaseError> {
        db.transaction(|txn| {
            let settings = settings.clone();
            let delete_ids = delete_ids.clone();
            Box::pin(async move {
                if !delete_ids.is_empty() {
                    site_setting::Entity::delete_many()
                        .filter(site_setting::Column::Id.is_in(delete_ids))
                        .exec(txn)
                        .await?;
                }

                for setting in settings {
                    let name_en = setting.name_en;
                    let name_zh = setting.name_zh;
                    let value = setting.value;
                    let setting_type = setting.r#type;

                    match setting.id {
                        Some(id) if id > 0 => {
                            let Some(model) = site_setting::Entity::find_by_id(id).one(txn).await?
                            else {
                                return Err(DbErr::Custom(format!(
                                    "站点设置不存在，无法更新 id={}",
                                    id
                                )));
                            };
                            let mut active = model.into_active_model();
                            active.name_en = Set(Some(name_en));
                            active.name_zh = Set(Some(name_zh));
                            active.value = Set(Some(value));
                            active.r#type = Set(Some(setting_type));
                            active.update(txn).await?;
                        }
                        _ => {
                            let active = site_setting::ActiveModel {
                                name_en: Set(Some(name_en)),
                                name_zh: Set(Some(name_zh)),
                                value: Set(Some(value)),
                                r#type: Set(Some(setting_type)),
                                ..Default::default()
                            };
                            site_setting::Entity::insert(active).exec(txn).await?;
                        }
                    }
                }

                Ok(())
            })
        })
        .await?;

        // 更新后清理缓存
        RedisService::try_del_key(RedisKeyConstant::SITE_INFO_MAP).await;

        Ok(())
    }

    pub async fn get_web_title_suffix(db: &DatabaseConnection) -> Result<String, DataBaseError> {
        let model = site_setting::Entity::find()
            .filter(site_setting::Column::NameEn.eq("webTitleSuffix"))
            .one(db)
            .await?;

        Ok(model
            .and_then(|m| m.value)
            .filter(|s| !s.trim().is_empty())
            .unwrap_or_else(|| " - ZeroBlog".to_string()))
    }
}
