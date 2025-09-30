use crate::entity::comment;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
//评论
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub(crate) id: Option<i64>,
    pub(crate) nickname: String,
    pub(crate) email: String,
    pub(crate) content: String,
    pub(crate) avatar: String,
    #[serde(rename = "createTime")]
    pub(crate) create_time: NaiveDateTime,
    pub(crate) ip: String,
    #[serde(rename = "published")]
    pub(crate) is_published: bool,
    #[serde(rename = "adminComment")]
    pub(crate) is_admin_comment: bool,
    pub(crate) page: i8,
    #[serde(rename = "notice")]
    pub(crate) is_notice: bool,
    //pub(crate) blog_id: Option<i64>,
    #[serde(rename = "parentCommentId")]
    pub(crate) parent_comment_id: Option<i64>,
    pub(crate) website: Option<String>,
    pub(crate) qq: Option<String>,
    #[serde(rename = "replyComments")]
    pub(crate) reply_comments: Option<Vec<Comment>>,
}

impl From<comment::Model> for Comment {
    fn from(model: comment::Model) -> Self {
        Self {
            id: Some(model.id),
            nickname: model.nickname,
            email: model.email,
            content: model.content,
            avatar: model.avatar,
            create_time: model.create_time.unwrap_or_default(),
            ip: model.ip.unwrap_or_default(),
            is_published: model.is_published,
            is_admin_comment: model.is_admin_comment,
            page: model.page,
            is_notice: model.is_notice,
            parent_comment_id: Some(model.parent_comment_id),
            website: model.website,
            qq: model.qq,
            reply_comments: None,
        }
    }
}
