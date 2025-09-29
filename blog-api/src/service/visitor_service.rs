use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
};

use crate::constant::RedisKeyConstant;
use crate::entity::visitor;
use crate::error::DataBaseError;
use crate::model::Visitor;
use crate::service::RedisService;

pub struct VisitorService;

impl VisitorService {
    /**
     * 根据UUID获取访客
     */
    pub async fn get_by_uuid(uuid: &str, db: &DatabaseConnection) -> Option<Visitor> {
        match visitor::Entity::find()
            .filter(visitor::Column::Uuid.eq(uuid))
            .one(db)
            .await
            .ok()
        {
            Some(visitor) => match visitor {
                Some(visitor) => Some(visitor.into()),
                None => None,
            },
            None => None,
        }
    }
    /**
     * 保存访客
     */
    pub async fn save_visitor(
        visitor: Visitor,
        db: &DatabaseConnection,
    ) -> Result<(), DataBaseError> {
        //记录访客缓存
        let flag =
            RedisService::sismember(RedisKeyConstant::IDENTIFICATION_SET, &visitor.uuid).await?;
        if !flag {
            //查询UUID是否存在
            match VisitorService::get_by_uuid(&visitor.uuid, db).await {
                Some(model) => {
                    //查询到 重新缓存数据
                    RedisService::sadd(
                        RedisKeyConstant::IDENTIFICATION_SET.to_string(),
                        model.uuid,
                    )
                    .await?;
                }
                None => {
                    //查询不到新增数据并缓存
                    RedisService::sadd(
                        RedisKeyConstant::IDENTIFICATION_SET.to_string(),
                        visitor.uuid.clone(),
                    )
                    .await?;
                    let mut model = visitor::Model::from(visitor).into_active_model();
                    model.not_set(visitor::Column::Id);
                    model.save(db).await?;
                }
            };
        }
        Ok(())
    }

    /**
     * 根据id删除访客
     */
    pub async fn _delete_visitor(id: i64, db: &DatabaseConnection) {
        match visitor::Entity::delete_by_id(id).exec(db).await {
            Ok(_) => (),
            Err(e) => {
                log::error!("delete visitor error: {e}");
            }
        };
    }
}
