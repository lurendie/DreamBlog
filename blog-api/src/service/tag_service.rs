use rbs::value::map::ValueMap;
use rbs::{to_value, Value};
use sea_orm::ActiveValue::NotSet;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, ModelTrait,
    PaginatorTrait, QueryFilter,
};

use crate::constant::redis_key_constants;
use crate::entity::{blog, blog_tag, tag};
use crate::enums::DataBaseError;
use crate::model::Serise;
use crate::model::TagDTO;

use super::RedisService;
pub struct TagService;
impl TagService {
    pub async fn get_tags(db: &DatabaseConnection) -> Result<Vec<Value>, DataBaseError> {
        let mut result = vec![];
        //1.查询redis缓存
        let redis_cache =
            RedisService::get_value_vec(redis_key_constants::TAG_CLOUD_LIST.to_string()).await;
        if let Some(redis_cache) = redis_cache {
            let arr = match redis_cache {
                Value::Array(arr) => {
                    log::info!(
                        "reids KEY:{} 获取缓存数据成功",
                        redis_key_constants::TAG_CLOUD_LIST.to_string()
                    );
                    arr
                }
                _ => vec![],
            };
            return Ok(arr);
        }
        //2.查询数据库
        tag::Entity::find()
            .all(db)
            .await
            .unwrap_or_default()
            .into_iter()
            .for_each(|model| {
                result.push(to_value!(TagDTO::from(model)));
            });

        //保存到Redis
        RedisService::set_value_vec(
            redis_key_constants::TAG_CLOUD_LIST.to_string(),
            &to_value!(&result),
        )
        .await?;
        log::info!(
            "redis KEY:{} 写入缓存数据成功",
            redis_key_constants::TAG_CLOUD_LIST
        );
        Ok(result)
    }

    pub(crate) async fn get_tags_count(db: &DatabaseConnection) -> rbs::value::map::ValueMap {
        let mut map = ValueMap::new();
        let mut legend = vec![];
        let mut series = vec![];

        match tag::Entity::find().all(db).await {
            Ok(items) => {
                for item in items {
                    legend.push(to_value!(&item.tag_name));

                    let count = match item.find_related(blog::Entity).count(db).await {
                        Ok(count) => count,
                        Err(e) => {
                            log::error!("查询标签文章数失败:{}", e);
                            0
                        }
                    };
                    let series_item = Serise::new(item.id, item.tag_name, count);
                    series.push(series_item);
                }
            }
            Err(e) => {
                log::error!("查询标签失败:{}", e);
            }
        }
        map.insert(to_value!("legend"), to_value!(legend));
        map.insert(to_value!("series"), to_value!(series));
        map
    }

    /**
     * 添加标签
     */
    pub async fn insert_or_update(
        tag_vo: TagDTO,
        db: &DatabaseConnection,
    ) -> Result<(), DataBaseError> {
        let id = tag_vo.id;
        let model: tag::Model = tag_vo.into();
        let mut active = model.into_active_model();
        if id.is_none() {
            active.id = NotSet;
        }
        active.reset_all().save(db).await?;
        Ok(())
    }

    /**
     * 查询标签 by page
     */
    pub async fn get_tags_by_page(
        page_num: u64,
        page_size: u64,
        db: &DatabaseConnection,
    ) -> Result<ValueMap, DataBaseError> {
        let page = tag::Entity::find().paginate(db, page_size);
        let models = page.fetch_page(page_num - 1).await?;
        let mut list: Vec<TagDTO> = vec![];
        for model in models {
            list.push(model.into());
        }
        let mut map: ValueMap = ValueMap::new();
        map.insert(to_value!("pageNum"), to_value!(page_num));
        map.insert(to_value!("pageSize"), to_value!(page_size));
        map.insert(to_value!("pages"), to_value!(page.num_pages().await?));
        map.insert(to_value!("total"), to_value!(page.num_items().await?));
        map.insert(to_value!("list"), to_value!(list));
        Ok(map)
    }

    pub async fn delete_by_id(id: i64, db: &DatabaseConnection) -> Result<(), DataBaseError> {
        let count = blog_tag::Entity::find()
            .filter(blog_tag::Column::TagId.eq(id))
            .count(db)
            .await?;
        match count > 0 {
            true => return Err(DataBaseError::Custom("标签下有文章，不能删除".to_string())),
            false => {
                tag::Entity::delete_by_id(id).exec(db).await?;
                Ok(())
            }
        }
    }
}
