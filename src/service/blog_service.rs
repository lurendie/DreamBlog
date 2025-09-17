use crate::constant::blog_info_constants::RANDOM_BLOG_LIMIT_NUM;
use crate::constant::blog_info_constants::{self, NEW_BLOG_PAGE_SIZE};
use crate::constant::redis_key_constants;
use crate::dao::BlogDao;
use crate::dao::{CategoryDao, TagDao};
use crate::models::dto::blog_dto::BlogDto;
use crate::models::dto::moment_dto::MomentDTO;
use crate::models::vo::blog_visibility::BlogVisibility;
use crate::models::vo::blog_vo::BlogVO;
use crate::models::vo::page_request::SearchRequest;
use crate::models::vo::search_blog::SearchBlog;
use crate::models::vo::{blog_archive::BlogArchive, blog_detail::BlogDetail, blog_info::BlogInfo};
use crate::rbatis::get_conn;
use crate::rbatis::RBATIS;
use crate::service::RedisService;
use rand::Rng;
use rbatis::IPage;
use rbatis::{rbdc::DateTime, IPageRequest, Page};
use rbs::to_value;
use rbs::value::map::ValueMap;
use rbs::Value;
use std::collections::HashMap;

use super::BlogTagService;
use super::TagService;

pub struct BlogService;

impl BlogService {
    pub(crate) async fn get_blog_list_by_is_published(page_num: u64) -> HashMap<String, Value> {
        //1.查询redis缓存
        let redis_cache = RedisService::get_hash_key(
            redis_key_constants::HOME_BLOG_INFO_LIST.to_string(),
            page_num.to_string(),
        )
        .await;
        //2.缓存不未Null则返回返回
        if let Some(redis_cache) = redis_cache {
            log::info!(
                "key:{} page:{} 数据存在",
                redis_key_constants::HOME_BLOG_INFO_LIST,
                page_num
            );
            return redis_cache;
        }
        //3.查询数据库
        let mut map: HashMap<String, Value> = HashMap::new();
        let page_list: Page<BlogInfo>;
        page_list = match BlogDao::get_blog_pages(page_num, blog_info_constants::PAGE_SIZE).await {
            Ok(mut ok) => {
                BlogService::bloginfo_handle(ok.get_records_mut()).await;
                ok
            }
            Err(e) => {
                log::error!("BlogList查询失败:{:?}", e);
                Page::new(0, 0)
            }
        };
        map.insert("list".to_string(), to_value!(page_list.get_records()));
        map.insert("totalPage".to_string(), to_value!(page_list.pages()));
        //4.如果数据库查询不是Null 存放到Redis中
        log::info!(
            "key:{} page:{} 数据不存在",
            redis_key_constants::HOME_BLOG_INFO_LIST,
            page_num
        );
        if !page_list.get_records().is_empty() {
            let _ = RedisService::set_hash_key(
                redis_key_constants::HOME_BLOG_INFO_LIST.to_string(),
                page_num.to_string(),
                &map,
            )
            .await;
        }
        map
    }
    /**
     * 获取随机文章
     */
    pub async fn get_blog_list_random() -> Vec<Value> {
        //1.查询Redis 缓存数据
        let redis_cache =
            RedisService::get_value_vec(redis_key_constants::RANDOM_BLOG_LIST.to_string()).await;
        if let Some(redis_cache) = redis_cache {
            let arr = match redis_cache {
                Value::Array(arr) => {
                    log::error!(
                        "key:{} 数据存在",
                        redis_key_constants::RANDOM_BLOG_LIST.to_string()
                    );
                    arr
                }
                _ => vec![],
            };
            return arr;
        }
        //2.查询数据库
        match BlogDao::get_blog_list().await {
            Ok(mut list) => {
                BlogService::bloginfo_handle(&mut list).await;
                let mut ids = vec![];
                let mut result = vec![];
                let mut rng = rand::thread_rng();

                if list.len() < RANDOM_BLOG_LIMIT_NUM {
                    for i in 0..(list.len() - 1) {
                        ids.push(i);
                        result.push(to_value!(list[i].clone()));
                    }
                } else {
                    //随机获取文章ID并且去重
                    while ids.len() < RANDOM_BLOG_LIMIT_NUM {
                        let index = rng.gen_range(0..(list.len() - 1));
                        if !ids.contains(&index) {
                            ids.push(index);
                            result.push(to_value!(list[index].clone()));
                        }
                    }
                }
                log::info!(
                    "key:{} 数据不存在",
                    redis_key_constants::RANDOM_BLOG_LIST.to_string()
                );
                //保存到Redis
                RedisService::set_value_vec(
                    redis_key_constants::RANDOM_BLOG_LIST.to_string(),
                    &to_value!(&result),
                )
                .await;
                return result;
            }
            Err(e) => {
                log::error!("{}", e);
                vec![]
            }
        }
    }

