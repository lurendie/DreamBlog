use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::entity::blog::Model as Blog;
use crate::model::{Category, TagDTO};
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
