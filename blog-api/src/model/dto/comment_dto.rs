use crate::{entity::comment, model::BlogIdAndTitle};
use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};
//评论
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CommentDTO {
    pub(crate) id: i64,
    pub(crate) nickname: String,
    pub(crate) avatar: String,

    pub(crate) published: bool,
    pub(crate) email: Option<String>,
    pub(crate) ip: Option<String>,
    #[serde(rename(serialize = "createTime"))]
    pub(crate) create_time: NaiveDateTime,
    #[serde(rename(serialize = "notice"))]
    pub is_notice: bool,
    pub page: i8,
    pub(crate) website: Option<String>,
    pub(crate) qq: Option<String>,
    #[serde(rename(serialize = "blog"))]
    pub(crate) blog_id_and_title: Option<BlogIdAndTitle>,
    pub(crate) content: String,
}

impl From<comment::Model> for CommentDTO {
    fn from(model: comment::Model) -> Self {
        Self {
            id: model.id,
            nickname: model.nickname,
            avatar: model.avatar,
            published: model.is_admin_comment,
            email: Some(model.email),
            ip: Some(model.ip.unwrap_or_default()),
            create_time: model.create_time.unwrap_or(Local::now().naive_local()),
            is_notice: model.is_notice,
            page: model.page,
            website: model.website,
            qq: model.qq,
            blog_id_and_title: None,
            content: model.content,
        }
    }
}
