use rbs::value::map::ValueMap;

use crate::service::{BlogService, CategoryService, CommentService, TagService};

/**
 * DashboardService 仪表盘
 */
pub struct DashboardService;

impl DashboardService {
    /**
     * 获取博文总数
     */
    pub async fn get_blog_count() -> i32 {
        BlogService::get_blog_count().await
    }
    /**
     * 获取评论总数
     */
    pub async fn get_comment_count() -> i32 {
        CommentService::get_comment_count().await
    }
    /**
     * 获取分类博文数量
     */
    pub async fn get_categorys_count() -> ValueMap {
        //获取分类博文数量
        CategoryService::get_categorys_count().await
    }

    /**
     * 获取标签博文数量
     */
    pub async fn get_tags_count() -> ValueMap {
        //获取分类博文数量
        TagService::get_tags_count().await
    }
}
