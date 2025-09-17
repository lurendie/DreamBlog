use rbatis::crud;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Category {
    pub id: u16,
   // #[serde(rename(deserialize = "category_name"))]
    pub name: String,
}

crud!(Category {});
