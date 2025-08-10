use rbs::value::map::ValueMap;
use sea_orm::{DatabaseConnection, EntityTrait, PaginatorTrait};

use crate::entity::{blog, comment};
use crate::service::{CategoryService, TagService};
/**
 * DashboardService 仪表盘
 */
pub struct DashboardService;

impl DashboardService {
    /**
     * 获取博文总数
     */
    pub async fn get_blog_count(db: &DatabaseConnection) -> u64 {
        blog::Entity::find().count(db).await.unwrap_or_default()
    }
    /**
     * 获取评论总数
     */
    pub async fn get_comment_count(db: &DatabaseConnection) -> u64 {
        comment::Entity::find().count(db).await.unwrap_or_default()
    }
    /**
     * 获取分类博文数量
     */
    pub async fn get_categorys_count(db: &DatabaseConnection) -> ValueMap {
        //获取分类博文数量
        CategoryService::get_series(db).await
    }

    /**
     * 获取标签博文数量
     */
    pub async fn get_tags_count(db: &DatabaseConnection) -> ValueMap {
        //获取分类博文数量
        TagService::get_tags_count(db).await
    }
}
