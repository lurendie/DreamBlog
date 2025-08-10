use crate::entity::friend;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
//友链
pub struct FriendInfo {
    nickname: String,
    description: String,
    website: String,
    avatar: String,
}

impl From<friend::Model> for FriendInfo {
    fn from(model: friend::Model) -> Self {
        Self {
            nickname: model.nickname,
            description: model.description,
            website: model.website,
            avatar: model.avatar,
        }
    }
}

