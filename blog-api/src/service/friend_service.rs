use crate::common::MarkdownParser;
use crate::constant::RedisKeyConstant;
use crate::entity::friend;
use crate::entity::site_setting;
use crate::error::DataBaseError;
use crate::model::Friend;
use crate::model::FriendInfo;
use crate::model::FriendQuery;
use crate::model::FriendUpdatePublished;
use crate::service::RedisService;
use chrono::Utc;
use rbs::{value, value::map::ValueMap};
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::NotSet;
use sea_orm::ActiveValue::Set;
use sea_orm::ColumnTrait;
use sea_orm::DatabaseConnection;
use sea_orm::EntityTrait;
use sea_orm::PaginatorTrait;
use sea_orm::QueryFilter;
use sea_orm::QueryOrder;
use sea_orm::QuerySelect;

pub struct FriendService;

impl FriendService {
    //获取友链数据
    pub(crate) async fn get_friend(db: &DatabaseConnection) -> Result<ValueMap, DataBaseError> {
        if let Ok(map) =
            RedisService::get_string(RedisKeyConstant::FRIEND_INFO_MAP.to_string()).await
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
            RedisService::set_string(RedisKeyConstant::FRIEND_INFO_MAP.to_string(), &friend_map)
                .await?;
        }
        Ok(friend_map)
    }

    pub async fn update_published(
        friend: FriendUpdatePublished,
        db: &DatabaseConnection,
    ) -> Result<(), DataBaseError> {
        let friend_model = friend::Entity::find_by_id(friend.id)
            .one(db)
            .await?
            .ok_or_else(|| DataBaseError::Custom("友链不存在".to_string()))?;
        let mut active_friend: friend::ActiveModel = friend_model.into();
        active_friend.is_published = Set(friend.published);
        active_friend.update(db).await?;
        Ok(())
    }

    pub async fn delete_friend(id: i64, db: &DatabaseConnection) -> Result<(), DataBaseError> {
        friend::Entity::delete_by_id(id).exec(db).await?;
        Ok(())
    }

    pub async fn update_friend(
        id: i64,
        friend_form: Friend,
        db: &DatabaseConnection,
    ) -> Result<(), DataBaseError> {
        let result = friend::Entity::find_by_id(id).one(db).await?;
        match result {
            Some(friend_model) => {
                let mut active_friend: friend::ActiveModel = friend_model.into();
                active_friend.nickname = Set(friend_form.nickname.clone());
                active_friend.description = Set(friend_form.description.clone());
                active_friend.website = Set(friend_form.website.clone());
                active_friend.avatar = Set(friend_form.avatar.clone());
                active_friend.is_published = Set(friend_form.is_published);

                active_friend.update(db).await?;
            }
            None => return Err(DataBaseError::Custom("友链不存在".to_string())),
        }

        Ok(())
    }

    pub async fn save_friend(
        friend_form: Friend,
        db: &DatabaseConnection,
    ) -> Result<(), DataBaseError> {
        let now = Utc::now().naive_utc();
        let new_friend = friend::ActiveModel {
            id: NotSet,
            nickname: Set(friend_form.nickname.clone()),
            description: Set(friend_form.description.clone()),
            website: Set(friend_form.website.clone()),
            avatar: Set(friend_form.avatar.clone()),
            is_published: Set(friend_form.is_published),
            views: Set(0),
            create_time: Set(now),
        };

        new_friend.insert(db).await?;
        Ok(())
    }

    pub async fn friends_by_query(
        db: &DatabaseConnection,
        page_num: u32,
        page_size: u32,
        query: FriendQuery,
    ) -> Result<(Vec<Friend>, u64), DataBaseError> {
        // 构建查询条件
        let mut query_builder = friend::Entity::find();

        if let Some(nickname) = &query.nickname {
            query_builder = query_builder.filter(friend::Column::Nickname.contains(nickname));
        }

        if let Some(is_published) = query.is_published {
            query_builder = query_builder.filter(friend::Column::IsPublished.eq(is_published));
        }

        // 获取总数
        let total = query_builder.clone().count(db).await.unwrap_or(0);

        // 分页查询
        let friend_models = query_builder
            .order_by_asc(friend::Column::Id)
            .limit(page_size as u64)
            .offset(page_num as u64)
            .all(db)
            .await?;
        let mut friends = Vec::new();
        friend_models.into_iter().for_each(|item| {
            friends.push(Friend::from(item));
        });
        Ok((friends, total))
    }
}
