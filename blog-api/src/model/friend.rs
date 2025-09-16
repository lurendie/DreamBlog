use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use crate::entity::friend;
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
