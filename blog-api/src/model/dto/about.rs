
use serde::{Deserialize, Serialize};
//关于
#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct About{
    id:Option<u16>, //id
    pub(crate) name_en :String, 
    name_zh :String,
    pub(crate)value :String, 
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AboutForm {
    pub title: String,
    #[serde(rename = "musicId")]
    pub music_id: Option<String>,
    pub content: String,
    #[serde(rename = "commentEnabled")]
    pub comment_enabled: bool,
}
