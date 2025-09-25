use crate::entity::blog::Model as Blog;
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FromQueryResult, Clone, Default)]
pub struct BlogIdAndTitle {
    pub id: i64,
    pub title: String,
}

impl From<Blog> for BlogIdAndTitle {
    fn from(model: Blog) -> Self {
        BlogIdAndTitle {
            id: model.id,
            title: model.title,
        }
    }
}
