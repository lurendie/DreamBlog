use crate::constant::BlogInfoConstant;
use crate::constant::RedisKeyConstant;
use crate::entity::{
    blog::{self},
    blog_tag, category, tag,
};

use crate::common::MarkdownParser;
use crate::common::TypeValue;
use crate::error::DataBaseError;
use crate::model::{
    BlogArchive, BlogDetail, BlogInfo, BlogVO, BlogVisibility, SearchBlog, SearchRequest,
};
use crate::model::{BlogDTO, BlogIdAndTitle};
use crate::service::RedisService;
use chrono::{Datelike, NaiveDate};
use rand::Rng;
use rbs::value;
use rbs::value::map::ValueMap;
use rbs::Value;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, DbBackend, DbErr, EntityTrait,
    FromQueryResult, ModelTrait, PaginatorTrait, QueryFilter, QueryOrder, QueryTrait, Statement,
    TransactionTrait,
};
use std::collections::HashMap;
use std::ops::Index;
use std::str::FromStr;

pub struct BlogService;

impl BlogService {
    pub(crate) async fn find_list_by_page(
        page_num: u64,
        db: &DatabaseConnection,
    ) -> Result<HashMap<String, Value>, DataBaseError> {
        //1.查询redis缓存
        let redis_cache = RedisService::get_hash_key(
            RedisKeyConstant::HOME_BLOG_INFO_LIST.to_string(),
            page_num.to_string(),
        )
        .await;
        //2.缓存不未Null则返回缓存数据
        if let Ok(redis_cache) = redis_cache {
            log::info!(
                "reids KEY:{} 当前页：{} 获取缓存数据成功",
                RedisKeyConstant::HOME_BLOG_INFO_LIST,
                page_num
            );
            return Ok(redis_cache);
        }
        //3.查询数据库
        let mut map: HashMap<String, Value> = HashMap::new();

        let page = blog::Entity::find()
            .filter(blog::Column::IsPublished.eq(true))
            .order_by_desc(blog::Column::CreateTime)
            .paginate(db, BlogInfoConstant::PAGE_SIZE);
        let list = match page.fetch_page(page_num - 1).await {
            Ok(list) => list,
            Err(e) => {
                log::error!("查询失败:{}", e);
                vec![]
            }
        };
        let mut blog_info_list = Vec::new();
        for item in list {
            blog_info_list.push(BlogInfo::from(item));
        }

        BlogService::bloginfo_handle(&mut blog_info_list, db).await;

        map.insert("list".to_string(), value!(&blog_info_list));
        map.insert(
            "totalPage".to_string(),
            value!(page.num_pages().await.expect("分页错误")),
        );
        //4.如果数据库查询不是Null 存放到Redis中
        if !blog_info_list.is_empty() {
            let _ = RedisService::set_hash_key(
                RedisKeyConstant::HOME_BLOG_INFO_LIST.to_string(),
                page_num.to_string(),
                &map,
            )
            .await?;
        }
        log::info!(
            "redis KEY:{} 缓存数据成功",
            RedisKeyConstant::HOME_BLOG_INFO_LIST
        );
        Ok(map)
    }
    /**
     * 获取随机文章
     */
    pub async fn find_list_random(db: &DatabaseConnection) -> Result<Vec<Value>, DataBaseError> {
        //1.查询Redis 缓存数据
        let redis_cache =
            RedisService::get_value_vec(RedisKeyConstant::RANDOM_BLOG_LIST.to_string()).await;
        if let Some(redis_cache) = redis_cache {
            let arr = match redis_cache {
                Value::Array(arr) => {
                    log::info!(
                        "reids KEY:{} 获取缓存数据成功",
                        RedisKeyConstant::RANDOM_BLOG_LIST.to_string()
                    );
                    arr
                }
                _ => vec![],
            };
            return Ok(arr);
        }
        //2.查询数据库

        let blog_models = blog::Entity::find()
            .filter(blog::Column::IsPublished.eq(true))
            .all(db)
            .await
            .unwrap_or_default();
        let mut blog_info_list = Vec::new();
        for item in blog_models {
            blog_info_list.push(BlogInfo::from(item));
        }
        BlogService::bloginfo_handle(&mut blog_info_list, db).await;

        let mut ids = vec![];
        let mut result = vec![];
        let mut rng = rand::thread_rng();

        if blog_info_list.len() < BlogInfoConstant::RANDOM_BLOG_LIMIT_NUM {
            if blog_info_list.len() > 0 {
                for i in 0..(blog_info_list.len() - 1) {
                    ids.push(i);
                    result.push(value!(blog_info_list[i].clone()));
                }
            }
        } else {
            //随机获取文章ID并且去重
            while ids.len() < BlogInfoConstant::RANDOM_BLOG_LIMIT_NUM {
                let index = rng.gen_range(0..(blog_info_list.len() - 1));
                if !ids.contains(&index) {
                    ids.push(index);
                    result.push(value!(blog_info_list[index].clone()));
                }
            }
        }
        if result.len() > 0 {
            //保存到Redis
            RedisService::set_value_vec(
                RedisKeyConstant::RANDOM_BLOG_LIST.to_string(),
                &value!(&result),
            )
            .await?;
            log::info!(
                "redis KEY:{} 缓存数据成功",
                RedisKeyConstant::RANDOM_BLOG_LIST
            );
        }
        return Ok(result);
    }

