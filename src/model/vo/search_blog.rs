use crate::entity::blog;
use serde::{Deserialize, Serialize};
/**
 * 文章搜索
 */
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SearchBlog {
    #[serde(rename(serialize = "id"))]
    id: i64,
    title: String,
    content: String,
}
impl SearchBlog {
    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }
}

impl From<blog::Model> for SearchBlog {
    fn from(b: blog::Model) -> Self {
        Self {
            id: b.id,
            title: b.title,
            content: b.content,
        }
    }
}
