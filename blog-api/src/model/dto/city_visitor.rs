use crate::entity::city_visitor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CityVisitor {
    pub city: String,
    pub uv: i32,
}

impl From<city_visitor::Model> for CityVisitor {
    fn from(value: city_visitor::Model) -> Self {
        Self {
            city: value.city,
            uv: value.uv,
        }
    }
}
