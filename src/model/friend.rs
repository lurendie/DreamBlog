use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
//友链
pub struct Friend {
    pub(crate) id: Option<i64>,
    pub(crate) nickname: String,
    pub(crate) description: String,
    pub(crate) website: String,
    pub(crate) avatar: String,
    pub(crate) is_published: bool,
    pub(crate) views: i32,
    pub(crate) create_time: NaiveDateTime,
}
