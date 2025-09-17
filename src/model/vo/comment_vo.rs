use crate::entity::comment;
use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};
//评论
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CommentVO {
    pub(crate) id: i64,
    pub(crate) nickname: String,
    pub(crate) content: String,
    pub(crate) avatar: String,
    #[serde(rename(serialize = "createTime"))]
    pub(crate) create_time: NaiveDateTime,
    // #[serde(deserialize_with = "CommonService::bool_from_int")]
    // pub(crate) is_published: bool,
    #[serde(rename(serialize = "adminComment"))]
    pub(crate) is_admin_comment: bool,
    #[serde(rename(serialize = "parentCommentId"))]
    pub(crate) parent_comment_id: Option<i64>,
    #[serde(rename(serialize = "parentCommentNickname"))]
    pub(crate) parent_comment_name: Option<String>,
    pub(crate) website: Option<String>,
    //pub(crate) qq: Option<String>,
    #[serde(rename(serialize = "replyComments"))]
    pub(crate) reply_comments: Option<Vec<CommentVO>>,
}

impl From<comment::Model> for CommentVO {
    fn from(model: comment::Model) -> Self {
        Self {
            id: model.id,
            nickname: model.nickname,
            content: model.content,
            avatar: model.avatar,
            create_time: model.create_time.unwrap_or(Local::now().naive_local()),
            is_admin_comment: model.is_admin_comment,
            parent_comment_id: Some(model.parent_comment_id),
            parent_comment_name: None,
            website: model.website,
            reply_comments: None,
        }
    }
}