    /**
     * 获取最新文章
     */
    pub async fn find_list_new(db: &DatabaseConnection) -> Result<Vec<Value>, DataBaseError> {
        //1.查询Redis 缓存数据
        let redis_cache =
            RedisService::get_value_vec(RedisKeyConstant::NEW_BLOG_LIST.to_string()).await;
        if let Some(redis_cache) = redis_cache {
            let arr = match redis_cache {
                Value::Array(arr) => {
                    log::info!(
                        "reids KEY:{} 获取缓存数据成功",
                        RedisKeyConstant::NEW_BLOG_LIST.to_string()
                    );
                    arr
                }
                _ => vec![],
            };
            return Ok(arr);
        }
        //2.查询数据库
        let blog_models = blog::Entity::find()
            .filter(blog::Column::IsPublished.eq(true))
            .all(db)
            .await
            .unwrap_or_default();
        let mut blog_info_list = Vec::new();
        for item in blog_models {
            blog_info_list.push(BlogInfo::from(item));
        }
        BlogService::bloginfo_handle(&mut blog_info_list, db).await;

        //反转元素
        blog_info_list.reverse();
        BlogService::bloginfo_handle(&mut blog_info_list, db).await;
        let mut result = vec![];
        //如果文章数量小于NEW_BLOG_PAGE_SIZE 则直接返回
        if blog_info_list.len() < BlogInfoConstant::NEW_BLOG_PAGE_SIZE {
            if blog_info_list.len() > 0 {
                for i in 0..(blog_info_list.len() - 1) {
                    result.push(value!(blog_info_list[i].clone()));
                }
            }
        } else {
            for i in 0..BlogInfoConstant::NEW_BLOG_PAGE_SIZE {
                result.push(value!(blog_info_list[i].clone()));
            }
        }

        if result.len() > 0 {
            //保存到Redis
            RedisService::set_value_vec(
                RedisKeyConstant::NEW_BLOG_LIST.to_string(),
                &value!(&result),
            )
            .await?;
            log::info!("redis KEY:{} 缓存数据成功", RedisKeyConstant::NEW_BLOG_LIST);
        }

        Ok(result)
    }