    /**
     * 获取最新文章
     */
    pub async fn get_blog_list_new() -> Vec<Value> {
        //1.查询Redis 缓存数据
        let redis_cache =
            RedisService::get_value_vec(redis_key_constants::NEW_BLOG_LIST.to_string()).await;
        if let Some(redis_cache) = redis_cache {
            let arr = match redis_cache {
                Value::Array(arr) => {
                    log::error!(
                        "key:{} 数据存在",
                        redis_key_constants::NEW_BLOG_LIST.to_string()
                    );
                    arr
                }
                _ => vec![],
            };
            return arr;
        }
        //2.查询数据库
        match BlogDao::get_blog_list().await {
            Ok(mut list) => {
                //反转元素
                list.reverse();
                BlogService::bloginfo_handle(&mut list).await;
                let mut result = vec![];
                //如果文章数量小于NEW_BLOG_PAGE_SIZE 则直接返回
                if list.len() < NEW_BLOG_PAGE_SIZE {
                    for i in 0..(list.len() - 1) {
                        result.push(to_value!(list[i].clone()));
                    }
                } else {
                    for i in 0..NEW_BLOG_PAGE_SIZE {
                        result.push(to_value!(list[i].clone()));
                    }
                }
                log::info!(
                    "key:{} 数据不存在",
                    redis_key_constants::NEW_BLOG_LIST.to_string()
                );
                //保存到Redis
                RedisService::set_value_vec(
                    redis_key_constants::NEW_BLOG_LIST.to_string(),
                    &to_value!(&result),
                )
                .await;
                result
            }
            Err(e) => {
                log::error!("{}", e.to_string());
                vec![]
            }
        }
    }

    //根据分类名称查询博文
    pub async fn get_by_name(name: String, page_num: usize) -> HashMap<String, Value> {
        let mut map: HashMap<String, Value> = HashMap::new();
        let page_list: Page<BlogInfo> =
            match BlogDao::get_by_category(name, page_num, blog_info_constants::PAGE_SIZE).await {
                Ok(mut ok) => {
                    BlogService::bloginfo_handle(ok.get_records_mut()).await;
                    ok
                }
                Err(e) => {
                    log::error!("BlogList查询失败:{}", e);
                    Page::new(0, 0)
                }
            };

        map.insert("list".to_string(), to_value!(page_list.get_records()));
        map.insert("totalPage".to_string(), to_value!(page_list.pages()));
        map
    }
    //根据ID查找博文
    pub(crate) async fn get_by_id(id: u16) -> Option<BlogDetail> {
        let mut blog = BlogDao::get_by_id(id)
            .await
            .unwrap_or_else(|| BlogDetail::new());
        blog.content = markdown::to_html(&blog.content);
        Some(blog)
    }

