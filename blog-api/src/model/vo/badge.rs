use serde::{Deserialize, Serialize};

//GitHub徽标
#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct Badge{
    pub color:String,
    pub subject:String,
    pub title:String,
    pub url:String,
    pub value:String
}