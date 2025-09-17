use crate::models::blog_tag::BlogTag;
use crate::rbatis::RBATIS;
use rbatis::rbdc::Error;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct BlogTagDao;

impl BlogTagDao {
    // pub fn new() -> Self {
    //     BlogTagDao::default()
    // }
    /**
     * 根据ID查询出标签
     */
    pub async fn get_tags_by_blog_id(id: u16) -> Result<Vec<BlogTag>, Error> {
        BlogTag::select_by_column(
            &RBATIS.acquire().await.unwrap(),
            "blog_id",
            id.to_string().as_str(),
        )
        .await
    }
}
