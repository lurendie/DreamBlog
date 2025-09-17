use rbatis::{crud, impl_select_page};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Moment {
    id: Option<u64>,
    pub(crate) content: String,
    #[serde(rename(serialize = "createTime"))]
    pub(crate) create_time: String,
    likes: u64,
    #[serde(rename(serialize = "published"))]
    is_published: u8,
}
crud!(Moment {}, "moment");
impl_select_page!(Moment{select_page() => "`where is_published = 1`"});
