use rbatis::rbdc::datetime::DateTime;
use rbatis::{crud, impl_select, impl_select_page};
use serde::de::Unexpected;
use serde::{Deserialize, Deserializer, Serialize};

use crate::dao::CategoryDao;
use crate::models::category::Category;
//博客详情信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BlogDetail {
    pub(crate) id: Option<u16>,
    pub(crate) title: String,
    pub(crate) content: String,
    #[serde(deserialize_with = "bool_from_int")]
    pub(crate) is_appreciation: bool,
    #[serde(
        rename(serialize = "commentEnabled"),
        deserialize_with = "bool_from_int"
    )]
    pub(crate) is_comment_enabled: bool,
    #[serde(rename(serialize = "createTime"))]
    pub(crate) create_time: DateTime,
    #[serde(rename(serialize = "updateTime"))]
    pub(crate) update_time: DateTime,
    pub(crate) views: u16,
    pub(crate) words: u16,
    #[serde(rename(serialize = "readTime"))]
    pub(crate) read_time: u16,
    #[serde(deserialize_with = "bool_from_int")]
    pub(crate) is_top: bool,

    pub(crate) password: Option<String>,
}

impl BlogDetail {
    pub(crate) fn new() -> Self {
        BlogDetail::default()
    }
}

// int 类型转boolean
fn bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match u64::deserialize(deserializer)? {
        0 => Ok(false),
        1 => Ok(true),
        other => Err(serde::de::Error::invalid_value(
            Unexpected::Unsigned(other),
            &"0 or 1",
        )),
    }
}

// // id 类型转 category
fn _category_from_id<'de, D>(deserializer: D) -> Result<Option<Category>, D::Error>
where
    D: Deserializer<'de>,
{
    if let Ok(id) = u16::deserialize(deserializer) {
        let fut = CategoryDao::get_by_id(id as u16);
        println!("id: {:?}", id);
        // let category = category_dao::get_by_id(id as u16).await.unwrap();
        // let mut category = Category::default();
        let _ = Box::pin(async move { return Some(fut.await.unwrap()) });
    }
    Ok(Some(Category::default()))
}

crud!(BlogDetail {}, "blog");
impl_select!(BlogDetail{select_by_id(id:String) -> Option => "`where id = #{id} limit 1`"},"blog");

impl_select_page!(BlogDetail{select_page_blog_all(title:&str) =>"where 1=1
if !title.is_empty():
   `and title like #{title}`"}, "blog");
