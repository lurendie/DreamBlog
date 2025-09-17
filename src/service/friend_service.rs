use crate::entity::site_setting;
use crate::enums::DataBaseError;
use crate::model::FriendInfo;
use crate::util::MarkdownParser;
use rbs::{to_value, value::map::ValueMap};
use sea_orm::ColumnTrait;
use sea_orm::DatabaseConnection;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use crate::entity::friend;

pub struct FriendService;

impl FriendService {
    //获取友链数据
    pub(crate) async fn get_friend(db: &DatabaseConnection) -> Result<ValueMap, DataBaseError> {
        let mut friend_map = ValueMap::new();
        let mut friend_info = ValueMap::new();
        let site_settings = site_setting::Entity::find()
            .filter(site_setting::Column::NameEn.contains("friend"))
            .all(db)
            .await?;

        site_settings.into_iter().for_each(|item| {
            if let Some(name) = item.name_en {
                if name.contains("friendContent") {
                    friend_info.insert(
                        to_value!("content"),
                        to_value!(MarkdownParser::parser_html(item.value.unwrap_or_default())),
                    );
                } else if name.contains("friendCommentEnabled") {
                    friend_info.insert(
                        to_value!("commentEnabled"),
                        to_value!(item.value.unwrap_or_default() == "1"),
                    );
                }
            }
        });
        let models =friend::Entity::find().filter(friend::Column::IsPublished.eq(true)).all(db).await?;
        let mut friend_list =vec![];
        for model in models {
            friend_list.push(FriendInfo::from(model));
        }
        friend_map.insert(to_value!("friendInfo"), to_value!(friend_info));
        friend_map.insert(to_value!("friendList"), to_value!(friend_list));
        Ok(friend_map)
    }
}
