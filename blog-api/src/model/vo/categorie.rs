use crate::entity::category;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Categorie {
    pub(crate) id: Option<i64>,
    #[serde(rename(deserialize = "category_name"))]
    pub(crate) name: String,
    #[serde(skip)]
    pub(crate) _blogs: Vec<Categorie>,
}

impl Categorie {
    pub fn new(id: Option<i64>, name: String, blogs: Vec<Categorie>) -> Categorie {
        Categorie {
            id,
            name,
            _blogs: blogs,
        }
    }
}

impl From<category::Model> for Categorie {
    fn from(model: category::Model) -> Self {
        Categorie {
            id: Some(model.id),
            name: model.category_name,
            _blogs: vec![],
        }
    }
}
