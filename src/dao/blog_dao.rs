use crate::models::dto::blog_datetime::BlogDateTime;
use crate::models::dto::blog_dto::BlogDto;
use crate::models::vo::page_request::SearchRequest;
use crate::models::vo::{blog_detail::BlogDetail, blog_info::BlogInfo};
use crate::models::{category::Category, tag::Tag};
use crate::rbatis::RBATIS;

use rbatis::{Error, IPage, Page, PageRequest};
use rbs::to_value;

pub struct BlogDao;

impl BlogDao {
    // 获取公开的分页博文
    pub async fn get_blog_pages(
        page_num: u64,
        page_size: u64,
    ) -> Result<Page<BlogInfo>, rbatis::Error> {
        let page = BlogInfo::select_page(
            &RBATIS.acquire().await.expect("异常"),
            &PageRequest::new(page_num, page_size),
        )
        .await
        .unwrap_or_else(|e: rbatis::Error| {
            log::error!("{e}",);
            Page::new(0, 0)
        });
        Ok(page)
    }
    /**
     * 获取公开的博文(不分页)
     */
    pub async fn get_blog_list() -> Result<Vec<BlogInfo>, rbatis::Error> {
        let sql = "
        select
             blog.id,blog.title,blog.description,blog.create_time,blog.views ,blog.words,blog.read_time,blog.password,blog.is_top,blog.first_picture
         from
             blog  WHERE is_published = ?";

        let blog_info = RBATIS
            .query_decode::<Vec<BlogInfo>>(&sql, vec![to_value!(true)])
            .await
            .unwrap_or_else(|e| {
                log::error!("{e}");
                vec![]
            });

        Ok(blog_info)
    }
    //查询所有博文数量(全部)
    pub async fn get_blog_count() -> Result<i32, rbatis::Error> {
        let sql = "
        select
             count(blog.id)
         from
             blog";
        let blog_info = RBATIS
            .query_decode::<i32>(&sql, vec![])
            .await
            .unwrap_or_else(|e| {
                log::error!("{e}");
                0
            });

        Ok(blog_info)
    }

    //根据名称查询该分类博文(分页)
    pub async fn get_by_category(
        name: String,
        page_num: usize,
        page_size: u64,
    ) -> Result<Page<BlogInfo>, rbatis::Error> {
        //分类查询
        let sql = format!(
            "
    select
     id,category_name as name
    from
     category
     where category.category_name = \"{}\"
 ",
            name
        );
        let args = vec![];
        let category_query = RBATIS.query_decode::<Category>(&*sql, args).await;
        let category = category_query.unwrap_or_else(|e| {
            log::error!("{e}");
            Category::default()
        });
        //博文查询
        let sql = "select 
        blog.*
        from blog where blog.category_id = ?  order by create_time desc limit ?,?";
        let args = vec![
            to_value!(category.id),
            to_value!((page_num as u64 - 1) * page_size),
            to_value!(page_size),
        ];
        let blog_query = RBATIS.query_decode::<Vec<BlogInfo>>(&sql, args).await;
        let sql = "select 
        count(blog.id)
        from blog where blog.category_id = ?  order by create_time desc";
        let args = vec![to_value!(category.id)];
        let count_query = RBATIS
            .query_decode::<u64>(&sql, args)
            .await
            .unwrap_or_default();
        let page = Page {
            records: blog_query.unwrap_or_else(|e| {
                log::error!("{:?}", e);
                vec![]
            }),
            total: count_query,
            page_no: page_num.try_into().unwrap(),
            page_size,
            do_count: true,
        };
        Ok(page)
    }

    pub(crate) async fn get_by_id(id: u16) -> Option<BlogDetail> {
        let blog_detail =
            BlogDetail::select_by_id(&RBATIS.acquire().await.unwrap(), id.to_string()).await;
        blog_detail.unwrap_or_else(|e| {
            log::error!("{e}");
            None
        })
    }

