/*
 * @Author: lurendie
 * @Date: 2024-02-24 22:58:03
 * @LastEditors: lurendie
 * @LastEditTime: 2024-04-19 23:46:51
 * @FilePath: \zero_blog\src\service\category_service.rs
 */

use rbs::value::map::ValueMap;
use rbs::{to_value, Value};
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, ModelTrait, PaginatorTrait,
};

use crate::constant::redis_key_constants;
use crate::entity::{blog, category};
use crate::enums::DataBaseError;
use crate::model::Serise;
use crate::model::Categorie;
use crate::model::Category;
use crate::service::RedisService;

pub struct CategoryService;

impl CategoryService {
    /**
     * 查询所有分类(首页)
     */
    pub async fn get_list(db: &DatabaseConnection) -> Result<Vec<Value>, DataBaseError> {
        //1.查询Redis
        let result =
            RedisService::get_value_vec(redis_key_constants::CATEGORY_NAME_LIST.to_string()).await;
        if let Some(result) = result {
            let arr = match result {
                Value::Array(arr) => {
                    log::info!(
                        "reids KEY:{} 获取缓存数据成功",
                        redis_key_constants::CATEGORY_NAME_LIST.to_string()
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
                result.push(to_value!(Category::from(model)));
            });

        //3.保存Redis
        RedisService::set_value_vec(
            redis_key_constants::CATEGORY_NAME_LIST.to_string(),
            &to_value!(&result),
        )
        .await?;
        log::info!(
            "redis KEY:{} 写入缓存数据成功",
            redis_key_constants::CATEGORY_NAME_LIST
        );
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
                    legend.push(to_value!(&item.category_name));

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
        map.insert(to_value!("legend"), to_value!(legend));
        map.insert(to_value!("series"), to_value!(series));
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
        map.insert(to_value!("pageNum"), to_value!(page_num));
        map.insert(to_value!("pageSize"), to_value!(page_size));
        map.insert(to_value!("pages"), to_value!(page.num_pages().await?));
        map.insert(to_value!("total"), to_value!(page.num_items().await?));
        map.insert(to_value!("list"), to_value!(list));
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
        Ok(())
    }

    pub async fn delete_category(id: i64, db: &DatabaseConnection) -> Result<u64, sea_orm::DbErr> {
        //判断分类是否有文章
        let count = match category::Entity::find_by_id(id).one(db).await {
            Ok(Some(item)) => {
                let count = match item.find_related(blog::Entity).count(db).await {
                    Ok(count) => count,
                    Err(e) => {
                        log::error!("查询分类文章数失败:{}", e);
                        0
                    }
                };
                count
            }
            Ok(None) => {
                return Err(DbErr::Custom("分类下有文章，不能删除".to_string()));
            }
            Err(e) => {
                log::error!("查询分类失败:{}", e);
                0
            }
        };

        if count > 0 {
            return Err(DbErr::Custom("分类下有文章，不能删除".to_string()));
        }
        let result = category::Entity::delete_by_id(id).exec(db).await?;
        Ok(result.rows_affected)
    }
}