    //根据tag名称查询博文
    pub async fn get_by_tag_name(name: String, page_num: usize) -> HashMap<String, Value> {
        let mut map: HashMap<String, Value> = HashMap::new();
        let page_list: Page<BlogInfo> =
            match BlogDao::get_by_tag(name, page_num, blog_info_constants::PAGE_SIZE).await {
                Ok(mut ok) => {
                    BlogService::bloginfo_handle(ok.get_records_mut()).await;
                    ok
                }
                Err(e) => {
                    log::error!("BlogList查询失败:{}", e);
                    Page::new(0, 0)
                }
            };
        map.insert("list".to_string(), to_value!(page_list.get_records()));
        map.insert("totalPage".to_string(), to_value!(page_list.pages()));
        map
    }

//获取归档文章
pub(crate) async fn get_archives() -> ValueMap {
    //获取所有文章的日期
    let mut map: ValueMap = ValueMap::new();
    let blog_datetimes = BlogDao::get_all_datetime().await.unwrap_or_else(|e| {
        log::error!("{:?}", e);
        vec![]
    });
    let mut date_times = vec![];
    let _ = blog_datetimes
        .iter()
        .map(|itme| date_times.push(itme.create_time.format("YYYY-MM")))
        .collect::<Vec<_>>();
    //通过日期获取文章
    for item in date_times.iter_mut() {
        let mut itme_map: Vec<BlogArchive> = vec![];

            let blogs = BlogDao::get_by_date(item.clone())
                .await
                .unwrap_or_else(|e| {
                    log::error!("{:?}", e);
                    vec![]
                });
            for blog in blogs {
                let mut blog_archive = BlogArchive::new();
                blog_archive.id = blog.id.unwrap_or_default().to_string();
                blog_archive.password = blog.password.unwrap_or_default();
                blog_archive.privacy = false;
                blog_archive.day = blog.create_time.as_str()[8..10].to_string() + "日";
                blog_archive.title = blog.title;
                itme_map.push(blog_archive);
            }
            //更改
            item.insert(4, '年');
            item.insert(10, '月');
            map.insert(to_value!(item), to_value!(itme_map));
        }
        map
    }

pub(crate) async fn get_archives_count() -> Option<usize> {
    Some(BlogDao::get_archives_count().await.unwrap() as usize)
}

    /**
     * 处理BlogInfo结构体依赖关系
     */
    async fn bloginfo_handle(list: &mut Vec<BlogInfo>) {
        for item in list.iter_mut() {
            let id = item.id.unwrap();
            item.category = Some(
                CategoryDao::get_by_bloginfo_id(id)
                    .await
                    .unwrap_or_default(),
            );
            if let Some(password) = &item.password {
                //如果password!=null
                if *password != "" {
                    item.privacy = Some(true);
                }
            } else {
                item.privacy = Some(false)
            }
            item.password = Some(String::from(""));
            //转HTML
            item.description = markdown::to_html(&item.description);
            let tags = TagDao::get_blog_tags(id).await;
            item.tags = Some(tags);
            item.create_time = item.create_time.as_str()[0..19].to_string();
        }
    }

    /**
     * 所有文章总数(包含隐藏) -后台
     */
    pub async fn get_blog_count() -> i32 {
        BlogDao::get_blog_count().await.unwrap_or_default()
    }

    /**
     * 获取所有文章，用于首页展示，每页10条数据，并返回总页数，用于分页展示。 -后台
     */
    pub async fn get_blog_all_page(mut page: SearchRequest) -> ValueMap {
        if page.get_title() == "" {
            let _ = &page.set_title(None);
        }
        let mut map: ValueMap = ValueMap::new();
        let mut page_list = BlogDao::select_page_blog_dto(&page)
            .await
            .unwrap_or_else(|opt| {
                log::error!("{:?}", opt);
                Page::new(0, 0)
            });
        for item in page_list.get_records_mut() {
            if item.get_password().is_none() {
                item.set_password(Some(""));
            }
            item.set_category(Some(
                CategoryDao::get_by_id(item.get_category_id())
                    .await
                    .unwrap_or_default(),
            ));
        }
        map.insert(to_value!("pageNum"), to_value!(page_list.page_no()));
        map.insert(to_value!("pageSize"), to_value!(page_list.page_size()));
        map.insert(to_value!("pages"), to_value!(page_list.pages()));
        map.insert(to_value!("total"), to_value!(page_list.total()));
        map.insert(to_value!("list"), to_value!(page_list.get_records()));
        map
    }

