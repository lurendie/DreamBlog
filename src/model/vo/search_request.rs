/*
 * @Author: lurendie
 * @Date: 2024-04-06 09:12:34
 * @LastEditors: lurendie
 * @LastEditTime: 2024-04-23 22:57:39
 * @FilePath: \zero_blog\src\models\vo\page_request.rs
 */
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SearchRequest {
    #[serde(rename = "pageNum")]
    page_num: Option<u64>,
    #[serde(rename = "pageSize")]
    page_size: Option<u64>,
    #[serde(rename = "page")]
    page: Option<u64>,
    #[serde(rename = "blogId")]
    blog_id: Option<i64>,
    #[serde(rename = "password")]
    password: Option<String>,
    #[serde(rename = "title")]
    title: Option<String>,
    #[serde(rename = "categoryId", default)]
    category_id: Option<String>,
}

impl Default for SearchRequest {
    fn default() -> Self {
        Self {
            page_num: Some(1),
            page_size: Some(10),
            page: None,
            blog_id: None,
            password: None,
            title: None,
            category_id: None,
        }
    }
}

impl SearchRequest {
    pub fn get_page_num(&self) -> u64 {
        self.page_num.unwrap_or_default()
    }
    pub fn get_page_size(&self) -> u64 {
        self.page_size.unwrap_or_default()
    }
    pub fn get_blog_id(&self) -> i64 {
        self.blog_id.unwrap_or_default()
    }
    // pub fn get_page(&self) -> u64 {
    //     self.page.unwrap_or_default()
    // }
    pub fn get_password(&self) -> String {
        self.password.clone().unwrap_or_default()
    }
    pub fn get_title(&self) -> Option<String> {
        self.title.clone()
    }

    pub fn set_title(&mut self, title: Option<String>) {
        self.title = title;
    }

    pub fn set_page_num(&mut self, page_num: Option<u64>) {
        self.page_num = page_num;
    }

    pub fn set_page_size(&mut self, page_size: Option<u64>) {
        self.page_size = page_size;
    }

    // pub fn set_blog_id(&mut self, blog_id: Option<i64>) {
    //     self.blog_id = blog_id;
    // }

    // pub fn set_page(&mut self, page: Option<u64>) {
    //     self.page = page;
    // }

    // pub fn set_password(&mut self, password: Option<String>) {
    //     self.password = password;
    // }

    // pub fn set_category_id(&mut self, category_id: Option<i64>) {
    //     self.category_id = Some(category_id.unwrap_or_default().to_string());
    // }

    pub fn get_category_id(&self) -> Option<i64> {
        self.category_id.clone().unwrap_or_default().parse().ok()
    }
}
