use crate::dao::site_setting_dao;
use crate::dao::FriendDao;
use rbs::{to_value, value::map::ValueMap};

pub struct FriendService;

impl FriendService {
    //获取友链数据
    pub(crate) async fn get_friend() -> ValueMap {
        let mut friend_map = ValueMap::new();
        let mut friend_info = ValueMap::new();
        let friends = site_setting_dao::get_list()
            .await
            .into_iter()
            .filter(|item| {
                item.name_en == "friendContent" || item.name_en == "friendCommentEnabled"
            })
            .collect::<Vec<_>>();

        friends.iter().for_each(|item| {
            if item.name_en == "friendContent" {
                friend_info.insert(
                    to_value!("content"),
                    to_value!(markdown::to_html(&item.value)),
                );
            }
            if item.name_en == "friendCommentEnabled" {
                friend_info.insert(to_value!("commentEnabled"), to_value!(item.value == "1"));
            };
        });
        let friend_list = FriendDao::get_friends().await.unwrap_or_else(|e| {
            log::error!("{}", e);
            vec![]
        });
        friend_map.insert(to_value!("friendInfo"), to_value!(friend_info));
        friend_map.insert(to_value!("friendList"), to_value!(friend_list));
        friend_map
    }
}
