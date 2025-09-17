use rbs::to_value;

use crate::models::category::Category;
use crate::rbatis::RBATIS;

pub struct CategoryDao;

impl CategoryDao {
    /**
     * 获取所有的分类
     */
    pub async fn get_list() -> Vec<Category> {
        let sql = "select id,category_name as name from category";
        let category: Vec<Category> = match RBATIS.query_decode(sql, Vec::new()).await {
            Ok(ok) => ok,
            Err(e) => {
                log::error!("{e}");
                vec![]
            }
        };
        category
    }

    /**
     * 根据Blog_ID查询
     */
    pub async fn get_by_bloginfo_id(id: u16) -> Result<Category, rbatis::rbdc::Error> {
        let sql = format!(
            "
        select
         id,category_name as name
        from
         category
         where category.id =(select category_id  from blog where blog.id ={})
     ",
            id
        );
        let args = vec![];
        let category = RBATIS.query_decode::<Category>(&sql, args).await;
        category
    }

    /**
     * 根据分类ID查询分类信息
     */
    //#[sql("select id,category_name as name from category where id = ?")]
    pub async fn get_by_id(id: u16) -> Result<Category, rbatis::rbdc::Error> {
        let sql = "select id,category_name as name from category where id = ?";
        let args = vec![to_value!(id)];

        let category = RBATIS.query_decode::<Category>(&sql, args).await;
        category
    }
}