    //根据分类名称查询博文
    pub async fn find_by_categorya_name(
        name: String,
        page_num: usize,
        db: &DatabaseConnection,
    ) -> HashMap<String, Value> {
        let mut map: HashMap<String, Value> = HashMap::new();
        let category_model = match category::Entity::find()
            .filter(category::Column::CategoryName.eq(&name))
            .one(db)
            .await
        {
            Ok(category_model) => category_model.unwrap_or_default(),
            Err(e) => {
                log::error!("{:?}", e);
                category::Model::default()
            }
        };

        let page = category_model
            .find_related(blog::Entity)
            .filter(blog::Column::IsPublished.eq(true))
            .order_by_desc(blog::Column::CreateTime)
            .paginate(db, BlogInfoConstant::PAGE_SIZE);
        let blog_models = page
            .fetch_page(page_num as u64 - 1)
            .await
            .unwrap_or_default();
        let mut blog_info_list = Vec::new();
        for item in blog_models {
            blog_info_list.push(BlogInfo::from(item));
        }
        BlogService::bloginfo_handle(&mut blog_info_list, db).await;
        map.insert("list".to_string(), value!(blog_info_list));
        map.insert(
            "totalPage".to_string(),
            value!(page.num_pages().await.unwrap_or_default()),
        );
        map
    }

    //根据ID查找博文
    pub(crate) async fn find_id_detail(id: i64, db: &DatabaseConnection) -> Option<BlogDetail> {
        let blog_model = match blog::Entity::find_by_id(id).one(db).await {
            Ok(blog) => blog.unwrap_or_default(),
            Err(e) => {
                log::error!("{:?}", e);
                return None;
            }
        };
        let mut blog = BlogDetail::from(blog_model);
        blog.content = MarkdownParser::parser_html(blog.content.clone());
        Some(blog)
    }

    //根据tag名称查询博文
    pub async fn find_by_tag_name(
        name: String,
        page_num: usize,
        db: &DatabaseConnection,
    ) -> HashMap<String, Value> {
        let mut map: HashMap<String, Value> = HashMap::new();
        let tag_model = match tag::Entity::find()
            .filter(tag::Column::TagName.eq(&name))
            .one(db)
            .await
        {
            Ok(Some(tag_model)) => tag_model,
            Err(e) => {
                log::error!("{:?}", e);
                tag::Model::default()
            }
            _ => tag::Model::default(),
        };
        let page = tag_model
            .find_related(blog::Entity)
            .filter(blog::Column::IsPublished.eq(true))
            .order_by_desc(blog::Column::CreateTime)
            .paginate(db, BlogInfoConstant::PAGE_SIZE);
        let blog_models = page
            .fetch_page(page_num as u64 - 1)
            .await
            .unwrap_or_default();

        let mut blog_info_list = Vec::new();
        for item in blog_models {
            blog_info_list.push(BlogInfo::from(item));
        }
        BlogService::bloginfo_handle(&mut blog_info_list, db).await;
        map.insert("list".to_string(), value!(blog_info_list));
        map.insert(
            "totalPage".to_string(),
            value!(page.num_pages().await.unwrap_or_default()),
        );
        map
    }

    //获取归档文章
    pub(crate) async fn find_archives(db: &DatabaseConnection) -> Result<ValueMap, DataBaseError> {
        //获取所有文章的日期
        let mut map: ValueMap = ValueMap::new();
        let mut dates = ValueMap::new();
        //1.获取所有文章的日期
        blog::Entity::find()
            .filter(blog::Column::IsPublished.eq(true))
            .order_by_desc(blog::Column::CreateTime)
            .all(db)
            .await
            .unwrap_or_default()
            .into_iter()
            .for_each(|model| {
                let date = model.create_time.date();
                let date_str = format!("{}年{}月", date.year(), date.month());
                if let rbs::Value::Null = dates.index(date_str.as_str()) {
                    dates.insert(date_str.into(), date.to_string().into());
                }
            });

        for (key, value) in dates {
            let date_time = NaiveDate::from_str(value.as_str().unwrap_or_default()).unwrap();
            let sql = Statement::from_sql_and_values(
                DbBackend::MySql,
                r#"SELECT id,title,CONCAT(DAY(create_time),"日") as `day`,password
            FROM blog
            WHERE YEAR(create_time) = ?
              AND MONTH(create_time) = ?;"#,
                [date_time.year().into(), date_time.month().into()],
            );
            let mut blogs = BlogArchive::find_by_statement(sql)
                .all(db)
                .await
                .unwrap_or_default();

            for model in blogs.iter_mut() {
                if model.password.is_none() {
                    model.password = Some("".to_string());
                    model.privacy = Some(false);
                } else {
                    model.password = Some("".to_string());
                    model.privacy = Some(true);
                }
            }
            map.insert(value!(key), value!(blogs));
        }
        Ok(map)
    }

