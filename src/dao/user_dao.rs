use rbatis::Error;

use crate::models::user::User;
use crate::rbatis::RBATIS;

pub struct UserDao;
impl UserDao {
    pub async fn get_by_username(username: &String) -> Result<Vec<User>, Error> {
        let user =
            User::select_by_column(&RBATIS.acquire().await.unwrap(), "username", username).await;
        user
    }
}
