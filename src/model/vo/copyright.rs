use serde::{Deserialize, Serialize};

#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct Copyright{
    pub title:String,
    #[serde(rename="siteName")]
    pub site_name:String
}