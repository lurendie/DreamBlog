use crate::entity::user;
use chrono::NaiveDateTime;
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

    pub fn get_create_time(&self) -> NaiveDateTime {
        self.create_time
    }
    pub fn get_update_time(&self) -> NaiveDateTime {
        self.update_time
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

/// 后台修改账户提交的字段（全部可选，只更新非空项）
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct UpdateAccountDTO {
    pub username: Option<String>,
    pub password: Option<String>,
    pub nickname: Option<String>,
    pub avatar: Option<String>,
    pub email: Option<String>,
}
