use crate::entity::tag;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagDTO {
    pub id: Option<i64>,
    //#[serde(rename(deserialize = "tag_name"))]
    pub name: String,
    pub color: String,
}

impl Default for TagDTO {
    fn default() -> Self {
        Self {
            id: Some(0),
            name: "未知".to_string(),
            color: "red".to_string(),
        }
    }
}

impl From<tag::Model> for TagDTO {
    fn from(t: tag::Model) -> Self {
        Self {
            id: Some(t.id),
            name: t.tag_name,
            color: t.color.unwrap_or("red".to_string()),
        }
    }
}
