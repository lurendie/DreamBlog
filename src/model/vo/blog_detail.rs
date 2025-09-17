use crate::entity::blog;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
//博客详情信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BlogDetail {
    pub(crate) id: Option<i64>,
    pub(crate) title: String,
    pub(crate) content: String,
    pub(crate) is_appreciation: bool,
    #[serde(rename(serialize = "commentEnabled"))]
    pub(crate) is_comment_enabled: bool,
    #[serde(rename(serialize = "createTime"))]
    pub(crate) create_time: NaiveDateTime,
    #[serde(rename(serialize = "updateTime"))]
    pub(crate) update_time: NaiveDateTime,
    pub(crate) views: i32,
    pub(crate) words: i32,
    #[serde(rename(serialize = "readTime"))]
    pub(crate) read_time: i32,
    pub(crate) is_top: bool,

    pub(crate) password: Option<String>,
}

impl BlogDetail {
    pub(crate) fn _new() -> Self {
        BlogDetail::default()
    }
}

impl From<blog::Model> for BlogDetail {
    fn from(model: blog::Model) -> Self {
        BlogDetail {
            id: Some(model.id),
            title: model.title,
            content: model.content,
            is_appreciation: model.is_appreciation,
            is_comment_enabled: model.is_comment_enabled,
            create_time: model.create_time,
            update_time: model.update_time,
            views: model.views,
            words: model.words,
            read_time: model.read_time,
            is_top: model.is_top,
            password: model.password,
        }
    }
}
