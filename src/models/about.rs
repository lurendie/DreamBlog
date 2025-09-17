use rbatis::crud;
use serde::{Deserialize, Serialize};
//关于
#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct About{
    id:Option<u16>, //id
    pub(crate) name_en :String, 
    name_zh :String,
    pub(crate)value :String, 
}
crud!(About {});