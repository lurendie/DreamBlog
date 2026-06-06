use crate::entity::friend;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
//友链
pub struct Friend {
    pub(crate) id: Option<i64>,
    pub(crate) nickname: String,
    pub(crate) description: String,
    pub(crate) website: String,
    pub(crate) avatar: String,
    pub(crate) is_published: bool,
    pub(crate) views: i32,
    pub(crate) create_time: NaiveDateTime,
}

impl From<friend::Model> for Friend {
    fn from(friend: friend::Model) -> Self {
        Self {
            id: Some(friend.id),
            nickname: friend.nickname,
            description: friend.description,
            website: friend.website,
            avatar: friend.avatar,
            is_published: friend.is_published,
            views: friend.views,
            create_time: friend.create_time,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct FriendUpdatePublished {
    pub id: i64,
    pub published: bool,
}

#[derive(Debug, Deserialize)]
pub struct FriendCommentEnabledUpdate {
    #[serde(rename = "commentEnabled")]
    pub comment_enabled: bool,
}

#[derive(Debug, Deserialize)]
pub struct FriendContentUpdate {
    pub content: String,
}
#[derive(Debug, Deserialize)]
pub struct FriendQuery {
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
    pub nickname: Option<String>,
    pub is_published: Option<bool>,
}