    //根据ID查找博文 - 后台
    pub async fn update_visibility(v: &BlogVisibility) -> bool {
        let ok = BlogVisibility::update_by_column(&RBATIS.acquire().await.unwrap(), v, "id")
            .await
            .is_ok();
        ok
    }
    /**
     * 获取id的文章 -后台
     */
    pub(crate) async fn get_blog_dto(id: u16) -> Option<BlogDto> {
        let list =
            BlogDto::get_blog(&RBATIS.acquire().await.unwrap(), id.to_string().as_str()).await;

        match list {
            Ok(mut blogs) => {
                if !blogs.is_empty() {
                    let mut blog = blogs.pop().expect("异常");
                    //分类
                    blog.set_category(Some(
                        CategoryDao::get_by_id(blog.get_category_id())
                            .await
                            .unwrap(),
                    ));
                    //todo 标签
                    let mut tags = vec![];
                    let tags_id = TagService::get_tags_by_blog_id(blog.get_id()).await;
                    for id in tags_id {
                        let tag = TagService::get_tags_by_id(id).await;
                        tags.push(tag)
                    }
                    blog.set_tags(Some(tags));
                    return Some(blog);
                }
                None
            }

            Err(e) => {
                log::error!("{:?}", e);
                None
            }
        }
    }
    /**
     * 添加或者更新文章
     */
    pub(crate) async fn update_blog_dto(mut query: BlogVO) -> bool {
        // 开启事务
        let mut tx = match RBATIS.acquire_begin().await {
            Ok(tx) => tx,
            Err(e) => {
                log::error!("开启事务失败: {:?}", e);
                return false;
            }
        };

        // 新增或更新文章
        if query.get_id() == 0 {
            query.set_create_time(DateTime::now());
            query.set_update_time(DateTime::now());
            let data = match BlogVO::insert(&tx, &query).await {
                Ok(data) => data,
                Err(e) => {
                    log::error!("插入文章失败: {:?}", e);
                    tx.rollback()
                        .await
                        .unwrap_or_else(|e| log::error!("回滚事务失败: {:?}", e));
                    return false;
                }
            };
            // 获取新插入的id
            let new_id = match data.last_insert_id {
                Value::U64(id) => id as u16,
                Value::U32(id) => id as u16,
                _ => {
                    log::error!("获取新插入的id失败");
                    tx.rollback()
                        .await
                        .unwrap_or_else(|e| log::error!("回滚事务失败: {:?}", e));
                    return false;
                }
            };

            // 更新添加标签
            let add_tags = query.get_tag_list().unwrap_or_default();
            if !BlogTagService::inser_tags(add_tags, new_id, &mut tx).await {
                tx.rollback()
                    .await
                    .unwrap_or_else(|e| log::error!("回滚事务失败: {:?}", e));
                return false;
            }
        } else {
            query.set_update_time(DateTime::now());
            if !BlogVO::update_by_column(&tx, &query, "id").await.is_ok() {
                log::error!("更新文章失败");
                tx.rollback()
                    .await
                    .unwrap_or_else(|e| log::error!("回滚事务失败: {:?}", e));
                return false;
            }

            // 标签处理
            let add_tags = query.get_tag_list().unwrap_or_default();
            let remove_tags = TagService::get_tags_by_blog_id(query.get_id()).await;
            let (add, remove) = BlogService::array_diff(add_tags, remove_tags);

            // 新增标签或者删除标签失败则进行事务回滚
            if !BlogTagService::inser_tags(add, query.get_id(), &mut tx).await
                || !BlogTagService::remove_tags(remove, query.get_id(), &mut tx).await
            {
                tx.rollback()
                    .await
                    .unwrap_or_else(|e| log::error!("回滚事务失败: {:?}", e));
                return false;
            }
        }

        // 提交事务
        if let Err(e) = tx.commit().await {
            log::error!("提交事务失败: {:?}", e);
            tx.rollback()
                .await
                .unwrap_or_else(|e| log::error!("回滚事务失败: {:?}", e));
            return false;
        }
        true
    }

