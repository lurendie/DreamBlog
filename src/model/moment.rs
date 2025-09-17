use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use crate::entity::moment;
use crate::util::MarkdownParser;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Moment {
    pub(crate) id: Option<u64>,
    pub(crate) content: String,
    #[serde(rename(serialize = "createTime"))]
    pub(crate) create_time: NaiveDateTime,
    pub(crate) likes: i32,
    #[serde(rename(serialize = "published"))]
    pub(crate) is_published: bool,
}

impl From<moment::Model> for Moment {
    fn from(model: moment::Model) -> Self {
        Self {
            id: Some(model.id as u64),
            content: MarkdownParser::parser_html(model.content), 
            create_time: model.create_time,
            likes: model.likes.unwrap_or(0),
            is_published: model.is_published,
        }
    }
}
