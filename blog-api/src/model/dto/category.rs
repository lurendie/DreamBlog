use crate::entity::category;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Category {
    id: i64,
    // #[serde(rename(deserialize = "category_name"))]
    name: String,
}

impl Category {
    // pub fn new(id: i64, name: String) -> Self {
    //     Self { id, name }
    // }

    pub fn get_id(&self) -> i64 {
        self.id
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }

    // pub fn set_name(&mut self, name: String) {
    //     self.name = name;
    // }

    // pub fn set_id(&mut self, id: i64) {
    //     self.id = id;
    // }
}

impl From<category::Model> for Category {
    fn from(model: category::Model) -> Self {
        Self {
            id: model.id,
            name: model.category_name,
        }
    }
}
