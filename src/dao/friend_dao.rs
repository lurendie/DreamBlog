use crate::models::vo::friend_info::FriendInfo;
use crate::rbatis::RBATIS;
use rbatis::Error;

pub struct FriendDao;
impl FriendDao {
    pub(crate) async fn get_friends() -> Result<Vec<FriendInfo>, Error> {
        FriendInfo::select_all_published(&RBATIS.acquire().await.unwrap(), "1").await
    }
}
