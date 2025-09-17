use chrono::NaiveDateTime;
use sea_orm::{DatabaseConnection, ModelTrait};
use serde::{Deserialize, Serialize};

use crate::entity::{category, tag};
use crate::model::{Category, TagDTO};

use crate::entity::blog::{self, Model as Blog};
//博客简要信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BlogInfo {
    pub id: Option<i64>,
    pub title: String,
    pub description: String,
    #[serde(rename(serialize = "createTime"))]
    pub create_time: NaiveDateTime,
    pub views: i32,
    pub words: i32,
    #[serde(rename(serialize = "readTime"))]
    pub read_time: i32,
    pub password: Option<String>,
    pub privacy: Option<bool>,
    pub is_top: bool,
    pub tags: Option<Vec<TagDTO>>,
    pub category: Option<Category>,
    #[serde(rename(serialize = "firstPicture", deserialize = "first_picture"))]
    pub first_picture: Option<String>,
}

// impl BlogInfo {
//     fn new() -> Self {
//         BlogInfo::default()
//     }
// }

impl From<Blog> for BlogInfo {
    fn from(model: Blog) -> Self {
        BlogInfo {
            id: Some(model.id),
            title: model.title,
            description: model.description,
            create_time: model.create_time,
            views: model.views,
            words: model.words,
            read_time: model.read_time,
            password: model.password,
            privacy: None,
            is_top: model.is_top,
            tags: None,
            category: None,
            first_picture: Some(model.first_picture),
        }
    }
}

impl BlogInfo {
    pub async fn related_handle(&mut self, model: blog::Model, db: &DatabaseConnection) {
        let category_model = match model.find_related(category::Entity).one(db).await {
            Ok(category_model) => category_model.unwrap_or_default(),
            Err(e) => {
                log::error!("{:?}", e);
                category::Model::default()
            }
        };

        self.category = Some(Category::from(category_model));

        let tag_models = model
            .find_related(tag::Entity)
            .all(db)
            .await
            .unwrap_or_default();
        let mut tags = vec![];
        for tag_model in tag_models {
            tags.push(TagDTO::from(tag_model))
        }
        self.tags = Some(tags);
    }
}
