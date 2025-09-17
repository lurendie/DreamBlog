use crate::models::blog::bool_from_int;
use rbatis::{crud, impl_select, impl_select_page};
use serde::{Deserialize, Serialize};
//评论
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Comment {
    pub(crate) id: Option<u16>,
    pub(crate) nickname: String,
    pub(crate) content: String,
    pub(crate) avatar: String,
    #[serde(rename(serialize = "createTime"))]
    pub(crate) create_time: String,
    // #[serde(deserialize_with = "bool_from_int")]
    // pub(crate) is_published: bool,
    #[serde(deserialize_with = "bool_from_int", rename(serialize = "adminComment"))]
    pub(crate) is_admin_comment: bool,
    #[serde(rename(serialize = "parentCommentId"))]
    pub(crate) parent_comment_id: Option<i16>,
    #[serde(rename(serialize = "parentCommentNickname"))]
    pub(crate) parent_comment_name: Option<String>,
    pub(crate) website: Option<String>,
    //pub(crate) qq: Option<String>,
    #[serde(rename(serialize = "replyComments"))]
    pub(crate) reply_comments: Option<Vec<Comment>>,
}
crud!(Comment {});
//分页查询 获取公开的
impl_select_page!(Comment{select_page(page:&str,blog_id:&str) => "where blog_id =#{blog_id} and parent_comment_id = -1 and page =#{page} and is_published = 1"});
impl_select!(Comment{select_all_comment(blog_id:&str)=>"`where blog_id = #{blog_id}`"});
impl_select!(Comment{select_close_comment(blog_id:&str)=>"`where blog_id = #{blog_id} and is_published = 0`"});
