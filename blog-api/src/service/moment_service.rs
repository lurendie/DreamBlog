use crate::common::MarkdownParser;
use crate::constant::RedisKeyConstant;
use crate::entity::moment;
use crate::error::DataBaseError;
use crate::model::Moment;
use crate::model::MomentDTO;
use crate::service::RedisService;
use chrono::Local;
use rbs::{value, value::map::ValueMap};
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection, DbBackend, EntityTrait,
    PaginatorTrait, QueryFilter, Statement,
};
pub struct MomentService;

impl MomentService {
    fn public_moment_cache_field(page_num: u64, page_size: u64) -> String {
        // v2：响应新增 totalPage 字段，缓存字段加版本前缀避免命中旧结构
        format!("v2:{}:{}", page_num, page_size)
    }

    //获取所有的动态
    pub(crate) async fn get_moments(
        page_num: u64,
        page_size: u64,
        db: &DatabaseConnection,
    ) -> Result<ValueMap, DataBaseError> {
        let page = moment::Entity::find().paginate(db, page_size);
        let models = page.fetch_page(page_num - 1).await?;
        let mut list: Vec<Moment> = vec![];
        for mut model in models {
            // 与公开列表一致：markdown 渲染为 HTML 后再返回
            let content = MarkdownParser::parser_html(model.content);
            model.content = content;
            list.push(model.into());
        }
        let mut value_map = ValueMap::new();
        let total_pages = page.num_pages().await?;
        value_map.insert(value!("pageNum"), value!(page_num));
        value_map.insert(value!("pageSize"), value!(page_size));
        value_map.insert(value!("pages"), value!(total_pages));
        value_map.insert(value!("totalPage"), value!(total_pages));
        value_map.insert(value!("total"), value!(page.num_items().await?));
        value_map.insert(value!("list"), value!(list));
        Ok(value_map)
    }
    //创建动态
    pub async fn create_and_update(
        moment_dto: MomentDTO,
        db: &DatabaseConnection,
    ) -> Result<(), DataBaseError> {
        // create_time/likes 为 Copy 类型，先取出默认值，避免后续移动 content 后无法整体借用
        let create_time = moment_dto
            .create_time
            .unwrap_or_else(|| Local::now().naive_local());
        let likes = moment_dto.likes.unwrap_or(0);
        let model = moment::Entity::find_by_id(moment_dto.id.unwrap_or(0))
            .one(db)
            .await?;
        match model {
            Some(model) => {
                let mut active_model = moment::ActiveModel::from(model);
                active_model.content = Set(moment_dto.content);
                active_model.likes = Set(Some(likes));
                active_model.create_time = Set(create_time);
                active_model.is_published = Set(moment_dto.is_published);
                active_model.update(db).await?;
            }
            None => {
                moment::ActiveModel::from(moment::Model::from(moment_dto))
                    .insert(db)
                    .await?;
            }
        }
        Self::clear_public_moment_cache().await;
        Ok(())
    }

    //获取公开的动态
    pub(crate) async fn get_public_moments(
        page_num: u64,
        page_size: u64,
        db: &DatabaseConnection,
    ) -> Result<ValueMap, DataBaseError> {
        let cache_field = Self::public_moment_cache_field(page_num, page_size);
        let redis_cache = RedisService::get_hash_key(
            RedisKeyConstant::PUBLIC_MOMENT_LIST.to_string(),
            cache_field.clone(),
        )
        .await;
        if let Ok(redis_cache) = redis_cache {
            tracing::info!(
                "redis KEY:{} 字段:{} 获取缓存数据成功",
                RedisKeyConstant::PUBLIC_MOMENT_LIST,
                cache_field
            );
            return Ok(redis_cache);
        }

        let page = moment::Entity::find()
            .filter(moment::Column::IsPublished.eq(true))
            .paginate(db, page_size);
        let models = page.fetch_page(page_num - 1).await?;
        let mut list: Vec<Moment> = vec![];
        for mut model in models {
            let content = MarkdownParser::parser_html(model.content);
            model.content = content;
            list.push(model.into());
        }
        let mut value_map = ValueMap::new();
        let total_pages = page.num_pages().await?;
        value_map.insert(value!("pageNum"), value!(page_num));
        value_map.insert(value!("pageSize"), value!(page_size));
        value_map.insert(value!("pages"), value!(total_pages));
        value_map.insert(value!("totalPage"), value!(total_pages));
        value_map.insert(value!("total"), value!(page.num_items().await?));
        value_map.insert(value!("list"), value!(list));
        RedisService::try_set_hash_key(
            RedisKeyConstant::PUBLIC_MOMENT_LIST.to_string(),
            cache_field,
            &value_map,
        )
        .await;
        Ok(value_map)
    }

    /**
     * 更新动态的发布状态
     */
    pub(crate) async fn update_published(
        id: i64,
        is_published: bool,
        db: &DatabaseConnection,
    ) -> Result<(), DataBaseError> {
        let model = moment::Entity::find_by_id(id).one(db).await?;
        match model {
            Some(model) => {
                let mut active = moment::ActiveModel::from(model);
                active.set(moment::Column::IsPublished, is_published.into());
                active.update(db).await?;
                Self::clear_public_moment_cache().await;
            }
            None => {
                return Err(DataBaseError::Custom(format!("动态 id:{} 没有检索到", id)));
            }
        }
        Ok(())
    }

    /**
     * 删除动态
     */
    pub(crate) async fn delete_moment(
        id: i64,
        db: &DatabaseConnection,
    ) -> Result<(), DataBaseError> {
        let model = moment::Entity::find_by_id(id).one(db).await?;
        match model {
            Some(model) => {
                moment::ActiveModel::from(model).delete(db).await?;
                Self::clear_public_moment_cache().await;
            }
            None => {
                return Err(DataBaseError::Custom(format!("动态 id:{} 没有检索到 ", id)));
            }
        }
        Ok(())
    }

    /**
     * 获取ID动态
     */
    pub(crate) async fn get_moment_by_id(
        id: i64,
        db: &DatabaseConnection,
    ) -> Result<Moment, DataBaseError> {
        let model = moment::Entity::find_by_id(id).one(db).await?;
        match model {
            Some(model) => Ok(Moment::from(model)),
            None => Err(DataBaseError::Custom(format!("动态 id:{} 没有检索到 ", id))),
        }
    }

    pub async fn moment_like(id: i64, db: &DatabaseConnection) -> Result<(), DataBaseError> {
        // 单条 UPDATE 原子自增，避免并发点赞读改写互相覆盖丢失
        let sql = Statement::from_sql_and_values(
            DbBackend::MySql,
            "UPDATE moment SET likes = COALESCE(likes, 0) + 1 WHERE id = ?",
            [id.into()],
        );
        let result = db.execute(sql).await?;
        if result.rows_affected() == 0 {
            return Err(DataBaseError::Custom(format!("动态 id:{} 没有检索到 ", id)));
        }
        Self::clear_public_moment_cache().await;
        Ok(())
    }

    async fn clear_public_moment_cache() {
        RedisService::try_del_key(RedisKeyConstant::PUBLIC_MOMENT_LIST).await;
    }
}
