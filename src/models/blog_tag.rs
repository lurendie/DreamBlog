use rbatis::crud;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BlogTag {
    blog_id: u16,
    tag_id: u16,
}

impl BlogTag {
    pub fn new() -> Self {
        BlogTag::default()
    }
    pub fn from(blog_id: u16, tag_id: u16) -> Self {
        BlogTag { blog_id, tag_id }
    }
    pub fn get_blog_id(&self) -> u16 {
        self.blog_id
    }
    pub fn get_tag_id(&self) -> u16 {
        self.tag_id
    }
    pub fn set_blog_id(&mut self, id: u16) -> &mut Self {
        self.blog_id = id;
        self
    }
    pub fn set_tag_id(&mut self, id: u16) -> &mut Self {
        self.tag_id = id;
        self
    }
}
crud!(BlogTag {});
