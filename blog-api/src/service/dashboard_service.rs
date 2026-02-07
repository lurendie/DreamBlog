use rbs::value::map::ValueMap;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};
use chrono::Local;

use crate::entity::{blog, comment, city_visitor, visit_record};
use crate::model::{CityVisitor, VisitRecord, VisitRecordChart};
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

    /**
     * 获取今日PV/UV
     */
    pub async fn get_today_pv_uv(db: &DatabaseConnection) -> (i32, i32) {
        let today = Local::now().format("%m-%d").to_string();
        let record = visit_record::Entity::find()
            .filter(visit_record::Column::Date.eq(today))
            .one(db)
            .await
            .unwrap_or(None);
        match record {
            Some(model) => (model.pv, model.uv),
            None => (0, 0),
        }
    }

    /**
     * 获取近一周访问记录
     */
    pub async fn get_visit_record_chart(db: &DatabaseConnection) -> VisitRecordChart {
        let mut models = visit_record::Entity::find()
            .order_by_desc(visit_record::Column::Id)
            .limit(7)
            .all(db)
            .await
            .unwrap_or_default();
        models.reverse();
        let records = models.into_iter().map(VisitRecord::from).collect();
        VisitRecordChart::from_records(records)
    }

    /**
     * 获取城市访客数据
     */
    pub async fn get_city_visitor_list(db: &DatabaseConnection) -> Vec<CityVisitor> {
        let models = city_visitor::Entity::find()
            .order_by_desc(city_visitor::Column::Uv)
            .all(db)
            .await
            .unwrap_or_default();
        models.into_iter().map(CityVisitor::from).collect()
    }
}
