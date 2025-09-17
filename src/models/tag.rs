use rbatis::crud;
use serde::{Deserialize, Serialize};

use super::blog::Blog;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: Option<u16>,
    #[serde(rename(serialize = "tag_name"))]
    pub name: String,
    pub color: String,
    #[serde(skip, default)]
    pub blogs: Vec<Blog>,
}

impl Default for Tag {
    fn default() -> Self {
        Self {
            id: Some(0),
            name: "未知".to_string(),
            color: "#000000".to_string(),
            blogs: vec![],
        }
    }
}
crud!(Tag {});
