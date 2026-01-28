/*
 * @Author: lurendie
 * @Date: 2024-02-24 22:58:03
 * @LastEditors: lurendie
 * @LastEditTime: 2024-04-19 23:46:51
 * @FilePath: \zero_blog\src\service\category_service.rs
 */

use rbs::value::map::ValueMap;
use rbs::{value, Value};
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, EntityTrait, ModelTrait, PaginatorTrait,
};

use crate::constant::RedisKeyConstant;
use crate::entity::{blog, category};
use crate::error::DataBaseError;
use crate::model::Categorie;
use crate::model::Category;
use crate::model::Serise;
use crate::service::RedisService;

pub struct CategoryService;

impl CategoryService {
    /**
     * 查询所有分类(首页)
     */
    pub async fn get_list(db: &DatabaseConnection) -> Result<Vec<Value>, DataBaseError> {
        //1.查询Redis
        let result =
            RedisService::get_value_vec(RedisKeyConstant::CATEGORY_NAME_LIST.to_string()).await;
        if let Some(result) = result {
            let arr = match result {
                Value::Array(arr) => {
                    log::info!(
                        "reids KEY:{} 获取缓存数据成功",
                        RedisKeyConstant::CATEGORY_NAME_LIST.to_string()
                    );
                    arr
                }
                _ => vec![],
            };
            return Ok(arr);
        }
        //2.查询数据库
        let mut result = vec![];
        category::Entity::find()
            .all(db)
            .await
            .unwrap_or_default()
            .into_iter()
            .for_each(|model| {
                result.push(value!(Category::from(model)));
            });

        if result.len() > 0 {
            //3.保存Redis
            RedisService::set_value_vec(
                RedisKeyConstant::CATEGORY_NAME_LIST.to_string(),
                &value!(&result),
            )
            .await?;
            log::info!(
                "redis KEY:{} 缓存数据成功",
                RedisKeyConstant::CATEGORY_NAME_LIST
            );
        }
        Ok(result)
    }

    /**
     * 查询分类名称
     */
    pub async fn get_series(db: &DatabaseConnection) -> ValueMap {
        let mut map = ValueMap::new();
        let mut legend = vec![];
        let mut series = vec![];
        match category::Entity::find().all(db).await {
            Ok(items) => {
                for item in items {
                    legend.push(value!(&item.category_name));

                    let count = match item.find_related(blog::Entity).count(db).await {
                        Ok(count) => count,
                        Err(e) => {
                            log::error!("查询分类文章数失败:{}", e);
                            0
                        }
                    };
                    let series_item = Serise::new(item.id, item.category_name, count);
                    series.push(series_item);
                }
            }
            Err(e) => {
                log::error!("查询分类失败:{}", e);
            }
        }
        map.insert(value!("legend"), value!(legend));
        map.insert(value!("series"), value!(series));
        map
    }

    pub(crate) async fn find_categories(db: &DatabaseConnection) -> Vec<Categorie> {
        let mut list = vec![];
        category::Entity::find()
            .all(db)
            .await
            .unwrap_or_default()
            .into_iter()
            .for_each(|model| {
                list.push(Categorie::new(
                    Some(model.id),
                    model.category_name.to_string(),
                    vec![],
                ))
            });
        list
    }

    //查询所有分类(后台)
    pub async fn get_page_categories(
        page_num: u64,
        page_size: u64,
        db: &DatabaseConnection,
    ) -> Result<ValueMap, DataBaseError> {
        let page = category::Entity::find().paginate(db, page_size);
        let models = page.fetch_page(page_num - 1).await?;
        let mut list: Vec<Categorie> = vec![];
        for model in models {
            list.push(model.into());
        }
        let mut map = ValueMap::new();
        map.insert(value!("pageNum"), value!(page_num));
        map.insert(value!("pageSize"), value!(page_size));
        map.insert(value!("pages"), value!(page.num_pages().await?));
        map.insert(value!("total"), value!(page.num_items().await?));
        map.insert(value!("list"), value!(list));
        Ok(map)
    }

    pub async fn insert_category(
        name: String,
        db: &DatabaseConnection,
    ) -> Result<(), DataBaseError> {
        category::ActiveModel {
            category_name: sea_orm::ActiveValue::Set(name),
            ..Default::default()
        }
        .insert(db)
        .await?;
        RedisService::_del_key(RedisKeyConstant::CATEGORY_NAME_LIST).await?;
        Ok(())
    }

    pub async fn update_category(
        category: Category,
        db: &DatabaseConnection,
    ) -> Result<(), DataBaseError> {
        category::ActiveModel {
            category_name: sea_orm::ActiveValue::set(category.get_name().to_string()),
            id: sea_orm::ActiveValue::set(category.get_id()),
        }
        .update(db)
        .await?;
        RedisService::_del_key(RedisKeyConstant::CATEGORY_NAME_LIST).await?;
        Ok(())
    }

    pub async fn delete_category(id: i64, db: &DatabaseConnection) -> Result<u64, DataBaseError> {
        //判断分类是否有文章
        let model = category::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| DataBaseError::Custom("分类不存在".to_string()))?;

        let count = model.find_related(blog::Entity).count(db).await?;
        if count > 0 {
            return Err(DataBaseError::Custom("分类下有文章，不能删除".to_string()));
        }
        let result = category::Entity::delete_by_id(id).exec(db).await?;
        RedisService::_del_key(RedisKeyConstant::CATEGORY_NAME_LIST).await?;
        Ok(result.rows_affected)
    }
}
