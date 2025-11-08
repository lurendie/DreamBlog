use crate::entity::comment;
use crate::error::DataBaseError;
use crate::model::{CommentDTO, CommentVO};
use crate::service::{BlogService, EmailService, UserService};
use chrono::Local;
use rand::Rng;
use rbs::value;
use rbs::value::map::ValueMap;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DatabaseTransaction, DbErr, EntityTrait,
    IntoActiveModel, PaginatorTrait, QueryFilter, QueryOrder, TransactionTrait,
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
            .filter(comment::Column::Page.eq(page))
            .order_by_desc(comment::Column::CreateTime);

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
        page_type: u8,
        blog_id: i64,
        db: &DatabaseConnection,
    ) -> Result<ValueMap, DataBaseError> {
        let mut map = ValueMap::new();
        let select = comment::Entity::find().filter(comment::Column::Page.eq(page_type));
        let page = {
            if !matches!(blog_id, 0) {
                select
                    .filter(comment::Column::BlogId.eq(blog_id))
                    .paginate(db, page_size)
            } else {
                select.paginate(db, page_size)
            }
        };
        let models = page.fetch_page(page_num - 1).await?;
        let mut comments = vec![];
        for model in models.into_iter() {
            let blog_id = model.blog_id.unwrap_or_default();
            let mut comment = CommentDTO::from(model);
            if matches!(comment.page, 0) {
                comment.blog_id_and_title =
                    Some(BlogService::find_blog_id_and_title(db, blog_id).await?);
            }
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
        ip: String,
    ) -> Result<(), DataBaseError> {
        let option_model = comment::Entity::find_by_id(comment_dto.id).one(db).await;
        if let Ok(Some(model)) = option_model {
            let mut active = model.into_active_model();
            active.avatar = Set(comment_dto.avatar);
            active.content = Set(comment_dto.content);
            active.email = Set(comment_dto.email);
            active.ip = Set(Some(comment_dto.ip));
            active.nickname = Set(comment_dto.nickname);
            active.website = Set(Some(comment_dto.website));
            active.update(db).await?;
        } else {
            comment_dto.ip = ip;
            let mut rng = rand::thread_rng(); //生成随机数
            let index = rng.gen_range(1..5);
            comment_dto.avatar = format!("/img/comment-avatar/{}.jpg", index);
            comment_dto.published = true;
            comment_dto.create_time = Local::now().naive_local();
            let model = comment::Model::from(comment_dto);
            let model = model.into_active_model().insert(db).await?;
            //开启了订阅回复功能
            if model.is_notice && model.parent_comment_id != -1 && !model.is_admin_comment {
                let parent_model = Self::find_by_id(model.parent_comment_id, db).await?;
                if parent_model.email.eq(&model.email) {
                    return Ok(()); //如果评论者和父评论者是同一个人，则不发送邮件
                }
                let parent_model_dto = CommentDTO::from(parent_model);
                let err = EmailService::send_guest_email(db, model, parent_model_dto).await;
                if let Err(e) = err {
                    return Err(DataBaseError::Custom(e.to_string()));
                }
            } else if model.is_notice && model.parent_comment_id == -1 && !model.is_admin_comment {
                let owenr_user = UserService::find_admin_role(db).await?;
                let err = EmailService::send_owenr_email(model, db, owenr_user.get_email()).await;
                if let Err(e) = err {
                    return Err(DataBaseError::Custom(e.to_string()));
                }
            };
        };
        Ok(())
    }

    pub async fn find_by_id(
        id: i64,
        db: &DatabaseConnection,
    ) -> Result<comment::Model, DataBaseError> {
        let model = match comment::Entity::find_by_id(id).one(db).await? {
            Some(m) => m,
            None => return Err(DataBaseError::Custom(format!("id:{}的评论不存在", id))),
        };
        Ok(model)
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
