use rbatis::{crud, impl_select_page};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MomentDTO {
    id: Option<u64>,
    pub(crate) content: Option<String>,
    #[serde(rename(deserialize = "createTime"))]
    pub(crate) create_time: Option<String>,
    pub(crate) likes: Option<u64>,
    #[serde(rename(deserialize = "published"))]
    pub(crate) is_published: Option<u8>,
}

impl MomentDTO {
    pub fn new(
        content: Option<String>,
        create_time: Option<String>,
        likes: Option<u64>,
        is_published: Option<u8>,
    ) -> Self {
        Self {
            id: None,
            content,
            create_time,
            likes,
            is_published,
        }
    }

    pub fn set_id(&mut self, id: u64) {
        self.id = Some(id);
    }   
    pub fn get_id(&self) -> Option<u64> {
        self.id
    }

    pub fn get_content(&self) -> Option<String> {
        self.content.clone()
    }


    pub fn set_content(&mut self, content: String) {
        self.content = Some(content);
    }

    pub fn get_create_time(&self) -> Option<String> {
        self.create_time.clone()
    }

    pub fn set_create_time(&mut self, create_time: String) {
        self.create_time = Some(create_time);
    }

    pub fn get_likes(&self) -> u64 {
        self.likes.unwrap_or_default()
    }

    pub fn set_likes(&mut self, likes: u64) {
        self.likes = Some(likes);
    }

    pub fn get_is_published(&self) -> Option<u8> {
        self.is_published
    }

    pub fn set_is_published(&mut self, is_published: u8) {
        self.is_published = Some(is_published);
    }
}
crud!(MomentDTO {}, "moment");
impl_select_page!(MomentDTO{select_page() => "`where is_published = 1`"});
