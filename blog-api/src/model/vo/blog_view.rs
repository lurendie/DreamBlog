use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlogView {
    pub id: i64,
    pub views: i32,
}

impl BlogView {
    pub fn new(id: i64, views: i32) -> Self {
        Self { id, views }
    }
}
