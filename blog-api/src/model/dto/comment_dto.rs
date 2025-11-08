use crate::{entity::comment, model::BlogIdAndTitle};
use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};
//评论
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CommentDTO {
    #[serde(default = "Default::default")]
    pub(crate) id: i64,
    #[serde(default = "Default::default")]
    pub(crate) nickname: String,
    #[serde(default = "Default::default")]
    pub(crate) avatar: String,
    #[serde(default = "Default::default")]
    pub(crate) published: bool,
    #[serde(default = "Default::default")]
    pub(crate) email: String,
    #[serde(default = "Default::default")]
    pub(crate) ip: String,
    #[serde(rename(serialize = "createTime"), default = "Default::default")]
    pub(crate) create_time: NaiveDateTime,

    #[serde(rename(deserialize = "notice"), default = "Default::default")]
    pub is_notice: bool,
    #[serde(default = "Default::default")]
    pub page: i8,
    #[serde(default = "Default::default")]
    pub(crate) website: String,
    #[serde(default = "Default::default")]
    pub(crate) qq: String,
    #[serde(rename(serialize = "blog"))]
    pub(crate) blog_id_and_title: Option<BlogIdAndTitle>,
     #[serde(rename(deserialize = "blogId"))]
    pub(crate) blog_id: i64,
    pub(crate) content: String,
    #[serde(rename = "parentCommentId", default = "default_parent_comment_id")]
    pub(crate) parent_comment_id: i64,
}

pub fn default_parent_comment_id() -> i64 {
    -1
}

impl From<comment::Model> for CommentDTO {
    fn from(model: comment::Model) -> Self {
        Self {
            id: model.id,
            nickname: model.nickname,
            avatar: model.avatar,
            published: model.is_published,
            email: model.email,
            ip: model.ip.unwrap_or_default(),
            create_time: model.create_time.unwrap_or(Local::now().naive_local()),
            is_notice: model.is_notice,
            page: model.page,
            website: model.website.unwrap_or_default(),
            qq: model.qq.unwrap_or_default(),
            blog_id_and_title: None,
            content: model.content,
            parent_comment_id: model.parent_comment_id,
            blog_id: model.blog_id.unwrap_or_default(),
        }
    }
}
