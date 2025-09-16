use std::collections::HashMap;

use crate::entity::friend;
use crate::middleware::AppClaims;
use crate::model::Friend;
use crate::service::FriendService;
use crate::{app_state::AppState, model::ApiResponse};
use actix_jwt_session::Authenticated;
use actix_web::{routes, web, Responder};
use chrono::Utc;
use rbs::to_value;
use sea_orm::{ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, EntityTrait, QueryFilter, Set};
use sea_orm::{PaginatorTrait, QueryOrder, QuerySelect};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FriendQuery {
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
    pub nickname: Option<String>,
    pub is_published: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct FriendUpdatePublished {
    pub id: i64,
    pub published: bool,
}

#[routes]
#[get("/friends")]
pub async fn get_friends_by_query(
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
    query: web::Query<FriendQuery>,
) -> impl Responder {
    let db = app.get_mysql_pool();
    let page_num = query.page_num.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(10);
    let offset = (page_num - 1) * page_size;

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
    let friends = query_builder
        .order_by_asc(friend::Column::Id)
        .limit(page_size as u64)
        .offset(offset as u64)
        .all(db)
        .await;

    match friends {
        Ok(friend_models) => {
            let mut friends = Vec::new();
            friend_models.into_iter().for_each(|item| {
                friends.push(Friend::from(item));
            });
            let mut result = HashMap::new();
            result.insert("total".to_string(), to_value!(total));
            result.insert("records".to_string(), to_value!(friends));
            ApiResponse::<String>::success_with_msg(
                "获取友链列表成功".to_string(),
                Some(to_value!(result).to_string()),
            )
            .json()
        }
        Err(e) => ApiResponse::<String>::error(format!("获取友链列表失败: {}", e)).json(),
    }
}

#[routes]
#[put("/friend/published")]
pub async fn update_friend_published(
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
    params: web::Query<FriendUpdatePublished>,
) -> impl Responder {
    let db = app.get_mysql_pool();
    let friend_id = params.id;

    let result = friend::Entity::find_by_id(friend_id).one(db).await;

    match result {
        Ok(Some(friend_model)) => {
            let mut active_friend: friend::ActiveModel = friend_model.into();
            active_friend.is_published = Set(params.published);

            match active_friend.update(db).await {
                Ok(_) => ApiResponse::<String>::success_with_msg(
                    "更新友链发布状态成功".to_string(),
                    None,
                )
                .json(),
                Err(e) => {
                    ApiResponse::<String>::error(format!("更新友链发布状态失败: {}", e)).json()
                }
            }
        }
        Ok(None) => ApiResponse::<String>::error("友链不存在".to_string()).json(),
        Err(e) => ApiResponse::<String>::error(format!("查询友链失败: {}", e)).json(),
    }
}

#[routes]
#[post("/friend")]
pub async fn save_friend(
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
    friend_form: web::Json<Friend>,
) -> impl Responder {
    let db = app.get_mysql_pool();
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

    match new_friend.insert(db).await {
        Ok(_) => ApiResponse::<String>::success_with_msg("添加友链成功".to_string(), None).json(),
        Err(e) => ApiResponse::<String>::error(format!("添加友链失败: {}", e)).json(),
    }
}

#[routes]
#[put("/friend")]
pub async fn update_friend(
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
    friend_form: web::Json<Friend>,
) -> impl Responder {
    let db = app.get_mysql_pool();

    if friend_form.id.is_none() {
        return ApiResponse::<String>::error("友链ID不能为空".to_string()).json();
    }

    let friend_id = friend_form.id.unwrap();

    let result = friend::Entity::find_by_id(friend_id).one(db).await;

    match result {
        Ok(Some(friend_model)) => {
            let mut active_friend: friend::ActiveModel = friend_model.into();
            active_friend.nickname = Set(friend_form.nickname.clone());
            active_friend.description = Set(friend_form.description.clone());
            active_friend.website = Set(friend_form.website.clone());
            active_friend.avatar = Set(friend_form.avatar.clone());
            active_friend.is_published = Set(friend_form.is_published);

            match active_friend.update(db).await {
                Ok(_) => {
                    ApiResponse::<String>::success_with_msg("更新友链成功".to_string(), None).json()
                }
                Err(e) => ApiResponse::<String>::error(format!("更新友链失败: {}", e)).json(),
            }
        }
        Ok(None) => ApiResponse::<String>::error("友链不存在".to_string()).json(),
        Err(e) => ApiResponse::<String>::error(format!("查询友链失败: {}", e)).json(),
    }
}

#[routes]
#[delete("/friend")]
pub async fn delete_friend_by_id(
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
    params: web::Query<IdParam>,
) -> impl Responder {
    let db = app.get_mysql_pool();
    let friend_id = params.id;

    match friend::Entity::delete_by_id(friend_id).exec(db).await {
        Ok(result) => {
            if result.rows_affected > 0 {
                ApiResponse::<String>::success_with_msg("删除友链成功".to_string(), None).json()
            } else {
                ApiResponse::<String>::error("友链不存在".to_string()).json()
            }
        }
        Err(e) => ApiResponse::<String>::error(format!("删除友链失败: {}", e)).json(),
    }
}

#[routes]
#[get("/friendInfo")]
pub async fn get_friend_info(
    _: Authenticated<AppClaims>,
    app: web::Data<AppState>,
) -> impl Responder {
    match FriendService::get_friend(app.get_mysql_pool()).await {
        Ok(data) => ApiResponse::<String>::success_with_msg(
            "获取友链信息成功".to_string(),
            Some(to_value!(data).to_string()),
        )
        .json(),
        Err(e) => ApiResponse::<String>::error(format!("获取友链信息失败: {}", e)).json(),
    }
}

#[routes]
#[put("/friendInfo/commentEnabled")]
pub async fn update_friend_comment_enabled(
    _: Authenticated<AppClaims>,
    _app: web::Data<AppState>,
) -> impl Responder {
    // 这里需要实现更新友链评论启用状态的逻辑
    // 由于服务层没有提供相应方法，这里先返回一个占位响应
    ApiResponse::<String>::success_with_msg("更新友链评论启用状态成功".to_string(), None).json()
}

#[routes]
#[put("/friendInfo/content")]
pub async fn update_friend_content(
    _: Authenticated<AppClaims>,
    _app: web::Data<AppState>,
    // _content_update: web::Json<FriendContentUpdate>,
) -> impl Responder {
    // 这里需要实现更新友链内容的逻辑
    // 由于服务层没有提供相应方法，这里先返回一个占位响应
    ApiResponse::<String>::success_with_msg("更新友链内容成功".to_string(), None).json()
}

#[derive(Debug, Deserialize)]
pub struct IdParam {
    pub id: i64,
}
