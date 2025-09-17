use serde::{Deserialize, Serialize};
use rbatis::{crud, impl_select};
#[derive(Debug, Clone,Serialize,Deserialize)]
//友链
pub(crate) struct FriendInfo{
     nickname:String,
     description:String,
     website:String,
     avatar:String,
}
crud!(FriendInfo{},"friend");
impl_select!(FriendInfo{select_all_published(is_published:&str) => "`where is_published = #{is_published}`"},"friend");