use crate::models::blog::bool_from_int;
use rbatis::{crud, impl_select_page};
use serde::{Deserialize, Serialize};
//评论
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub(crate) id: Option<u16>,
    pub(crate) nickname: String,
    pub(crate) email: String,
    pub(crate) content: String,
    pub(crate) avatar: String,
    pub(crate) create_time: String,
    pub(crate) ip: String,
    #[serde(deserialize_with = "bool_from_int")]
    pub(crate) is_published: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub(crate) is_admin_comment: bool,
    pub(crate) page: u16,
    #[serde(deserialize_with = "bool_from_int")]
    pub(crate) is_notice: bool,
    pub(crate) blog_id: Option<u16>,
    pub(crate) parent_comment_id: Option<u16>,
    pub(crate) website: Option<String>,
    pub(crate) qq: Option<String>,
    pub(crate) comments: Option<Vec<Comment>>,
}
crud!(Comment {});
//分页查询
impl_select_page!(Comment{select_page(page:&str,blog_id:&str) => ""});