    //比对数组差异并返回
    pub fn array_diff(arr1: Vec<u16>, arr2: Vec<u16>) -> (Vec<u16>, Vec<u16>) {
        let mut add_result = Vec::new();
        let mut delete_result = Vec::new();
        //新数据 剔除重复，存在即新增
        for item in &arr1 {
            if !arr2.contains(item) {
                add_result.push(item.clone());
            }
        }
        //旧数据 剔除重复，存在即删除
        for item in &arr2 {
            if !arr1.contains(item) {
                delete_result.push(item.clone());
            }
        }
        (add_result, delete_result)
    }

    //删除Blog
    pub async fn delete_blog(id: u16) -> bool {
        let mut tx = match RBATIS.acquire_begin().await {
            Ok(tx) => tx,
            Err(e) => {
                log::error!("开启事务失败: {:?}", e);
                return false;
            }
        };
        //删除文章或者标签失败则进行事务回滚
        if !BlogVO::delete_by_column(&tx, "id", id).await.is_ok()
            || !BlogTagService::delete_tags_by_blog_id(id, &mut tx).await
        {
            log::error!("删除文章失败");
            tx.rollback()
                .await
                .unwrap_or_else(|e| log::error!("回滚事务失败: {:?}", e));
            return false;
        }

        if let Err(e) = tx.commit().await {
            log::error!("提交事务失败: {:?}", e);
            tx.rollback()
                .await
                .unwrap_or_else(|e| log::error!("回滚事务失败: {:?}", e));
            return false;
        }
        true
    }

    /**
     * 搜索博文
     */
    pub async fn search_blog(mut content: String) -> Vec<SearchBlog> {
        let mut find_str = content.clone();
        find_str.insert_str(0, r"[\u4E00-\u9FA5A-Za-z0-9_,，。\n\s*\r\t]{0,10}");
        find_str.insert_str(
            find_str.len(),
            r"[\u4E00-\u9FA5A-Za-z0-9_,，。\n\s*\r\t]{0,10}",
        );
        content.insert_str(0, "%");
        content.insert_str(content.len(), "%");
        //构建正则表达式,出现异常则panic
        let regex_builder = regex::RegexBuilder::new(&find_str)
            .case_insensitive(true)
            .build()
            .unwrap_or_else(|e| {
                log::error!("search_blog regex 构建失败:{:?}", e);
                panic!("search_blog regex 构建失败:{:?}", e)
            });
        match SearchBlog::select_by_title(&get_conn().await, &content).await {
            Ok(mut ok) => {
                for item in ok.iter_mut() {
                    let item_content = item.get_content().clone();
                    match regex_builder.find(&item_content) {
                        Some(find) => item.set_content(
                            item_content
                                .get(find.start()..find.end())
                                .unwrap_or_default()
                                .to_string(),
                        ),
                        None => {
                            log::info!("search_blog 未找到关键词:{:?}", find_str);
                        }
                    }
                }
                ok
            }
            Err(e) => {
                log::error!("search_blog error : {:?}", e);
                vec![]
            }
        }
    }

