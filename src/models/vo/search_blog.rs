use rbatis::{crud, impl_select};
use serde::{Deserialize, Serialize};
/**
 * 文章搜索
 */
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SearchBlog {
    #[serde(rename(serialize = "id"))]
    id: u16,
    title: String,
    content: String,
}
impl SearchBlog {
    pub fn get_id(&self) -> u16 {
        self.id
    }
    pub fn set_id(&mut self, id: u16) {
        self.id = id;
    }
    pub fn get_title(&self) -> String {
        self.title.clone()
    }
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }
    pub fn get_content(&self) -> String {
        self.content.clone()
    }
    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }
}
crud!(SearchBlog {}, "blog");
impl_select!(SearchBlog {select_by_title(title:&str)->Vec=>"`where blog.content like #{title}`"},"blog");
