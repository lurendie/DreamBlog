use crate::entity::comment;
use crate::enums::DataBaseError;
use crate::model::{CommentDTO, CommentVO};
use crate::service::BlogService;
use rbs::to_value;
use rbs::value::map::ValueMap;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel,
    PaginatorTrait, QueryFilter,
};
//每页显示5条博客简介
const PAGE_SIZE: u64 = 5;

pub struct CommentService;

impl CommentService {
    //分页评论
    pub(crate) async fn find_by_id_comments(
        page_num: u64,
        blog_id: i64,
        db: &DatabaseConnection,
    ) -> Result<ValueMap, DataBaseError> {
        let mut map = ValueMap::new();
        let page = comment::Entity::find()
            .filter(comment::Column::BlogId.eq(blog_id))
            .filter(comment::Column::IsPublished.eq(true))
            .filter(comment::Column::ParentCommentId.eq(-1))
            .paginate(db, PAGE_SIZE);
        let models = page.fetch_page(page_num - 1).await?;
        let mut comments = vec![];
        for model in models.into_iter() {
            let id = model.id;
            let mut comment = CommentVO::from(model);
            comment.reply_comments = Some(Self::find_comment_vo_by_id(id, db).await?);
            comments.push(comment);
        }
        map.insert("list".into(), to_value!(comments));
        map.insert("totalPage".into(), rbs::Value::U64(page.num_pages().await?));

        Ok(map)
    }

    //分页评论
    pub(crate) async fn find_comment_dto(
        page_num: u64,
        page_size: u64,
        db: &DatabaseConnection,
    ) -> Result<ValueMap, DataBaseError> {
        let mut map = ValueMap::new();
        let page = comment::Entity::find().paginate(db, page_size);
        let models = page.fetch_page(page_num - 1).await?;
        let mut comments = vec![];
        for model in models.into_iter() {
            let blog_id = model.blog_id.unwrap_or_default();

            let mut comment = CommentDTO::from(model);
            comment.blog_id_and_title =
                Some(BlogService::find_blog_id_and_title(db, blog_id).await?);
            comments.push(comment);
        }
        map.insert(
            to_value!("pageNum"),
            to_value!(page.num_pages().await.unwrap_or_default()),
        );
        map.insert(to_value!("pageSize"), to_value!(PAGE_SIZE));
        map.insert(
            to_value!("pages"),
            to_value!(page.num_pages().await.unwrap_or_default()),
        );
        map.insert(
            to_value!("total"),
            to_value!(page.num_items().await.unwrap_or_default()),
        );
        map.insert("list".into(), to_value!(comments));

        Ok(map)
    }

    // pub(crate) async fn _find_comment_by_id(
    //     id: i64,
    //     db: &DatabaseConnection,
    // ) -> Result<Vec<Comment>, DataBaseError> {
    //     let models = comment::Entity::find()
    //         .filter(comment::Column::ParentCommentId.eq(id))
    //         .filter(comment::Column::IsPublished.eq(true))
    //         .all(db)
    //         .await?;

    //     let mut futures = Vec::new();
    //     let mut comments = vec![];
    //     for item in models.into_iter() {
    //         // 使用 Box::pin 来递归调用 get_comments，允许存在递归
    //         let future = Box::pin(Self::find_comment_by_id(item.id, db));
    //         futures.push(future);
    //         comments.push(Comment::from(item));
    //     }
    //     let mut reply_comments = vec![];
    //     // 处理子评论
    //     for (item, future) in comments.iter_mut().zip(futures) {
    //         if let Ok(future) = future.await.as_mut() {
    //             // match item.parent_comment_id {
    //             //     Some(parent_comment_id) => {
    //             //         let parent_comment = comment::Entity::find_by_id(parent_comment_id)
    //             //             .one(db)
    //             //             .await?;
    //             //         // if let Some(parent_comment) = parent_comment {
    //             //         //     item.parent_comment_name = Some(parent_comment.nickname);
    //             //         // }
    //             //     }
    //             //     None => {}
    //             // }

    //             reply_comments.push(item.to_owned());
    //             reply_comments.append(future);
    //         }
    //     }
    //     Ok(reply_comments)
    // }

    pub(crate) async fn find_comment_vo_by_id(
        id: i64,
        db: &DatabaseConnection,
    ) -> Result<Vec<CommentVO>, DataBaseError> {
        let models = comment::Entity::find()
            .filter(comment::Column::ParentCommentId.eq(id))
            .filter(comment::Column::IsPublished.eq(true))
            .all(db)
            .await?;

        let mut futures = Vec::new();
        let mut comments = vec![];
        for item in models.into_iter() {
            // 使用 Box::pin 来递归调用 get_comments，允许存在递归
            let future = Box::pin(Self::find_comment_vo_by_id(item.id, db));
            futures.push(future);
            comments.push(CommentVO::from(item));
        }
        let mut reply_comments = vec![];
        // 处理子评论
        for (item, future) in comments.iter_mut().zip(futures) {
            if let Ok(future) = future.await.as_mut() {
                match item.parent_comment_id {
                    Some(parent_comment_id) => {
                        let parent_comment = comment::Entity::find_by_id(parent_comment_id)
                            .one(db)
                            .await?;
                        if let Some(parent_comment) = parent_comment {
                            item.parent_comment_name = Some(parent_comment.nickname);
                        }
                    }
                    None => {}
                }

                reply_comments.push(item.to_owned());
                reply_comments.append(future);
            }
        }
        Ok(reply_comments)
    }

    pub(crate) async fn get_all_count(
        blog_id: i64,
        db: &DatabaseConnection,
    ) -> Result<u64, DataBaseError> {
        let count = comment::Entity::find()
            .filter(comment::Column::BlogId.eq(blog_id))
            .count(db)
            .await?;
        Ok(count)
    }

    pub(crate) async fn get_close_count(
        blog_id: i64,
        db: &DatabaseConnection,
    ) -> Result<u64, DataBaseError> {
        let count = comment::Entity::find()
            .filter(comment::Column::BlogId.eq(blog_id))
            .filter(comment::Column::IsPublished.eq(false))
            .count(db)
            .await?;
        Ok(count)
    }

    pub async fn add_and_update_comment(
        comment_dto: CommentDTO,
        db: &DatabaseConnection,
    ) -> Result<(), DataBaseError> {
        let model = comment::Model::from(comment_dto);
        if model.id > 0 {
            model.into_active_model().update(db).await?;
        } else {
            model.into_active_model().insert(db).await?;
        }
        Ok(())
    }

    pub async fn delete_comment(id: i64, db: &DatabaseConnection) -> Result<u64, DataBaseError> {
        let count = comment::Entity::delete_many()
            .filter(comment::Column::Id.eq(id))
            .exec(db)
            .await?;
        Ok(count.rows_affected)
    }
}
