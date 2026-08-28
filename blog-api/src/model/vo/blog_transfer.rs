use crate::common::TypeValue;
use crate::entity::blog;
use crate::model::BlogVO;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// JSON representation used by the admin article import/export endpoints.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BlogTransfer {
    pub id: Option<i64>,
    pub title: String,
    #[serde(rename = "firstPicture")]
    pub first_picture: String,
    pub content: String,
    pub description: String,
    pub published: bool,
    pub recommend: bool,
    pub appreciation: bool,
    #[serde(rename = "commentEnabled")]
    pub comment_enabled: bool,
    pub top: bool,
    pub password: Option<String>,
    pub user_id: Option<i64>,
    #[serde(rename = "createTime")]
    pub create_time: Option<NaiveDateTime>,
    #[serde(rename = "updateTime")]
    pub update_time: Option<NaiveDateTime>,
    pub views: i32,
    pub words: i32,
    #[serde(rename = "readTime")]
    pub read_time: i32,
    #[serde(rename = "categoryId")]
    pub category_id: i64,
    #[serde(rename = "categoryName", default)]
    pub category_name: Option<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogExportFile {
    pub version: u8,
    #[serde(rename = "exportedAt")]
    pub exported_at: String,
    pub blogs: Vec<BlogTransfer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BlogImportPayload {
    Envelope { blogs: Vec<BlogTransfer> },
    List(Vec<BlogTransfer>),
}

impl BlogImportPayload {
    pub fn into_blogs(self) -> Vec<BlogTransfer> {
        match self {
            Self::Envelope { blogs } => blogs,
            Self::List(blogs) => blogs,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BlogImportResult {
    pub total: usize,
    pub created: usize,
    pub updated: usize,
    pub failed: usize,
    pub errors: Vec<BlogImportError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogImportError {
    pub index: usize,
    pub id: Option<i64>,
    pub title: String,
    pub message: String,
}

impl BlogTransfer {
    pub fn into_blog_vo(self) -> BlogVO {
        BlogVO {
            id: self.id,
            title: self.title,
            first_picture: self.first_picture,
            content: self.content,
            description: self.description,
            published: self.published,
            recommend: self.recommend,
            appreciation: self.appreciation,
            comment_enabled: self.comment_enabled,
            create_time: self.create_time,
            update_time: self.update_time,
            views: self.views,
            words: self.words,
            read_time: self.read_time,
            top: self.top,
            password: self.password,
            user_id: self.user_id,
            category_id: self.category_id,
            category: None,
            tag_list: Some(self.tags.into_iter().map(TypeValue::String).collect()),
        }
    }

    pub fn from_model(model: blog::Model, tags: Vec<String>) -> Self {
        Self {
            id: Some(model.id),
            title: model.title,
            first_picture: model.first_picture,
            content: model.content,
            description: model.description,
            published: model.is_published,
            recommend: model.is_recommend,
            appreciation: model.is_appreciation,
            comment_enabled: model.is_comment_enabled,
            top: model.is_top,
            password: model.password,
            user_id: model.user_id,
            create_time: Some(model.create_time),
            update_time: Some(model.update_time),
            views: model.views,
            words: model.words,
            read_time: model.read_time,
            category_id: model.category_id,
            category_name: None,
            tags,
        }
    }
}
