use crate::models::dto::tag_dto::TagVO;
use crate::rbatis::RBATIS;

pub struct TagDao;
impl TagDao {
    pub async fn get_list() -> Result<Vec<TagVO>, rbatis::Error> {
        let sql = format!(
            "
            select
            tag.id as id,tag_name ,color
            from tag
            "
        );
        let tags: Result<Vec<TagVO>, rbatis::Error> = RBATIS.query_decode(&sql, vec![]).await;
        tags
    }

    /**
     * 根据BlogId查询当前所有的云标签
     */
    pub async fn get_blog_tags(id: u16) -> Vec<TagVO> {
        //TagList
        let sql = format!(
            "
            select
            tag.id as id,tag_name ,color
            from blog_tag,tag
            where blog_tag.tag_id = tag.id and blog_tag.blog_id = {}
            ",
            id
        );
        let tags = RBATIS
            .query_decode::<Vec<TagVO>>(&*sql, vec![])
            .await
            .unwrap_or_else(|e| {
                log::error!("异常:{e}");
                vec![]
            });
        tags
    }
}