    //根据标签名称查询该分类博文(降序 分页)
    pub async fn get_by_tag(
        name: String,
        page_num: usize,
        page_size: u64,
    ) -> Result<Page<BlogInfo>, rbatis::Error> {
        let sql = format!(
            "
    select
    id,tag_name as name,color
    from
    tag
     where tag_name = \"{}\"
 ",
            name
        );
        let args = vec![];
        let tag_query = RBATIS.query_decode::<Tag>(&*sql, args).await;
        let tag = tag_query.unwrap_or_else(|e| {
            println!("{:?}", e);
            Tag::default()
        });
        let sql = "select 
    blog.*
    from blog,blog_tag where blog.id=blog_tag.blog_id and blog_tag.tag_id= ? order by create_time desc limit ?,?";
        let args = vec![
            to_value!(tag.id.unwrap_or_default()),
            to_value!((page_num as u64 - 1) * page_size),
            to_value!(page_size),
        ];
        let blog_query = RBATIS.query_decode::<Vec<BlogInfo>>(&sql, args).await;

        let sql = "select 
        count(blog.id)
        from blog,blog_tag where blog.id=blog_tag.blog_id and blog_tag.tag_id= ? order by create_time desc";
        let args = vec![to_value!(tag.id.unwrap_or_default())];
        let count_query = RBATIS
            .query_decode::<u64>(&sql, args)
            .await
            .unwrap_or_default();
        let page = Page {
            records: blog_query.unwrap_or_else(|e| {
                log::error!("{:?}", e);
                vec![]
            }),
            total: count_query,
            page_no: page_num.try_into().unwrap(),
            page_size,
            do_count: true,
        };
        Ok(page)
    }

    //查询所有的公开文章的日期时间
    pub(crate) async fn get_all_datetime() -> Result<Vec<BlogDateTime>, Error> {
        let sql = "select 
    blog.create_time
    from blog where is_published = ? GROUP BY create_time ORDER BY create_time DESC";
        let datetime_query = RBATIS
            .query_decode::<Vec<BlogDateTime>>(sql, vec![to_value!(1)])
            .await
            .unwrap_or_else(|e| {
                log::error!("{}", e);
                vec![]
            });
        Ok(datetime_query)
    }

    //根据时间查询博文
    pub(crate) async fn get_by_date(date_time: String) -> Result<Vec<BlogInfo>, Error> {
        let year = &date_time.as_str()[0..4];
        let month = &date_time.as_str()[6..];
        let sql = "SELECT *
        FROM blog
        WHERE YEAR(create_time) = ?
          AND MONTH(create_time) = ?;";
        let datetime_query = RBATIS
            .query_decode::<Vec<BlogInfo>>(sql, vec![to_value!(year), to_value!(month)])
            .await
            .unwrap_or_else(|e| {
                log::error!("{}", e);
                vec![]
            });
        Ok(datetime_query)
    }

    //查询公开文章的数量
    pub(crate) async fn get_archives_count() -> Result<u64, Error> {
        let sql = "SELECT count(id)
    FROM blog
    WHERE is_published = 1";
        let count = RBATIS
            .query_decode::<u64>(sql, vec![])
            .await
            .unwrap_or_else(|e| {
                log::error!("{}", e);
                0
            });
        Ok(count)
    }

    /**
     * 查询分类下所有文章的数量
     */
    pub async fn get_category_count(name: String) -> i32 {
        let sql = "SELECT count(*)
    FROM blog,category
    WHERE blog.category_id=category.id and category.category_name= ?;";
        RBATIS
            .query_decode(sql, vec![to_value!(name)])
            .await
            .unwrap_or_else(|e| {
                log::error!("{}", e);
                0
            })
    }

