use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FromQueryResult)]
pub struct BlogIdAndTitle {
    pub id: i64,
    pub title: String,
}
