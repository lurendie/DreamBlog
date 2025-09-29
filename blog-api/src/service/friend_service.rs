use crate::common::MarkdownParser;
use crate::constant::RedisKeyConstant;
use crate::entity::friend;
use crate::entity::site_setting;
use crate::error::DataBaseError;
use crate::model::FriendInfo;
use crate::service::RedisService;
use rbs::{value, value::map::ValueMap};
use sea_orm::ColumnTrait;
use sea_orm::DatabaseConnection;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;

pub struct FriendService;

impl FriendService {
    //获取友链数据
    pub(crate) async fn get_friend(db: &DatabaseConnection) -> Result<ValueMap, DataBaseError> {
        if let Ok(map) =
            RedisService::get_value_map(RedisKeyConstant::FRIEND_INFO_MAP.to_string()).await
        {
            return Ok(map);
        }
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
                        value!("content"),
                        value!(MarkdownParser::parser_html(item.value.unwrap_or_default())),
                    );
                } else if name.contains("friendCommentEnabled") {
                    friend_info.insert(
                        value!("commentEnabled"),
                        value!(item.value.unwrap_or_default() == "1"),
                    );
                }
            }
        });
        let models = friend::Entity::find()
            .filter(friend::Column::IsPublished.eq(true))
            .all(db)
            .await?;
        let mut friend_list = vec![];
        for model in models {
            friend_list.push(FriendInfo::from(model));
        }
        friend_map.insert(value!("friendInfo"), value!(friend_info));
        friend_map.insert(value!("friendList"), value!(friend_list));
        if !friend_map.is_empty() {
            RedisService::set_value_map(RedisKeyConstant::FRIEND_INFO_MAP.to_string(), &friend_map)
                .await?;
        }
        Ok(friend_map)
    }
}
