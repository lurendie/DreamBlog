use crate::entity::moment;
use crate::enums::DataBaseError;
use crate::model::MomentDTO;
use crate::model::Moment;
use crate::util::MarkdownParser;
use rbs::{to_value, value::map::ValueMap};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
};
pub struct MomentService;

impl MomentService {
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
            let content = MarkdownParser::parser_html(model.content);
            model.content = content;
            list.push(model.into());
        }
        let mut value_map = ValueMap::new();
        value_map.insert(to_value!("pageNum"), to_value!(page_num));
        value_map.insert(to_value!("pageSize"), to_value!(page_size));
        value_map.insert(to_value!("pages"), to_value!(page.num_pages().await?));
        value_map.insert(to_value!("total"), to_value!(page.num_items().await?));
        value_map.insert(to_value!("list"), to_value!(list));
        Ok(value_map)
    }
    //创建动态
    pub async fn create_and_update(
        moment_dto: MomentDTO,
        db: &DatabaseConnection,
    ) -> Result<(), DataBaseError> {
        let model = moment::Entity::find_by_id(moment_dto.id.unwrap_or_default())
            .one(db)
            .await?;
        match model {
            Some(model) => {
                moment::ActiveModel::from(model).update(db).await?;
            }
            None => {
                moment::ActiveModel::from(moment::Model::from(moment_dto))
                    .insert(db)
                    .await?;
            }
        }
        Ok(())
    }

    //获取公开的动态
    pub(crate) async fn get_public_moments(
        page_num: u64,
        page_size: u64,
        db: &DatabaseConnection,
    ) -> Result<ValueMap, DataBaseError> {
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
        value_map.insert(to_value!("pageNum"), to_value!(page_num));
        value_map.insert(to_value!("pageSize"), to_value!(page_size));
        value_map.insert(to_value!("pages"), to_value!(page.num_pages().await?));
        value_map.insert(to_value!("total"), to_value!(page.num_items().await?));
        value_map.insert(to_value!("list"), to_value!(list));
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
        let model = moment::Entity::find_by_id(id).one(db).await?;
        match model {
            Some(model) => {
                let likes = model.likes.unwrap_or_default() + 1;
                let mut active = moment::ActiveModel::from(model);
                active.set(moment::Column::Likes, likes.into());
                active.save(db).await?;
            }
            None => {
                return Err(DataBaseError::Custom(format!("动态 id:{} 没有检索到 ", id)));
            }
        }
        Ok(())
    }
}
