use crate::common::MarkdownParser;
use crate::constant::RedisKeyConstant;
use crate::entity::about;
use crate::error::DataBaseError;
use crate::model::AboutForm;
use crate::service::RedisService;
use rbs::value;
use rbs::value::map::ValueMap;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection, DbBackend, EntityTrait,
    FromQueryResult, QueryFilter, Statement, TransactionTrait,
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
            RedisService::try_set_string(RedisKeyConstant::ABOUT_INFO_MAP.to_string(), &map).await;
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
            ("musicId", "网易云歌曲ID", form.music_id.unwrap_or_default()),
            ("content", "正文Markdown", form.content),
            (
                "commentEnabled",
                "评论开关",
                form.comment_enabled.to_string(),
            ),
        ];

        // 事务内完成全部写入；主键生成带 FOR UPDATE，避免并发时 MAX(id)+1 冲突
        db.transaction(|txn| {
            Box::pin(async move {
                for (name_en, name_zh, value) in updates {
                    let existing = about::Entity::find()
                        .filter(about::Column::NameEn.eq(name_en))
                        .one(txn)
                        .await?;
                    if let Some(model) = existing {
                        let mut active: about::ActiveModel = model.into();
                        active.name_zh = sea_orm::ActiveValue::set(Some(name_zh.to_string()));
                        active.value = sea_orm::ActiveValue::set(Some(value));
                        active.update(txn).await?;
                    } else {
                        let next_id = AboutService::next_about_id(txn).await?;
                        let active = about::ActiveModel {
                            id: sea_orm::ActiveValue::set(next_id),
                            name_en: sea_orm::ActiveValue::set(Some(name_en.to_string())),
                            name_zh: sea_orm::ActiveValue::set(Some(name_zh.to_string())),
                            value: sea_orm::ActiveValue::set(Some(value)),
                        };
                        about::Entity::insert(active).exec(txn).await?;
                    }
                }
                Ok(())
            })
        })
        .await?;

        RedisService::try_del_key(RedisKeyConstant::ABOUT_INFO_MAP).await;
        Ok(())
    }

    /// 生成下一条 about 主键：事务内 FOR UPDATE 锁住 MAX 扫描区间，串行化并发插入
    async fn next_about_id(conn: &impl ConnectionTrait) -> Result<i64, sea_orm::DbErr> {
        #[derive(FromQueryResult)]
        struct MaxId {
            id: i64,
        }

        let sql = Statement::from_sql_and_values(
            DbBackend::MySql,
            "SELECT COALESCE(MAX(id), 0) + 1 as id FROM about FOR UPDATE",
            [],
        );
        let result = MaxId::find_by_statement(sql).one(conn).await?;
        Ok(result.map(|item| item.id).unwrap_or(1))
    }
}
