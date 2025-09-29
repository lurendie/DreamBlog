use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
};

use crate::entity::visitor;
use crate::error::DataBaseError;
use crate::model::Visitor;

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
        if visitor.id == 0 {
            let mut model = visitor::Model::from(visitor).into_active_model();
            model.not_set(visitor::Column::Id);
            model.save(db).await?;
        } else {
            visitor::Model::from(visitor)
                .into_active_model()
                .save(db)
                .await?;
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
