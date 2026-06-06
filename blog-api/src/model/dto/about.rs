use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AboutForm {
    pub title: String,
    #[serde(rename = "musicId")]
    pub music_id: Option<String>,
    pub content: String,
    #[serde(rename = "commentEnabled")]
    pub comment_enabled: bool,
}
