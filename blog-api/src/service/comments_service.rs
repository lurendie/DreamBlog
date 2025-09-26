use crate::entity::comment;
use crate::error::DataBaseError;
use crate::model::{CommentDTO, CommentVO};
use crate::service::BlogService;
use chrono::Local;
use rbs::value;
use rbs::value::map::ValueMap;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DatabaseTransaction, DbErr, EntityTrait,
    IntoActiveModel, PaginatorTrait, QueryFilter, TransactionTrait,
};
//每页显示5条博客简介
const PAGE_SIZE: u64 = 5;

pub struct CommentService;

impl CommentService {
    //分页评论
    pub(crate) async fn find_by_id_comments(
        page_num: u64,
        blog_id: i64,
        page: u8,
        db: &DatabaseConnection,
    ) -> Result<ValueMap, DataBaseError> {
        let mut map = ValueMap::new();
        let select_sql = comment::Entity::find()
            .filter(comment::Column::IsPublished.eq(true))
            .filter(comment::Column::ParentCommentId.eq(-1))
            .filter(comment::Column::Page.eq(page));

        let page_list = {
            match page == 0 {
                true => select_sql
                    .filter(comment::Column::BlogId.eq(blog_id))
                    .paginate(db, PAGE_SIZE),
                false => select_sql.paginate(db, PAGE_SIZE),
            }
        };
        let models = page_list.fetch_page(page_num - 1).await?;
        let mut comments = vec![];
        for model in models.into_iter() {
            let id = model.id;
            let mut comment = CommentVO::from(model);
            comment.reply_comments = Some(Self::find_comment_vo_by_id(id, db).await?);
            comments.push(comment);
        }
        map.insert("list".into(), value!(comments));
        map.insert(
            "totalPage".into(),
            rbs::Value::U64(page_list.num_pages().await?),
        );

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
            value!("pageNum"),
            value!(page.num_pages().await.unwrap_or_default()),
        );
        map.insert(value!("pageSize"), value!(PAGE_SIZE));
        map.insert(
            value!("pages"),
            value!(page.num_pages().await.unwrap_or_default()),
        );
        map.insert(
            value!("total"),
            value!(page.num_items().await.unwrap_or_default()),
        );
        map.insert("list".into(), value!(comments));

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
        page: u8,
        db: &DatabaseConnection,
    ) -> Result<u64, DataBaseError> {
        let select = comment::Entity::find().filter(comment::Column::Page.eq(page));
        let count = match page == 0 {
            true => {
                select
                    .filter(comment::Column::BlogId.eq(blog_id))
                    .count(db)
                    .await?
            }
            false => select.count(db).await?,
        };
        Ok(count)
    }

    pub(crate) async fn get_close_count(
        blog_id: i64,
        page: u8,
        db: &DatabaseConnection,
    ) -> Result<u64, DataBaseError> {
        let select = comment::Entity::find()
            .filter(comment::Column::Page.eq(page))
            .filter(comment::Column::IsPublished.eq(false));
        let count = match page == 0 {
            true => {
                select
                    .filter(comment::Column::BlogId.eq(blog_id))
                    .count(db)
                    .await?
            }
            false => select.count(db).await?,
        };
        Ok(count)
    }

    pub async fn save_comment(
        mut comment_dto: CommentDTO,
        db: &DatabaseConnection,
    ) -> Result<(), DataBaseError> {
        let option_model = comment::Entity::find_by_id(comment_dto.id).one(db).await?;
        comment_dto.create_time = Local::now().naive_local(); // 设置创建时间
        if let Some(mut model) = option_model {
            model.avatar = comment_dto.avatar;
            model.content = comment_dto.content;
            model.email = comment_dto.email.unwrap_or_default();
            model.ip = comment_dto.ip;
            model.nickname = comment_dto.nickname;
            model.website = comment_dto.website;

            dbg!(&model);
            model.into_active_model().update(db).await?;
        } else {
            //http://q.qlogo.cn/headimg_dl?dst_uin=QQ号码&spec=640
            let mut model = comment::Model::from(comment_dto);
            model.parent_comment_id = -1; // 设置默认值
            if model.qq.is_some() {
                model.avatar = format!(
                    "http://q.qlogo.cn/headimg_dl?dst_uin={}&spec=640",
                    model.qq.as_ref().unwrap()
                );
            } else {
                //随机头像
            }
            model.into_active_model().insert(db).await?;
        }
        Ok(())
    }

    /// 在事务内部删除评论的辅助方法
    async fn delete_comment_in_transaction<'a>(
        id: i64,
        conn: &'a DatabaseTransaction,
    ) -> Result<u64, DbErr> {
        let mut total_deleted = 0u64;

        // 删除当前评论
        let count = comment::Entity::delete_many()
            .filter(comment::Column::Id.eq(id))
            .exec(conn)
            .await?;
        total_deleted += count.rows_affected;

        // 查找所有直接子评论
        let child_comments = comment::Entity::find()
            .filter(comment::Column::ParentCommentId.eq(id))
            .all(conn)
            .await?;

        // 递归删除每个子评论
        for child in child_comments {
            // 创建一个新的异步块，确保它可以被发送到其他线程
            let child_id = child.id;
            let child_count = async {
                // 使用事务的克隆而不是直接引用
                let tx = conn.begin().await?;
                let result = Box::pin(Self::delete_comment_in_transaction(child_id, &tx)).await;
                tx.commit().await?;
                result
            }
            .await?;
            total_deleted += child_count;
        }

        Ok(total_deleted)
    }

    pub async fn delete_comment_recursive(
        id: i64,
        db: &DatabaseConnection,
    ) -> Result<u64, DataBaseError> {
        // 在一个事务中删除所有评论
        let result = db
            .transaction(|conn| {
                Box::pin(async move {
                    let mut total_deleted = 0u64;

                    let count = comment::Entity::delete_many()
                        .filter(comment::Column::Id.eq(id))
                        .exec(conn)
                        .await?;
                    total_deleted += count.rows_affected;

                    // 查找所有直接子评论
                    let tal = Self::delete_comment_in_transaction(id, conn).await?;

                    Ok(total_deleted + tal)
                })
            })
            .await?;
        Ok(result)
    }
}