    pub(crate) async fn moment_like(id: u16) -> Result<u64, rbatis::rbdc::Error> {
        let executor = get_conn().await;
        let mut table =
            MomentDTO::select_by_column(&executor, "id", id.to_string().as_str()).await?;
        for item in table.iter_mut() {
            item.set_likes(item.get_likes() + 1);
            let query = MomentDTO::update_by_column(&executor, &item, "id").await?;
            return Ok(query.rows_affected);
        }
        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    use crate::{constant::blog_info_constants::RANDOM_BLOG_LIMIT_NUM, service::BlogService};
    use rand::Rng;
    use rbatis::rbdc::DateTime;
    // use regex::Regex;

    #[test]
    pub(crate) fn test_datetime() {
        let time = DateTime::now().format("YYYY-MM-DD hh:mm:ss");
        println!("{:?}", time)
        //stdout "2024-05-18 12:46:22"
    }

    //毕竟两个数组的相同数并剔除重复
    #[test]
    pub(crate) fn test_array_diff() {
        let add = vec![1, 2, 3, 4, 5];
        let remove = vec![2, 3, 4, 5];
        let (add, delete) = BlogService::array_diff(add, remove);
        println!("add : {:?}, delete : {:?}", add, delete);
        //assert_eq!(add, vec![]);
        //assert_eq!(delete, vec![]);
    }

    //stdout add : [6], delete : []
    //stdout ["a", "d", "e", "f"]
    //stdout add : [], delete : [1, 2, 3, 4, 5, 6]

    //测试随机
    #[test]
    pub fn test_random() {
        let mut ids = vec![];
        let list = vec![1, 2, 3, 4, 5, 6];
        let mut rng = rand::thread_rng();
        let mut result = vec![];
        //随机获取文章ID并且去重
        if list.len() < RANDOM_BLOG_LIMIT_NUM {
            //如果元素数量小于RANDOM_BLOG_LIMIT_NUM 则不处理
        } else {
            while ids.len() < RANDOM_BLOG_LIMIT_NUM {
                let index = rng.gen_range(0..(list.len() - 1));
                if !ids.contains(&index) {
                    println!("已添加: {}", index);
                    ids.push(index);
                    result.push(list[index].clone());
                    continue;
                }
                println!("重复的index: {}", index);
            }
        }

        // dbg!(&result);
    }
    //是否存在重复元素
    fn _test(list: Vec<i32>) -> bool {
        let list_2 = list.clone();
        for ele in list {
            let mut index = 0;
            for ele_2 in list_2.iter() {
                if ele == *ele_2 {
                    index += 1;
                }
            }
            if index > 1 {
                println!("数字:{}  出现重复次数 : {}", ele, index);
                return false;
            }
        }
        return true;
    }

    //字符串搜索 截取前后段
    #[test]
    fn test_str() {
        let item_content = "的撒旦撒打算去的撒大苏打 , 为什么思想家,我相信理想的力量,力量,是创造力,创造力,是智慧,智慧,是勇气,勇气,是力量,力量,是创造力,创造力,是智慧,智慧,是勇气,勇气,是力量,力量,是创造力,创造力,是智慧,智慧,是勇气,勇气,是力量,力量,是创造力,创造力,是智慧,智慧,是勇气,勇气,是力量,力量,是创造力,创造力,是智慧,智慧,是勇气,勇气,是力量,力量,是创造力,创造力,是智慧,智慧,是勇气,勇气";
        let range = regex::Regex::new(
            r"[\u4E00-\u9FA5A-Za-z0-9_,，。]{0,10}力量{1}[\u4E00-\u9FA5A-Za-z0-9_,，。]{0,10}",
        )
        .unwrap()
        .find(&item_content)
        .unwrap()
        .range();
        // //获取到关键词索引
        // let index = match item_content.find(&find_str) {
        //     Some(index) => index,
        //     None => {
        //         log::error!("search_blog Index 获取失败:{:?}", find_str);
        //         0
        //     }
        // };
        //起始位置索引
        // let start_index = match (index as isize - 11) <= 0 {
        //     true => 0,
        //     false => index - 11,
        // };

        // //终点位置索引
        // let end_index = index + 11;
        //   let new_content;

        // match end_index >= (item_content.len() - 1) {
        //     true => new_content = item_content.substring(start_index, item_content.len() - 1),
        //     false => new_content = item_content.substring(start_index, end_index),
        // }
        //println!("{:?}", item_content.get(index..index + 3));
        dbg!(&range);
        println!("{:?}", item_content.get(range.start..range.end));
        // println!("{:?}", item_content.substring(range.start, range.end));
        //  println!("{:?}", new_content);
    }
}