    pub(crate) async fn find_archives_count(db: &DatabaseConnection) -> Option<u64> {
        Some(
            blog::Entity::find()
                .filter(blog::Column::IsPublished.eq(true))
                .count(db)
                .await
                .unwrap_or_default(),
        )
    }

    /**
     * 处理BlogInfo结构体依赖关系
     */
    async fn bloginfo_handle(list: &mut Vec<BlogInfo>, db: &DatabaseConnection) {
        for item in list.iter_mut() {
            let id = item.id.unwrap_or_default();
            if let Ok(ok) = blog::Entity::find_by_id(id).one(db).await {
                match ok {
                    Some(blog) => {
                        item.related_handle(blog, db).await;
                    }
                    None => {
                        log::error!("没有检索到ID：{} 的文章，无法处理依赖关系", id);
                    }
                }
            }
            if let Some(password) = &item.password {
                //如果password!=null
                if *password != "" {
                    item.privacy = Some(true);
                } else {
                    item.privacy = Some(false)
                }
            } else {
                item.privacy = Some(false)
            }
            item.password = None;
            //转HTML
            item.description = MarkdownParser::parser_html(item.description.clone());
            item.create_time = item.create_time;
        }
    }

    /**
     * 获取所有文章，用于首页展示，每页10条数据，并返回总页数，用于分页展示。 -后台
     */
    pub async fn find_all_page(mut search: SearchRequest, db: &DatabaseConnection) -> ValueMap {
        if search.get_title().unwrap_or_default() == "" {
            let _ = &search.set_title(None);
        }

        let page = blog::Entity::find()
            .apply_if(search.get_title(), |query, value| {
                query.filter(blog::Column::Title.like(value))
            })
            .apply_if(search.get_category_id(), |query, value| {
                query.filter(blog::Column::CategoryId.eq(value))
            })
            .paginate(db, search.get_page_size() as u64);

        let mut map: ValueMap = ValueMap::new();
        let page_list = page
            .fetch_page(search.get_page_num() as u64 - 1)
            .await
            .unwrap_or_default();
        let mut blog_list = vec![];
        for model in page_list.into_iter() {
            let mut blog_dto = BlogDTO::from(model.clone());
            if blog_dto.get_password().is_none() {
                blog_dto.set_password(Some(""));
            }
            blog_dto.related_handle(model, db).await;
            blog_list.push(blog_dto);
        }

        map.insert(
            value!("pageNum"),
            value!(page.num_pages().await.unwrap_or_default()),
        );
        map.insert(value!("pageSize"), value!(search.get_page_size()));
        map.insert(
            value!("pages"),
            value!(page.num_pages().await.unwrap_or_default()),
        );
        map.insert(
            value!("total"),
            value!(page.num_items().await.unwrap_or_default()),
        );
        map.insert(value!("list"), value!(blog_list));
        map
    }

