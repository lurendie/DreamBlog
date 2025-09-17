use rbatis::executor::RBatisTxExecutor;

use crate::models::blog_tag::BlogTag;

pub struct BlogTagService;

impl BlogTagService {
    /**
     * 添加标签
     */
    pub async fn inser_tags(tags: Vec<u16>, blog_id: u16, tx: &mut RBatisTxExecutor) -> bool {
        for tag_id in tags {
            let blog_tag = BlogTag::from(blog_id, tag_id);
            let ok = BlogTag::insert(tx, &blog_tag).await.is_ok();
            if !ok {
                return false;
            }
        }
        true
    }

    /**
     * 删除标签
     */
    pub async fn remove_tags(tags: Vec<u16>, blog_id: u16, tx: &mut RBatisTxExecutor) -> bool {
        for tag_id in tags {
            let blog_tag = BlogTag::from(blog_id, tag_id);
            let ok = BlogTag::delete_by_column(tx, "tag_id", blog_tag.get_tag_id())
                .await
                .is_ok();
            if !ok {
                return false;
            }
        }
        true
    }
    /**
     * 删除某个blog的所有标签
     */
    pub(crate) async fn delete_tags_by_blog_id(id: u16, tx: &mut RBatisTxExecutor) -> bool {
        let mut delete_tag = BlogTag::new();
        delete_tag.set_blog_id(id);
        match BlogTag::delete_by_column(tx, "blog_id", id).await {
            Ok(_) => true,
            Err(e) => {
                log::error!("delete_tags_by_blog_id error: {}", e);
                tx.rollback().await.unwrap_or_else(|e| {
                    log::error!("rollback error: {}", e);
                });
                false
            }
        }
    }
}
