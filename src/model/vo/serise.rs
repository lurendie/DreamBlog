use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
pub struct Serise {
    id: i64,
    name: String,
    value: u64,
}

impl Serise {
    pub fn new(id: i64, name: String, value: u64) -> Self {
        Self { id, name, value }
    }
}
