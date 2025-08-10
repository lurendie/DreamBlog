use chrono::NaiveDateTime;
use sea_orm::{DatabaseConnection, ModelTrait};
use serde::{Deserialize, Serialize};

use crate::{
    entity::{category, tag},
    model::category::Category,
};

use super::tag_dto::TagDTO;
use crate::entity::blog;

//Blog
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BlogDTO {
    id: Option<i64>,
    title: String,
    #[serde(rename(serialize = "firstPicture"))]
    first_picture: String,
    content: String,
    description: String,
    #[serde(rename(deserialize = "is_published"))]
    published: bool,
    #[serde(rename(deserialize = "is_recommend"))]
    recommend: bool,
    #[serde(rename(deserialize = "is_appreciation"))]
    appreciation: bool,
    #[serde(rename(deserialize = "is_comment_enabled", serialize = "commentEnabled"))]
    comment_enabled: bool,
    #[serde(rename(serialize = "createTime"))]
    create_time: NaiveDateTime,
    #[serde(rename(serialize = "updateTime"))]
    update_time: NaiveDateTime,
    views: i32,
    words: i32,
    #[serde(rename(serialize = "readTime"))]
    read_time: i32,

    #[serde(rename(deserialize = "is_top"))]
    top: bool,
    password: Option<String>,
    user_id: Option<i64>,
    #[serde(rename(deserialize = "category_id"), skip_serializing)]
    //跳过该字段，不进行序列化操作。
    //category_id: i64,
    //#[serde(skip_deserializing)] // 跳过该字段，不进行反序列化操作。
    category: Option<Category>,
    tags: Option<Vec<TagDTO>>,
}

impl From<blog::Model> for BlogDTO {
    fn from(model: blog::Model) -> Self {
        BlogDTO {
            id: Some(model.id),
            title: model.title,
            first_picture: model.first_picture,
            content: model.content,
            description: model.description,
            published: model.is_published,
            recommend: model.is_recommend,
            appreciation: model.is_appreciation,
            comment_enabled: model.is_comment_enabled,
            create_time: model.create_time,
            update_time: model.update_time,
            views: model.views,
            words: model.words,
            read_time: model.read_time,
            top: model.is_top,
            password: model.password,
            user_id: model.user_id,
            category: None,
            //    category_id: model.category_id,
            tags: None,
        }
    }
}

impl BlogDTO {
    // pub fn get_id(&self) -> i64 {
    //     self.id.unwrap_or(0)
    // }
    // pub fn get_title(&self) -> &str {
    //     &self.title
    // }
    // pub fn get_first_picture(&self) -> &str {
    //     &self.first_picture
    // }

    // pub fn get_content(&self) -> &str {
    //     &self.content
    // }
    // pub fn get_description(&self) -> &str {
    //     &self.description
    // }
    // pub fn get_is_published(&self) -> bool {
    //     self.published
    // }
    // pub fn get_is_recommend(&self) -> bool {
    //     self.recommend
    // }
    // pub fn get_is_appreciation(&self) -> bool {
    //     self.appreciation
    // }
    // pub fn get_is_comment_enabled(&self) -> bool {
    //     self.comment_enabled
    // }

    // pub fn get_views(&self) -> i32 {
    //     self.views
    // }
    // pub fn get_words(&self) -> i32 {
    //     self.words
    // }
    // pub fn get_read_time(&self) -> i32 {
    //     self.read_time
    // }
    // pub fn get_is_top(&self) -> bool {
    //     self.top
    // }
    pub fn get_password(&self) -> Option<&str> {
        self.password.as_deref()
    }
    // pub fn get_user_id(&self) -> i64 {
    //     self.user_id.unwrap_or(0)
    // }
    // pub fn get_category_id(&self) -> i64 {
    //     self.category_id
    // }
    // pub fn get_category(&self) -> Option<Category> {
    //     self.category.clone()
    // }

    // pub fn set_id(&mut self, id: i64) -> &mut Self {
    //     self.id = Some(id);
    //     self
    // }
    // pub fn set_title(&mut self, title: &str) -> &mut Self {
    //     self.title = title.to_string();
    //     self
    // }
    // pub fn set_first_picture(&mut self, first_picture: &str) -> &mut Self {
    //     self.first_picture = first_picture.to_string();
    //     self
    // }
    // pub fn set_content(&mut self, content: &str) -> &mut Self {
    //     self.content = content.to_string();
    //     self
    // }
    // pub fn set_description(&mut self, description: &str) -> &mut Self {
    //     self.description = description.to_string();
    //     self
    // }
    // pub fn set_is_published(&mut self, is_published: bool) -> &mut Self {
    //     self.published = is_published;
    //     self
    // }
    // pub fn set_is_recommend(&mut self, is_recommend: bool) -> &mut Self {
    //     self.recommend = is_recommend;
    //     self
    // }
    // pub fn set_is_appreciation(&mut self, is_appreciation: bool) -> &mut Self {
    //     self.appreciation = is_appreciation;
    //     self
    // }
    // pub fn set_is_comment_enabled(&mut self, is_comment_enabled: bool) -> &mut Self {
    //     self.comment_enabled = is_comment_enabled;
    //     self
    // }
    // pub fn set_create_time(&mut self, create_time: NaiveDateTime) -> &mut Self {
    //     self.create_time = create_time;
    //     self
    // }
    // pub fn set_update_time(&mut self, update_time: NaiveDateTime) -> &mut Self {
    //     self.update_time = update_time;
    //     self
    // }
    // pub fn set_views(&mut self, views: i32) -> &mut Self {
    //     self.views = views;
    //     self
    // }
    // pub fn set_words(&mut self, words: i32) -> &mut Self {
    //     self.words = words;
    //     self
    // }
    // pub fn set_read_time(&mut self, read_time: i32) -> &mut Self {
    //     self.read_time = read_time;
    //     self
    // }
    // pub fn set_is_top(&mut self, is_top: bool) -> &mut Self {
    //     self.top = is_top;
    //     self
    // }
    pub fn set_password(&mut self, password: Option<&str>) -> &mut Self {
        self.password = password.map(|s| s.to_string());
        self
    }
    // pub fn set_user_id(&mut self, user_id: i64) -> &mut Self {
    //     self.user_id = Some(user_id);
    //     self
    // }
    // pub fn set_category_id(&mut self, category_id: i64) -> &mut Self {
    //     self.category_id = category_id;
    //     self
    // }
    // pub fn set_category(&mut self, category: Option<Category>) -> &mut Self {
    //     self.category = category.clone();
    //     self
    // }
    // pub fn set_tags(&mut self, tags: Option<Vec<TagVO>>) -> &mut Self {
    //     self.tags = tags.clone();
    //     self
    // }
    // pub fn get_tags(&self) -> Vec<TagVO> {
    //     self.tags.clone().unwrap()
    // }

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
