use serde::{Deserialize, Serialize};
use rbatis::crud;
#[derive(Debug, Clone,Serialize,Deserialize)]
//友链
pub(crate) struct Friend{
     id :Option<u16>,
     nickname:String,
     description:String,
     website:String,
     avatar:String,
     is_published:u8,
     views:u16,
     create_time:String
}
crud!(Friend{});