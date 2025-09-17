use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Categorie {
    id: Option<u16>,
    name: String,
    blogs: Vec<Categorie>,
}
impl Categorie {
    pub fn new(id: Option<u16>, name: String, blogs: Vec<Categorie>) -> Categorie {
        Categorie { id, name, blogs }
    }
}