    /**
     * 查询标签下所有文章的数量
     */
    pub async fn get_tags_count(name: String) -> i32 {
        //1.查询标签对象
        let sql = format!(
            "
    select
    id,tag_name as name,color
    from
    tag
     where tag_name = ?
 "
        );
        let args = vec![to_value!(name)];
        let tag_query = RBATIS.query_decode::<Tag>(&*sql, args).await;
        let tag = tag_query.unwrap_or_else(|e| {
            println!("{:?}", e);
            Tag::default()
        });
        let sql = "select 
    count(blog.id)
    from blog,blog_tag where blog.id=blog_tag.blog_id and blog_tag.tag_id= ?";

        RBATIS
            .query_decode(sql, vec![to_value!(tag.id.unwrap_or_default())])
            .await
            .unwrap_or_else(|e| {
                log::error!("{}", e);
                0
            })
    }
    /**
     * 获取全部博文并分页返回，支持按标题模糊查询(废弃)
     */
    pub(crate) async fn _get_blog_all_page(page: &SearchRequest) -> Page<BlogDto> {
        BlogDto::select_page_blog_all(
            &RBATIS.acquire().await.unwrap(),
            &PageRequest::new(page.get_page_num() as u64, page.get_page_size() as u64),
            page.get_title().as_str(),
        )
        .await
        .unwrap_or_else(|e| {
            log::error!("{}", e);
            Page::default()
        })
    }

    pub async fn select_page_blog_dto(page_args: &SearchRequest) -> Result<Page<BlogDto>, Error> {
        if page_args.get_page_num() == 0 || page_args.get_page_size() == 0 {
            return Err(Error::from("page_num 或者 page_size equels 0"));
        }
        let mut arg = vec![];
        let mut sql = String::from("select * from blog where 1=1 ");
        //拼接标题条件
        if !page_args.get_title().is_empty() {
            sql.insert_str(sql.len(), "and title like ? ");
            arg.push(to_value!("%".to_string() + &page_args.get_title() + "%"));
        }
        //拼接分类id条件
        if page_args.get_category_id() != 0 {
            sql.insert_str(sql.len(), " and category_id = ? ");
            arg.push(to_value!(page_args.get_category_id()));
        }
        sql.insert_str(sql.len(), " order by create_time desc limit ?,?");
        arg.push(to_value!(((page_args.get_page_num() - 1)
            * page_args.get_page_size())
        .to_string()
        .as_str()));
        arg.push(to_value!(page_args.get_page_size()));
        let query = RBATIS
            .query_decode::<Vec<BlogDto>>(sql.as_str(), arg)
            .await
            .unwrap_or_else(|opt| {
                log::error!("{:?}", opt);
                vec![]
            });
        let total = BlogDao::get_blog_count().await.unwrap_or(0) as u64;
        let records = query;
        let rusult_page = Page::new(
            page_args.get_page_num() as u64,
            page_args.get_page_size() as u64,
        )
        .set_records(records)
        .set_total(total);
        Ok(rusult_page)
    }
}

#[cfg(test)]
mod test {
    use rbatis::rbdc::DateTime;
    use rbatis::Page;
    //使用Rbatis Page结构体测试自定义数据
    #[test]
    fn test_my_page() {
        let arr = vec![1, 2, 3, 4, 5, 1, 2, 3, 4, 5];
        let page = Page {
            records: arr,
            total: 5 as u64,
            page_no: 1,
            page_size: 5,
            do_count: true,
        };
        println!("{:?}", page)
    }

    #[test]
    fn test_format() {
        let mut date = DateTime::now().format("YYYY-MM-DD");
        println!("{}", date);
        date.insert(4, '年');
        date.insert(10, '日');
        println!("{}", date);
    }

    #[test]
    fn test_sql() {
        let title = "123";
        let mut sql = String::from("select * from blog where 1=1 ");
        if !title.is_empty() {
            sql.insert_str(sql.len(), "title = ?");
        }
        sql.insert_str(sql.len(), " order by create_time desc");
        dbg!(sql);
    }
}
