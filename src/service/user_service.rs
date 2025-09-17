use crate::dao::UserDao;
use crate::models::user::User;

pub struct UserService;

impl UserService {
    /**
     *根据Name获取User
     */
    pub async fn get_by_username(username: &String) -> Option<User> {
        let mut user_list = UserDao::get_by_username(&username).await;
        if let Ok(user_list) = user_list.as_mut() {
            if !user_list.is_empty() {
                let user = user_list.pop();
                return user;
            } else {
                return None;
            }
        }
        None
    }
}