    //根据ID查找博文 - 后台
    pub async fn update_visibility(
        v: &BlogVisibility,
        db: &DatabaseConnection,
    ) -> Result<(), DataBaseError> {
        let blog_model = blog::Entity::find_by_id(v.get_id().unwrap_or_default())
            .one(db)
            .await?;
        match blog_model {
            Some(mut blog) => {
                if v.get_appreciation().is_some() {
                    blog.is_appreciation = v.get_appreciation().unwrap_or_default();
                }
                if v.get_published().is_some() {
                    blog.is_published = v.get_published().unwrap_or_default();
                }
                if v.get_top().is_some() {
                    blog.is_top = v.get_top().unwrap_or_default();
                }
                if v.get_password().is_some() {
                    blog.password = v.get_password();
                }
                if v.get_recommend().is_some() {
                    blog.is_recommend = v.get_recommend().unwrap_or_default();
                }
                if v.get_comment_enabled().is_some() {
                    blog.is_comment_enabled = v.get_comment_enabled().unwrap_or_default();
                }
                blog::ActiveModel::from(blog).update(db).await?;
                return Ok(());
            }
            None => {
                return Err(DataBaseError::Custom("没有检索到文章".to_string()));
            }
        }
    }
    /**
     * 获取id的文章 -后台
     */
    pub(crate) async fn find_by_id(id: u16, db: &DatabaseConnection) -> Result<BlogDTO, DbErr> {
        match blog::Entity::find_by_id(id).one(db).await {
            Ok(Some(blog)) => {
                let mut blog_dto = BlogDTO::from(blog.clone());
                if blog_dto.get_password().unwrap_or_default() == "" {
                    blog_dto.set_password(None);
                }
                blog_dto.related_handle(blog, db).await;
                Ok(blog_dto)
            }
            Ok(None) => Err(DbErr::Custom("没有检索到文章".to_string())),
            Err(e) => Err(e),
        }
    }

    /**
     * 添加或者更新文章
     */
    pub(crate) async fn update_blog(
        blog_vo: BlogVO,
        db: &DatabaseConnection,
    ) -> Result<(), DataBaseError> {
        let ok = db
            .transaction(|conn| {
                Box::pin(async move {
                    let tag_list = blog_vo.get_tag_list().unwrap_or_default();
                    let mut new_tag_ids = vec![];
                    for tag_type in tag_list {
                        match tag_type {
                            TypeValue::Int32(tag_id) => {
                                new_tag_ids.push(tag_id as i64);
                            }
                            TypeValue::String(tag_name) => {
                                let insert_result = tag::Entity::find()
                                    .filter(tag::Column::TagName.contains(&tag_name))
                                    .one(conn)
                                    .await;
                                //如果tag存在，则直接获取id
                                if let Ok(Some(tag_model)) = insert_result {
                                    new_tag_ids.push(tag_model.id);
                                } else {
                                    //如果tag不存在，则插入tag，并获取id
                                    let tag_model = tag::ActiveModel {
                                        tag_name: ActiveValue::set(tag_name.to_string()),
                                        color: ActiveValue::set(Some("red".to_string())),
                                        ..Default::default()
                                    };
                                    let result = tag::Entity::insert(tag_model).exec(conn).await?;
                                    new_tag_ids.push(result.last_insert_id);
                                }
                            }
                        }
                    }
                    let blog_model = blog::Model::from(blog_vo.clone()).into();
                    match blog_vo.get_id() == 0 {
                        true => {
                            let model = blog::ActiveModel::insert(blog_model, conn).await?;
                            if !new_tag_ids.is_empty() {
                                let mut insert_tag_models = vec![];
                                for tag_id in new_tag_ids {
                                    let active = blog_tag::ActiveModel {
                                        tag_id: ActiveValue::set(tag_id),
                                        blog_id: ActiveValue::set(model.id),
                                        ..Default::default()
                                    };
                                    insert_tag_models.push(active);
                                }
                                blog_tag::Entity::insert_many(insert_tag_models)
                                    .exec(conn)
                                    .await?;
                            }
                            Ok(())
                        }
                        false => {
                            let model = blog::ActiveModel::update(blog_model, conn).await?;

                            //1.查询旧的标签
                            let blog_tag_models = blog_tag::Entity::find()
                                .filter(blog_tag::Column::BlogId.eq(model.id))
                                .all(conn)
                                .await?;
                            //旧标签数据如果是空，则直接插入新标签
                            if !blog_tag_models.is_empty() {
                                let mut tag_ids = vec![];
                                for model in blog_tag_models {
                                    tag_ids.push(model.tag_id);
                                }

                                let (insert_tag_ids, delete_tag_ids) =
                                    Self::array_diff(new_tag_ids, tag_ids);
                                if !insert_tag_ids.is_empty() {
                                    let mut insert_tag_models = vec![];
                                    for tag_id in insert_tag_ids {
                                        let active = blog_tag::ActiveModel {
                                            tag_id: ActiveValue::set(tag_id),
                                            blog_id: ActiveValue::set(model.id),
                                            ..Default::default()
                                        };
                                        insert_tag_models.push(active);
                                    }
                                    blog_tag::Entity::insert_many(insert_tag_models)
                                        .exec(conn)
                                        .await?;
                                }
                                if !delete_tag_ids.is_empty() {
                                    blog_tag::Entity::delete_many()
                                        .filter(blog_tag::Column::TagId.is_in(delete_tag_ids))
                                        .exec(conn)
                                        .await?;
                                }
                            } else {
                                //直接插入新标签
                                if !new_tag_ids.is_empty() {
                                    let mut insert_tag_models = vec![];
                                    for tag_id in new_tag_ids {
                                        let active = blog_tag::ActiveModel {
                                            tag_id: ActiveValue::set(tag_id),
                                            blog_id: ActiveValue::set(model.id),
                                            ..Default::default()
                                        };
                                        insert_tag_models.push(active);
                                    }
                                    blog_tag::Entity::insert_many(insert_tag_models)
                                        .exec(conn)
                                        .await?;
                                }
                            }
                            Ok(())
                        }
                    }
                })
            })
            .await?;
        Ok(ok)
    }

