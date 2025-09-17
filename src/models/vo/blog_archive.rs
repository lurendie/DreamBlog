use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct BlogArchive {
    pub id: String,
    pub password: String,
    pub privacy: bool,
    pub title: String,
    pub day: String,
}

impl BlogArchive {
    pub(crate) fn new() -> Self {
        BlogArchive {
            id: "".to_string(),
            password: "".to_string(),
            privacy: false,
            title: "".to_string(),
            day: "".to_string(),
        }
    }
}
