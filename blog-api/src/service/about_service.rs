use crate::entity::about;
use crate::error::DataBaseError;
use crate::common::MarkdownParser;
use rbs::value;
use rbs::value::map::ValueMap;
use sea_orm::{DatabaseConnection, EntityTrait};

pub struct AboutService;

impl AboutService {
    ///获取所有about信息
    pub(crate) async fn get_about(db: &DatabaseConnection) -> Result<ValueMap, DataBaseError> {
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
        Ok(map)
    }
}