    //比对数组差异并返回
    fn array_diff(arr1: Vec<i64>, arr2: Vec<i64>) -> (Vec<i64>, Vec<i64>) {
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
    pub async fn delete_by_id(id: i64, db: &DatabaseConnection) -> Result<(), DataBaseError> {
        let result = db
            .transaction(|conn| {
                Box::pin(async move {
                    blog::Entity::delete_by_id(id).exec(conn).await?;
                    blog_tag::Entity::delete_many()
                        .filter(blog_tag::Column::BlogId.eq(id))
                        .exec(conn)
                        .await?;
                    Ok(())
                })
            })
            .await?;

        Ok(result)
    }

    /**
     * 搜索博文
     */
    pub async fn search_content(
        mut content: String,
        db: &DatabaseConnection,
    ) -> Result<Vec<SearchBlog>, DataBaseError> {
        let mut find_str = content.clone();
        find_str.insert_str(0, r"[\u4E00-\u9FA5A-Za-z0-9_,，。\n\s*\r\t]{0,10}");
        find_str.insert_str(
            find_str.len(),
            r"[\u4E00-\u9FA5A-Za-z0-9_,，。\n\s*\r\t]{0,10}",
        );
        content.insert_str(0, "%");
        content.insert_str(content.len(), "%");
        //构建正则表达式,出现异常则返回异常
        let regex_builder = regex::RegexBuilder::new(&find_str)
            .case_insensitive(true)
            .build()?;
        let mut models = blog::Entity::find()
            .filter(blog::Column::IsPublished.eq(true))
            .filter(blog::Column::Content.contains(content))
            .all(db)
            .await?;
        let mut search_blogs = vec![];
        for item in models.iter_mut() {
            let mut search_blog = SearchBlog::from(item.clone());
            match regex_builder.find(&item.content) {
                Some(find) => {
                    search_blog.set_content(
                        item.content
                            .get(find.start()..find.end())
                            .unwrap_or_default()
                            .to_string(),
                    );
                    search_blogs.push(search_blog);
                }
                None => {
                    log::info!("search_blog 未找到关键词:{:?}", find_str);
                }
            }
        }
        Ok(search_blogs)
    }

    pub async fn check_category_exist_blog(
        category_id: i64,
        db: &DatabaseConnection,
    ) -> Result<bool, DataBaseError> {
        match category::Entity::find()
            .filter(category::Column::Id.eq(category_id))
            .one(db)
            .await?
        {
            Some(model) => {
                let count = model
                    .find_related(blog::Entity)
                    .count(db)
                    .await
                    .unwrap_or_default();
                Ok(count > 0)
            }
            None => {
                log::error!("分类下 {} 没有检索到文章", category_id);
                Ok(false)
            }
        }
    }

    pub(crate) async fn find_blogs_and_title(
        db: &DatabaseConnection,
    ) -> Result<Vec<BlogIdAndTitle>, DataBaseError> {
        let mut models = blog::Entity::find().all(db).await?;
        let mut blog_list = vec![];
        for model in models.iter_mut() {
            let blog_info = BlogIdAndTitle::from(model.clone());
            blog_list.push(blog_info);
        }
        Ok(blog_list)
    }

    pub(crate) async fn find_blog_id_and_title(
        db: &DatabaseConnection,
        blog_id: i64,
    ) -> Result<BlogIdAndTitle, DataBaseError> {
        let model = blog::Entity::find()
            .filter(blog::Column::Id.eq(blog_id))
            .one(db)
            .await?;
        if let Some(model) = model {
            let blog_info = BlogIdAndTitle::from(model.clone());
            Ok(blog_info)
        } else {
            log::error!("没有检索到文章");
            Err(DataBaseError::Custom("没有检索到文章".to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{constant::BlogInfoConstant, service::BlogService};
    use chrono::Local;
    use rand::Rng;
    // use sea_orm::{DbBackend, EntityTrait, FromQueryResult, Statement};
    // use regex::Regex;

    // #[test]
    // pub(crate) fn test_datetime() {
    //     let time = DateTime::now().format("YYYY-MM-DD hh:mm:ss");
    //     println!("{:?}", time)
    //     //stdout "2024-05-18 12:46:22"

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
        if list.len() < BlogInfoConstant::RANDOM_BLOG_LIMIT_NUM {
            //如果元素数量小于RANDOM_BLOG_LIMIT_NUM 则不处理
        } else {
            while ids.len() < BlogInfoConstant::RANDOM_BLOG_LIMIT_NUM {
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
    #[test]
    fn test_data() {
        let date = Local::now().naive_local();
        println!("{:?}", date.format("%Y-%m").to_string());
    }
    // use super::*;
    // use crate::app_state::get_connection;
    // use crate::entity::blog;
    // use chrono::Datelike;
    // use rbs::value::map::ValueMap;
    // use std::ops::Index;
    // use std::str::FromStr;
    // #[actix_web::test]
    // async fn test_find_date_time() {
    //     let db = get_connection().await;
    //     //第一步查询所有的时间
    //     let mut dates = ValueMap::new();
    //     blog::Entity::find()
    //         .order_by_desc(blog::Column::CreateTime)
    //         .all(&db)
    //         .await
    //         .unwrap()
    //         .into_iter()
    //         .for_each(|model| {
    //             let date = model.create_time.date();
    //             let date_str = format!("{}年{}月", date.year(), date.month());
    //             if let rbs::Value::Null = dates.index(date_str.as_str()) {
    //                 dates.insert(date_str.into(), date.to_string().into());
    //             }
    //         });

    //     for (key, value) in dates {
    //         println!("{} : {}", key, value);
    //         let date_time = NaiveDate::from_str(value.as_str().unwrap_or_default()).unwrap();
    //         let sql = Statement::from_sql_and_values(
    //             DbBackend::MySql,
    //             r#"SELECT id,title,DAY(create_time) as `day`,password
    //     FROM blog
    //     WHERE YEAR(create_time) = ?
    //       AND MONTH(create_time) = ?;"#,
    //             [date_time.year().into(), date_time.month().into()],
    //         );
    //         let blogs = BlogArchive::find_by_statement(sql)
    //             .all(&db)
    //             .await
    //             .unwrap_or_default();
    //         dbg!(&blogs);
    //     }

    //     //2.查询每月的文章数量
    // }
}
