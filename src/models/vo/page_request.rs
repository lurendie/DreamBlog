/*
 * @Author: lurendie
 * @Date: 2024-04-06 09:12:34
 * @LastEditors: lurendie
 * @LastEditTime: 2024-04-23 22:57:39
 * @FilePath: \zero_blog\src\models\vo\page_request.rs
 */
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Default, Serialize, Clone, Debug)]
pub struct SearchRequest {
    #[serde(rename = "pageNum")]
    page_num: Option<u16>,
    #[serde(rename = "pageSize")]
    page_size: Option<u16>,
    #[serde(rename = "page")]
    page: Option<u16>,
    #[serde(rename = "blogId")]
    blog_id: Option<u16>,
    #[serde(rename = "password")]
    password: Option<String>,
    #[serde(rename = "title")]
    title: Option<String>,
    #[serde(rename = "categoryId")]
    category_id: Option<u16>,
}

impl SearchRequest {
    pub fn get_page_num(&self) -> u16 {
        self.page_num.unwrap_or_default()
    }
    pub fn get_page_size(&self) -> u16 {
        self.page_size.unwrap_or_default()
    }
    pub fn get_blog_id(&self) -> u16 {
        self.blog_id.unwrap_or_default()
    }
    pub fn get_page(&self) -> u16 {
        self.page.unwrap_or_default()
    }
    pub fn get_password(&self) -> String {
        self.password.clone().unwrap_or_default()
    }
    pub fn get_title(&self) -> String {
        self.title.clone().unwrap_or_default()
    }

    pub fn set_title(&mut self, title: Option<String>) {
        self.title = title;
    }

    pub fn set_page_num(&mut self, page_num: Option<u16>) {
        self.page_num = page_num;
    }

    pub fn set_page_size(&mut self, page_size: Option<u16>) {
        self.page_size = page_size;
    }

    pub fn set_blog_id(&mut self, blog_id: Option<u16>) {
        self.blog_id = blog_id;
    }

    pub fn set_page(&mut self, page: Option<u16>) {
        self.page = page;
    }

    pub fn set_password(&mut self, password: Option<String>) {
        self.password = password;
    }

    pub fn set_category_id(&mut self, category_id: u16) {
        self.category_id = Some(category_id);
    }

    pub fn get_category_id(&self) -> u16 {
        self.category_id.unwrap_or(0)
    }
}
