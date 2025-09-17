use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
pub struct Series {
    id: u16,
    name: String,
    value: i32,
}

impl Series {
    pub fn new(id: u16, name: String, value: i32) -> Self {
        Self { id, name, value }
    }
}
