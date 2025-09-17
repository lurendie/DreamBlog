use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MomentDTO {
    pub(crate) id: Option<i64>,
    pub(crate) content: String,
    #[serde(rename(deserialize = "createTime"))]
    pub(crate) create_time: NaiveDateTime,
    pub(crate) likes: i32,
    pub(crate) is_published: bool,
}

// impl MomentDTO {
//     pub fn new(
//         content: Option<String>,
//         create_time: Option<String>,
//         likes: Option<u64>,
//         is_published: bool,
//     ) -> Self {
//         Self {
//             id: None,
//             content,
//             create_time,
//             likes,
//             is_published,
//         }
//     }

//     // pub fn set_id(&mut self, id: u16) {
//     //     self.id = Some(id);
//     // }
//     // pub fn get_id(&self) -> Option<u16> {
//     //     self.id
//     // }

//     // pub fn get_content(&self) -> Option<String> {
//     //     self.content.clone()
//     // }

//     // pub fn set_content(&mut self, content: String) {
//     //     self.content = Some(content);
//     // }

//     // pub fn get_create_time(&self) -> Option<String> {
//     //     self.create_time.clone()
//     // }

//     // pub fn set_create_time(&mut self, create_time: String) {
//     //     self.create_time = Some(create_time);
//     // }

//     // pub fn get_likes(&self) -> u64 {
//     //     self.likes.unwrap_or_default()
//     // }

//     // pub fn set_likes(&mut self, likes: u64) {
//     //     self.likes = Some(likes);
//     // }

//     // pub fn get_is_published(&self) -> bool {
//     //     self.is_published
//     // }

//     // pub fn set_is_published(&mut self, is_published: bool) {
//     //     self.is_published = is_published;
//     // }
// }
