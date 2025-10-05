use std::io;

use crate::entity::user;
use chrono::NaiveDateTime;
use deadpool_redis::redis::FromRedisValue;
use deadpool_redis::redis::Value;
use serde::{Deserialize, Serialize};
/*
 * @Author: lurendie
 * @Date: 2024-02-24 22:58:03
 * @LastEditors: lurendie
 * @LastEditTime: 2024-05-12 23:18:00
 */
#[derive(Serialize, Deserialize, Debug, Default, Hash, PartialEq, Eq, Clone)]
pub struct User {
    id: i64,
    username: String,           //用户名
    password: String,           //密码
    nickname: String,           //昵称
    avatar: String,             //头像
    email: String,              //邮箱
    create_time: NaiveDateTime, //创建时间
    update_time: NaiveDateTime, //更新时间
    role: String,               //角色访问权限
}

impl User {
    pub fn get_id(&self) -> i64 {
        self.id
    }

    pub fn get_username(&self) -> String {
        self.username.clone()
    }

    pub fn get_password(&self) -> String {
        self.password.clone()
    }

    pub fn set_password(&mut self, pass: String) {
        self.password = pass
    }

    pub fn get_role(&self) -> String {
        self.role.clone()
    }

    pub fn get_nickname(&self) -> String {
        self.nickname.clone()
    }

    pub fn get_avatar(&self) -> String {
        self.avatar.clone()
    }
    pub fn get_email(&self) -> String {
        self.email.clone()
    }
}

impl From<user::Model> for User {
    fn from(model: user::Model) -> Self {
        Self {
            id: model.id,
            username: model.username,
            password: model.password,
            nickname: model.nickname,
            avatar: model.avatar,
            email: model.email,
            create_time: model.create_time,
            update_time: model.update_time,
            role: model.role,
        }
    }
}

#[derive(Deserialize)]
pub struct LoginUser {
    pub(crate) username: String,
    pub(crate) password: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Hash, PartialEq, Eq, Clone)]
pub struct LoginedCacheUser {
    pub cache_info: CacheUserInfo,
    pub password: String,
    pub uuid: String,
}

impl LoginedCacheUser {
    pub fn new(cache_info: CacheUserInfo, password: &str, uuid: &str) -> Self {
        Self {
            cache_info: cache_info,
            password: password.to_string(),
            uuid: uuid.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Hash, PartialEq, Eq, Clone)]
pub struct CacheUserInfo {
    pub user: User,
    pub token: String,
}

impl CacheUserInfo {
    pub fn new(user: User, token: &str) -> Self {
        Self {
            user: user,
            token: token.to_string(),
        }
    }
}

impl FromRedisValue for LoginedCacheUser {
    fn from_redis_values(
        items: &[deadpool_redis::redis::Value],
    ) -> deadpool_redis::redis::RedisResult<Vec<Self>> {
        items.iter().map(FromRedisValue::from_redis_value).collect()
    }

    fn from_byte_vec(_vec: &[u8]) -> Option<Vec<Self>> {
        Self::from_redis_value(&deadpool_redis::redis::Value::Data(_vec.into()))
            .map(|rv| std::vec![rv])
            .ok()
    }

    fn from_redis_value(v: &Value) -> deadpool_redis::redis::RedisResult<Self> {
        match v {
            Value::Data(data) => Ok(serde_json::from_slice(data)?),
            _ => Err(deadpool_redis::redis::RedisError::from(io::Error::other(
                "Invalid data type",
            ))),
        }
    }
}
