use crate::common::MarkdownParser;
use crate::constant::RedisKeyConstant;
use crate::entity::about;
use crate::error::DataBaseError;
use crate::service::RedisService;
use rbs::value;
use rbs::value::map::ValueMap;
use sea_orm::{DatabaseConnection, EntityTrait};

pub struct AboutService;

impl AboutService {
    ///获取所有about信息
    pub(crate) async fn get_about(db: &DatabaseConnection) -> Result<ValueMap, DataBaseError> {
        //从缓存中获取
        if let Ok(map) =
            RedisService::get_value_map(RedisKeyConstant::ABOUT_INFO_MAP.to_string()).await
        {
            return Ok(map);
        }
        let mut map = ValueMap::new();
        about::Entity::find()
            .all(db)
            .await?
            .into_iter()
            .for_each(|item| {
                //转HTML
                let name = item.name_en.unwrap_or_default();
                let value = item.value.unwrap_or_default();
                if name.contains("content") {
                    let content = MarkdownParser::parser_html(value);
                    map.insert(value!(name), value!(content));
                } else {
                    map.insert(value!(name), value!(value));
                }
            });
        //缓存
        if !map.is_empty() {
            RedisService::set_value_map(RedisKeyConstant::ABOUT_INFO_MAP.to_string(), &map).await?;
        }
        Ok(map)
    }
}
