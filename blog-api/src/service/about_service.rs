use crate::common::MarkdownParser;
use crate::constant::RedisKeyConstant;
use crate::entity::about;
use crate::error::DataBaseError;
use crate::model::AboutForm;
use crate::service::RedisService;
use rbs::value;
use rbs::value::map::ValueMap;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbBackend, EntityTrait, FromQueryResult,
    QueryFilter, Statement,
};

pub struct AboutService;

impl AboutService {
    ///获取所有about信息
    pub(crate) async fn get_about(db: &DatabaseConnection) -> Result<ValueMap, DataBaseError> {
        //从缓存中获取
        if let Ok(map) =
            RedisService::get_string(RedisKeyConstant::ABOUT_INFO_MAP.to_string()).await
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
            RedisService::set_string(RedisKeyConstant::ABOUT_INFO_MAP.to_string(), &map).await?;
        }
        Ok(map)
    }

    /// 后台获取about信息（不做HTML转换）
    pub(crate) async fn get_about_raw(db: &DatabaseConnection) -> Result<ValueMap, DataBaseError> {
        let mut map = ValueMap::new();
        about::Entity::find()
            .all(db)
            .await?
            .into_iter()
            .for_each(|item| {
                let name = item.name_en.unwrap_or_default();
                let value = item.value.unwrap_or_default();
                map.insert(value!(name), value!(value));
            });
        Ok(map)
    }

    pub(crate) async fn update_about(
        form: AboutForm,
        db: &DatabaseConnection,
    ) -> Result<(), DataBaseError> {
        let updates = vec![
            ("title", "标题", form.title),
            (
                "musicId",
                "网易云歌曲ID",
                form.music_id.unwrap_or_default(),
            ),
            ("content", "正文Markdown", form.content),
            (
                "commentEnabled",
                "评论开关",
                form.comment_enabled.to_string(),
            ),
        ];

        for (name_en, name_zh, value) in updates {
            let existing = about::Entity::find()
                .filter(about::Column::NameEn.eq(name_en))
                .one(db)
                .await?;
            if let Some(model) = existing {
                let mut active: about::ActiveModel = model.into();
                active.name_zh = sea_orm::ActiveValue::set(Some(name_zh.to_string()));
                active.value = sea_orm::ActiveValue::set(Some(value));
                active.update(db).await?;
            } else {
                let next_id = Self::next_about_id(db).await?;
                let active = about::ActiveModel {
                    id: sea_orm::ActiveValue::set(next_id),
                    name_en: sea_orm::ActiveValue::set(Some(name_en.to_string())),
                    name_zh: sea_orm::ActiveValue::set(Some(name_zh.to_string())),
                    value: sea_orm::ActiveValue::set(Some(value)),
                };
                about::Entity::insert(active).exec(db).await?;
            }
        }

        RedisService::_del_key(RedisKeyConstant::ABOUT_INFO_MAP).await?;
        Ok(())
    }

    async fn next_about_id(db: &DatabaseConnection) -> Result<i64, DataBaseError> {
        #[derive(FromQueryResult)]
        struct MaxId {
            id: i64,
        }

        let sql = Statement::from_sql_and_values(
            DbBackend::MySql,
            "SELECT COALESCE(MAX(id), 0) + 1 as id FROM about",
            [],
        );
        let result = MaxId::find_by_statement(sql).one(db).await?;
        Ok(result.map(|item| item.id).unwrap_or(1))
    }
}
