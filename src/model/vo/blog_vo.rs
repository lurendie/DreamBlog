use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::{enums::TypeValue, model::category::Category};

//Blog
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BlogVO {
    pub(crate) id: Option<i64>,
    pub(crate) title: String,
    #[serde(rename(deserialize = "firstPicture"))]
    pub(crate) first_picture: String,
    pub(crate) content: String,
    pub(crate) description: String,
    #[serde(rename(serialize = "is_published"))]
    pub(crate) published: bool,
    #[serde(rename(serialize = "is_recommend"))]
    pub(crate) recommend: bool,
    #[serde(rename(serialize = "is_appreciation"))]
    pub(crate) appreciation: bool,
    #[serde(rename(serialize = "is_comment_enabled", deserialize = "commentEnabled"))]
    pub(crate) comment_enabled: bool,
    #[serde(rename(deserialize = "createTime"))]
    pub(crate) create_time: Option<NaiveDateTime>,
    #[serde(rename(deserialize = "updateTime"))]
    pub(crate) update_time: Option<NaiveDateTime>,
    pub(crate) views: i32,
    pub(crate) words: i32,
    #[serde(rename(deserialize = "readTime"))]
    pub(crate) read_time: i32,
    //category_id: u16,
    #[serde(rename(serialize = "is_top"))]
    pub(crate) top: bool,
    pub(crate) password: Option<String>,
    pub(crate) user_id: Option<i64>,
    #[serde(rename(deserialize = "cate"))]
    //跳过该字段，不进行序列化操作。
    pub(crate) category_id: i64,
    //#[serde(skip_deserializing)] // 跳过该字段，不进行反序列化操作。
    pub(crate) category: Option<Category>,
    #[serde(rename(deserialize = "tagList"), skip_serializing)]
    pub(crate) tag_list: Option<Vec<TypeValue>>,
}

impl BlogVO {
    pub fn get_id(&self) -> i64 {
        self.id.unwrap_or(0)
    }
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
    // pub fn get_password(&self) -> Option<&str> {
    //     self.password.as_deref()
    // }
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
    //     self.create_time = Some(create_time);
    //     self
    // }
    // pub fn set_update_time(&mut self, update_time: NaiveDateTime) -> &mut Self {
    //     self.update_time = Some(update_time);
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
    // pub fn set_password(&mut self, password: Option<&str>) -> &mut Self {
    //     self.password = password.map(|s| s.to_string());
    //     self
    // }
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
    // pub fn set_tag_list(&mut self, tag_list: Option<Vec<TypeValue>>) -> &mut Self {
    //     self.tag_list = tag_list.clone();
    //     self
    // }
    pub fn get_tag_list(&self) -> Option<Vec<TypeValue>> {
        self.tag_list.clone()
    }
}
