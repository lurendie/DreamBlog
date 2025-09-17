use crate::models::vo::comment::Comment;
use crate::rbatis::RBATIS;
use rbatis::{executor::Executor, sql, Error, Page, PageRequest};

pub struct CommentDao;
impl CommentDao {
    //获取评论分页
    pub(crate) async fn get_comments_page(
        page_num: usize,
        page_size: u64,
        page: u16,
        blog_id: u16,
    ) -> Result<Page<Comment>, Error> {
        Comment::select_page(
            &RBATIS.acquire().await?,
            &PageRequest::new(page_num as u64, page_size),
            page.to_string().as_str(),
            blog_id.to_string().as_str(),
        )
        .await
    }
    //根据parent_comment_id 获取评论  使用递归函数已实现深沉递归,性能怪兽
    pub(crate) async fn get_comments(parent_comment_id: u16) -> Result<Vec<Comment>, Error> {
        let sql = "select * from comment where  parent_comment_id =?";
        let mut page = RBATIS
            .acquire()
            .await?
            .query_decode::<Vec<Comment>>(sql, vec![parent_comment_id.into()])
            .await;

        let mut futures = Vec::new();

        for item in page.as_ref().unwrap().iter() {
            // 使用 Box::pin 来递归调用 get_comments，允许存在递归
            if item.id.unwrap() > 0 {
                let future = Box::pin(CommentDao::get_comments(item.id.unwrap()));
                futures.push(future);
            }
        }
        let mut reply_comments = vec![];
        // 处理子评论
        for (item, comments) in page.as_mut().unwrap().iter_mut().zip(futures) {
            if let Ok(comments) = comments.await.as_mut() {
                item.parent_comment_name = Some(
                    CommentDao::get_comment_nickname(
                        &RBATIS.acquire().await.expect("mysql连接异常"),
                        item.parent_comment_id.unwrap(),
                    )
                    .await?,
                );
                //item.reply_comments = Some(comments.to_owned());
                reply_comments.push(item.to_owned());
                reply_comments.append(comments);
            }
        }
        Ok(reply_comments)
    }
    /**
     * 根据BlogId 获取某一个博文的评论
     */
    pub(crate) async fn get_all_comments(blog_id: u16) -> usize {
        Comment::select_all_comment(
            &RBATIS.acquire().await.unwrap(),
            blog_id.to_string().as_str(),
        )
        .await
        .unwrap_or_else(|e| {
            log::error!("dao get_all_comments error:{e}");
            vec![]
        })
        .len()
    }

    /**
     * 根据BlogId 获取某一个博文的评论
     */
    pub(crate) async fn get_close_comments(blog_id: u16) -> usize {
        Comment::select_close_comment(
            &RBATIS.acquire().await.unwrap(),
            blog_id.to_string().as_str(),
        )
        .await
        .unwrap_or_else(|e| {
            log::error!("dao get_close_comments error:{e}");
            vec![]
        })
        .len()
    }

    //获取Name
    #[sql("select nickname from comment where id = ?")]
    pub(crate) async fn get_comment_nickname(rb: &dyn Executor, id: i16) -> Result<String, Error> {
        impled!()
    }

    #[sql("select count(id) from comment")]
    pub(crate) async fn get_comment_count(rb: &dyn Executor) -> rbatis::Result<i32> {
        impled!()
    }
}
