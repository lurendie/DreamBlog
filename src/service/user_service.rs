use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::entity::user;
use crate::enums::DataBaseError;
use crate::model::User;

pub struct UserService;

impl UserService {
    /**
     *根据Name获取User
     */
    pub async fn get_by_username(
        username: &String,
        db: &DatabaseConnection,
    ) -> Result<User, DataBaseError> {
        let user = user::Entity::find()
            .filter(user::Column::Username.eq(username))
            .one(db)
            .await?;
        if let Some(user) = user {
            return Ok(User::from(user));
        }
        Err(DataBaseError::Custom("没有检索到该用户".to_string()))
    }
}
