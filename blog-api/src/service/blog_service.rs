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
use crate::model::{BlogDTO, BlogIdAndTitle, Category, TagDTO};
use crate::service::RedisService;
use chrono::{Datelike, Local, NaiveDate};
use rand::Rng;
use rbs::value;
use rbs::value::map::ValueMap;
use rbs::Value;
use sea_orm::prelude::Expr;
use sea_orm::IntoActiveModel;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DatabaseConnection, DbBackend,
    EntityTrait, FromQueryResult, ModelTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect,
    QueryTrait, Statement, TransactionTrait,
};
use std::collections::HashMap;
use std::ops::Index;
use std::str::FromStr;

pub struct BlogService;

impl BlogService {
    fn category_blog_cache_field(name: &str, page_num: usize) -> String {
        format!("{}:{}", name, page_num)
    }

    fn tag_blog_cache_field(name: &str, page_num: usize) -> String {
        format!("{}:{}", name, page_num)
    }

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
            tracing::info!(
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
                tracing::error!("查询失败:{}", e);
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
            value!(page.num_pages().await.unwrap_or_default()),
        );
        //4.如果数据库查询不是Null 存放到Redis中
        if !blog_info_list.is_empty()
            && RedisService::try_set_hash_key(
                RedisKeyConstant::HOME_BLOG_INFO_LIST.to_string(),
                page_num.to_string(),
                &map,
            )
            .await
        {
            tracing::info!(
                "redis KEY:{} 缓存数据成功",
                RedisKeyConstant::HOME_BLOG_INFO_LIST
            );
        }
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
                    tracing::info!(
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
            .await?;
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
                for i in 0..blog_info_list.len() {
                    ids.push(i);
                    result.push(value!(blog_info_list[i].clone()));
                }
            }
        } else {
            //随机获取文章ID并且去重（上界为 len，避免最后一个元素永远不被选中）
            while ids.len() < BlogInfoConstant::RANDOM_BLOG_LIMIT_NUM {
                let index = rng.gen_range(0..blog_info_list.len());
                if !ids.contains(&index) {
                    ids.push(index);
                    result.push(value!(blog_info_list[index].clone()));
                }
            }
        }
        if result.len() > 0
            //保存到Redis
            && RedisService::try_set_value_vec(
                RedisKeyConstant::RANDOM_BLOG_LIST.to_string(),
                &value!(&result),
            )
            .await
        {
            tracing::info!(
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
                    tracing::info!(
                        "reids KEY:{} 获取缓存数据成功",
                        RedisKeyConstant::NEW_BLOG_LIST.to_string()
                    );
                    arr
                }
                _ => vec![],
            };
            return Ok(arr);
        }
        //2.查询数据库（显式按创建时间倒序，最新在前）
        let blog_models = blog::Entity::find()
            .filter(blog::Column::IsPublished.eq(true))
            .order_by_desc(blog::Column::CreateTime)
            .all(db)
            .await?;
        let mut blog_info_list: Vec<BlogInfo> =
            blog_models.into_iter().map(BlogInfo::from).collect();
        //截取最新 N 篇后再处理依赖关系，避免重复处理全部文章
        blog_info_list.truncate(BlogInfoConstant::NEW_BLOG_PAGE_SIZE);
        BlogService::bloginfo_handle(&mut blog_info_list, db).await;

        let mut result = vec![];
        for item in blog_info_list {
            result.push(value!(item.clone()));
        }

        if result.len() > 0
            //保存到Redis
            && RedisService::try_set_value_vec(
                RedisKeyConstant::NEW_BLOG_LIST.to_string(),
                &value!(&result),
            )
            .await
        {
            tracing::info!("redis KEY:{} 缓存数据成功", RedisKeyConstant::NEW_BLOG_LIST);
        }

        Ok(result)
    }

    //根据分类名称查询博文
    pub async fn find_by_categorya_name(
        name: String,
        page_num: usize,
        db: &DatabaseConnection,
    ) -> HashMap<String, Value> {
        let cache_field = Self::category_blog_cache_field(&name, page_num);
        let redis_cache = RedisService::get_hash_key(
            RedisKeyConstant::CATEGORY_BLOG_LIST.to_string(),
            cache_field.clone(),
        )
        .await;
        if let Ok(redis_cache) = redis_cache {
            tracing::info!(
                "redis KEY:{} 字段:{} 获取缓存数据成功",
                RedisKeyConstant::CATEGORY_BLOG_LIST,
                cache_field
            );
            return redis_cache;
        }

        let mut map: HashMap<String, Value> = HashMap::new();
        let category_model = match category::Entity::find()
            .filter(category::Column::CategoryName.eq(&name))
            .one(db)
            .await
        {
            Ok(category_model) => category_model.unwrap_or_default(),
            Err(e) => {
                tracing::error!("{:?}", e);
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
        if RedisService::try_set_hash_key(
            RedisKeyConstant::CATEGORY_BLOG_LIST.to_string(),
            cache_field,
            &map,
        )
        .await
        {
            tracing::info!(
                "redis KEY:{} 缓存数据成功",
                RedisKeyConstant::CATEGORY_BLOG_LIST
            );
        }
        map
    }

    //根据ID查找博文
    pub(crate) async fn find_id_detail(id: i64, db: &DatabaseConnection) -> Option<BlogDetail> {
        // 仅已发布文章可通过前台访问（防止草稿/已下架文章被缓存命中或直接访问）
        let published = blog::Entity::find()
            .filter(blog::Column::Id.eq(id))
            .filter(blog::Column::IsPublished.eq(true))
            .count(db)
            .await
            .unwrap_or(0)
            > 0;
        if !published {
            return None;
        }

        let mut cached_blog = RedisService::get_hash_key::<BlogDetail>(
            RedisKeyConstant::BLOG_DETAIL_MAP.to_string(),
            id.to_string(),
        )
        .await
        .ok();
        let views = Self::increment_blog_views(id, db).await;
        if let Some(blog) = cached_blog.as_mut() {
            if let Some(views) = views {
                blog.views = views;
            }
            return cached_blog;
        }

        let blog_model = match blog::Entity::find_by_id(id).one(db).await {
            Ok(blog) => blog.unwrap_or_default(),
            Err(e) => {
                tracing::error!("{:?}", e);
                return None;
            }
        };
        let category_id = blog_model.category_id;
        let tag_models = blog_model
            .find_related(tag::Entity)
            .all(db)
            .await
            .unwrap_or_default();
        let mut blog = BlogDetail::from(blog_model);
        // 分类信息（详情页展示分类标签）
        if let Ok(Some(category_model)) = category::Entity::find_by_id(category_id).one(db).await {
            blog.category = Some(Category::from(category_model));
        }
        blog.tags = Some(
            tag_models
                .into_iter()
                .map(crate::model::TagDTO::from)
                .collect(),
        );
        blog.content = MarkdownParser::parser_html(blog.content.clone());
        if let Some(views) = views {
            blog.views = views;
        }
        if RedisService::try_set_hash_key(
            RedisKeyConstant::BLOG_DETAIL_MAP.to_string(),
            id.to_string(),
            &blog,
        )
        .await
        {
            tracing::info!(
                "redis KEY:{} 缓存数据成功",
                RedisKeyConstant::BLOG_DETAIL_MAP
            );
        }
        Some(blog)
    }

    //根据tag名称查询博文
    pub async fn find_by_tag_name(
        name: String,
        page_num: usize,
        db: &DatabaseConnection,
    ) -> HashMap<String, Value> {
        let cache_field = Self::tag_blog_cache_field(&name, page_num);
        let redis_cache = RedisService::get_hash_key(
            RedisKeyConstant::TAG_BLOG_LIST.to_string(),
            cache_field.clone(),
        )
        .await;
        if let Ok(redis_cache) = redis_cache {
            tracing::info!(
                "redis KEY:{} 字段:{} 获取缓存数据成功",
                RedisKeyConstant::TAG_BLOG_LIST,
                cache_field
            );
            return redis_cache;
        }

        let mut map: HashMap<String, Value> = HashMap::new();
        let tag_model = match tag::Entity::find()
            .filter(tag::Column::TagName.eq(&name))
            .one(db)
            .await
        {
            Ok(Some(tag_model)) => tag_model,
            Err(e) => {
                tracing::error!("{:?}", e);
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
        if RedisService::try_set_hash_key(
            RedisKeyConstant::TAG_BLOG_LIST.to_string(),
            cache_field,
            &map,
        )
        .await
        {
            tracing::info!("redis KEY:{} 缓存数据成功", RedisKeyConstant::TAG_BLOG_LIST);
        }
        map
    }

    //获取归档文章
    pub(crate) async fn find_archives(db: &DatabaseConnection) -> Result<ValueMap, DataBaseError> {
        let redis_cache =
            RedisService::get_string(RedisKeyConstant::ARCHIVE_BLOG_MAP.to_string()).await;
        if let Ok(redis_cache) = redis_cache {
            tracing::info!(
                "获取 KEY:{} 缓存数据成功",
                RedisKeyConstant::ARCHIVE_BLOG_MAP
            );
            return Ok(redis_cache);
        }
        //获取所有文章的日期
        let mut map: ValueMap = ValueMap::new();
        let mut dates = ValueMap::new();
        //1.获取所有文章的日期
        blog::Entity::find()
            .filter(blog::Column::IsPublished.eq(true))
            .order_by_desc(blog::Column::CreateTime)
            .all(db)
            .await?
            .into_iter()
            .for_each(|model| {
                let date = model.create_time.date();
                let date_str = format!("{}年{}月", date.year(), date.month());
                if let rbs::Value::Null = dates.index(date_str.as_str()) {
                    dates.insert(date_str.into(), date.to_string().into());
                }
            });

        for (key, value) in dates {
            let date_str = value
                .as_str()
                .ok_or_else(|| DataBaseError::Custom("归档日期缺失".to_string()))?;
            let date_time = NaiveDate::from_str(date_str)
                .map_err(|e| DataBaseError::Custom(format!("归档日期解析失败:{e}")))?;
            let sql = Statement::from_sql_and_values(
                DbBackend::MySql,
                r#"SELECT id,title,CONCAT(DAY(create_time),"日") as `day`,password
            FROM blog
            WHERE YEAR(create_time) = ?
              AND MONTH(create_time) = ?
              AND is_published = 1;"#,
                [date_time.year().into(), date_time.month().into()],
            );
            let mut blogs = BlogArchive::find_by_statement(sql).all(db).await?;

            for model in blogs.iter_mut() {
                // 按“密码是否非空”判定隐私状态，空字符串密码不再误判为私密
                let has_password = model
                    .password
                    .as_deref()
                    .map(|p| !p.is_empty())
                    .unwrap_or(false);
                model.password = Some("".to_string());
                model.privacy = Some(has_password);
            }
            map.insert(value!(key), value!(blogs));
        }

        if map.len() > 0
            //保存到Redis
            && RedisService::try_set_string(RedisKeyConstant::ARCHIVE_BLOG_MAP.to_string(), &map)
                .await
        {
            tracing::info!(
                "redis KEY:{} 缓存数据成功",
                RedisKeyConstant::ARCHIVE_BLOG_MAP
            );
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
     * 批量加载分类/标签，消除逐条 find_related 的 N+1 查询
     */
    async fn bloginfo_handle(list: &mut Vec<BlogInfo>, db: &DatabaseConnection) {
        let blog_view_map =
            RedisService::get_hash_all::<i64, i32>(RedisKeyConstant::BLOG_VIEWS_MAP.to_string())
                .await
                .unwrap_or_else(|e| {
                    tracing::debug!(
                        "获取 Redis KEY:{} 失败，使用数据库浏览量，错误信息：{}",
                        RedisKeyConstant::BLOG_VIEWS_MAP,
                        e
                    );
                    HashMap::new()
                });

        let ids: Vec<i64> = list.iter().filter_map(|item| item.id).collect();
        if !ids.is_empty() {
            Self::load_related_batch(list, &ids, db).await;
        }

        for item in list.iter_mut() {
            let id: i64 = item.id.unwrap_or_default();

            if blog_view_map.contains_key(&id) {
                item.views = *blog_view_map.get(&id).unwrap_or_else(|| {
                    tracing::error!("获取 Redis KEY:{} 失败", RedisKeyConstant::BLOG_VIEWS_MAP,);
                    &0
                });
            } else {
                //如果Redis没有，则缓存数据
                RedisService::try_set_hash_key::<i32>(
                    RedisKeyConstant::BLOG_VIEWS_MAP.to_string(),
                    id.to_string(),
                    &item.views,
                )
                .await;
            }

            if let Some(password) = &item.password {
                //如果password!=null
                if !password.is_empty() {
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
        }
    }

    /// 批量加载分类与标签并回填 BlogInfo（3 次批量查询替代逐条 N+1）
    async fn load_related_batch(list: &mut [BlogInfo], ids: &[i64], db: &DatabaseConnection) {
        // 1) 批量加载博客模型（取 category_id）
        let blog_models: HashMap<i64, blog::Model> = blog::Entity::find()
            .filter(blog::Column::Id.is_in(ids.iter().copied()))
            .all(db)
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|model| (model.id, model))
            .collect();

        // 2) 批量加载分类
        let category_ids: Vec<i64> = blog_models.values().map(|m| m.category_id).collect();
        let categories: HashMap<i64, category::Model> = category::Entity::find()
            .filter(category::Column::Id.is_in(category_ids))
            .all(db)
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|model| (model.id, model))
            .collect();

        // 3) 批量加载标签（经 blog_tag 关联表）
        let tag_links: Vec<blog_tag::Model> = blog_tag::Entity::find()
            .filter(blog_tag::Column::BlogId.is_in(ids.iter().copied()))
            .all(db)
            .await
            .unwrap_or_default();
        let tag_ids: Vec<i64> = tag_links.iter().map(|link| link.tag_id).collect();
        let tags: HashMap<i64, tag::Model> = if tag_ids.is_empty() {
            HashMap::new()
        } else {
            tag::Entity::find()
                .filter(tag::Column::Id.is_in(tag_ids))
                .all(db)
                .await
                .unwrap_or_default()
                .into_iter()
                .map(|model| (model.id, model))
                .collect()
        };
        let mut tags_by_blog: HashMap<i64, Vec<tag::Model>> = HashMap::new();
        for link in tag_links {
            if let Some(tag_model) = tags.get(&link.tag_id) {
                tags_by_blog
                    .entry(link.blog_id)
                    .or_default()
                    .push(tag_model.clone());
            }
        }

        // 回填
        for item in list.iter_mut() {
            let id = item.id.unwrap_or_default();
            match blog_models.get(&id) {
                Some(blog_model) => {
                    let category_model = categories
                        .get(&blog_model.category_id)
                        .cloned()
                        .unwrap_or_default();
                    item.category = Some(Category::from(category_model));
                    let tag_models = tags_by_blog.get(&id).cloned().unwrap_or_default();
                    item.tags = Some(tag_models.into_iter().map(TagDTO::from).collect());
                }
                None => {
                    tracing::error!("检索到ID：{} 的文章出现异常，无法处理依赖关系", id);
                }
            }
        }
    }

    async fn increment_blog_views(id: i64, db: &DatabaseConnection) -> Option<i32> {
        let update_result = blog::Entity::update_many()
            .col_expr(
                blog::Column::Views,
                Expr::col(blog::Column::Views).add(1).into(),
            )
            .filter(blog::Column::Id.eq(id))
            .exec(db)
            .await;
        match update_result {
            Ok(result) if result.rows_affected > 0 => {}
            Ok(_) => return None,
            Err(e) => {
                tracing::error!("更新文章浏览量失败 id:{} 错误:{}", id, e);
                return None;
            }
        }
        let new_views = blog::Entity::find_by_id(id)
            .one(db)
            .await
            .ok()
            .flatten()
            .map(|blog| blog.views)?;
        RedisService::try_set_hash_key(
            RedisKeyConstant::BLOG_VIEWS_MAP.to_string(),
            id.to_string(),
            &new_views,
        )
        .await;
        Some(new_views)
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
            .paginate(db, search.get_page_size().unwrap_or(10).max(1) as u64);

        let mut map: ValueMap = ValueMap::new();
        let page_list = page
            .fetch_page(search.get_page_num().unwrap_or(1).max(1) as u64 - 1)
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
            Some(blog) => {
                let mut active_model = blog::ActiveModel::from(blog);
                if v.get_appreciation().is_some() {
                    active_model.is_appreciation =
                        ActiveValue::Set(v.get_appreciation().unwrap_or_default());
                }
                if v.get_published().is_some() {
                    active_model.is_published =
                        ActiveValue::Set(v.get_published().unwrap_or_default());
                }
                if v.get_top().is_some() {
                    active_model.is_top = ActiveValue::Set(v.get_top().unwrap_or_default());
                }
                if v.get_password().is_some() {
                    active_model.password = ActiveValue::Set(v.get_password());
                }
                if v.get_recommend().is_some() {
                    active_model.is_recommend =
                        ActiveValue::Set(v.get_recommend().unwrap_or_default());
                }
                if v.get_comment_enabled().is_some() {
                    active_model.is_comment_enabled =
                        ActiveValue::Set(v.get_comment_enabled().unwrap_or_default());
                }
                active_model.update(db).await?;
                Self::clear_blog_cache().await;
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
    pub(crate) async fn find_by_id(
        id: i64,
        db: &DatabaseConnection,
    ) -> Result<BlogDTO, DataBaseError> {
        match blog::Entity::find_by_id(id).one(db).await {
            Ok(Some(blog)) => {
                let mut blog_dto = BlogDTO::from(blog.clone());
                // if blog_dto.get_password().unwrap_or_default() == "" {
                //     blog_dto.set_password(None);
                // }
                blog_dto.related_handle(blog, db).await;
                Ok(blog_dto)
            }
            Ok(None) => Err(DataBaseError::Custom("没有检索到文章".to_string())),
            Err(e) => Err(DataBaseError::MySQLError(e)),
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
                    let mut blog_vo = blog_vo;
                    blog_vo.description = MarkdownParser::description_or_excerpt(
                        &blog_vo.description,
                        &blog_vo.content,
                    );
                    let tag_list = blog_vo.get_tag_list().unwrap_or_default();
                    let mut new_tag_ids = vec![];
                    for tag_type in tag_list {
                        match tag_type {
                            TypeValue::Int32(tag_id) => {
                                new_tag_ids.push(tag_id as i64);
                            }
                            TypeValue::String(tag_name) => {
                                let insert_result = tag::Entity::find()
                                    .filter(tag::Column::TagName.eq(&tag_name))
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
                            // 查询数据库中已有记录，用于保护 create_time/views 不被客户端覆盖
                            let existing =
                                blog::Entity::find_by_id(blog_vo.get_id()).one(conn).await?;
                            let mut active = blog_model.clone().into_active_model();
                            active.is_appreciation = ActiveValue::Set(blog_vo.appreciation);
                            active.category_id = ActiveValue::Set(blog_vo.category_id);
                            active.is_comment_enabled = ActiveValue::Set(blog_vo.comment_enabled);
                            active.is_top = ActiveValue::Set(blog_vo.top);
                            active.is_published = ActiveValue::Set(blog_vo.published);
                            active.is_recommend = ActiveValue::Set(blog_vo.recommend);
                            // views：仅当客户端传入正数时更新，否则保留原值（避免被置 0）
                            if blog_vo.views > 0 {
                                active.views = ActiveValue::Set(blog_vo.views);
                            } else if let Some(existing) = &existing {
                                active.views = ActiveValue::Set(existing.views);
                            }
                            active.words = ActiveValue::Set(blog_vo.words);
                            active.title = ActiveValue::Set(blog_vo.title);
                            active.content = ActiveValue::Set(blog_vo.content);
                            active.password = ActiveValue::Set(blog_vo.password);
                            active.description = ActiveValue::Set(blog_vo.description);
                            active.first_picture = ActiveValue::Set(blog_vo.first_picture);
                            active.read_time = ActiveValue::Set(blog_vo.read_time);
                            // create_time：仅当客户端传入且年份 >= 2000 时才更新，否则保留数据库原值
                            let create_time_usable = blog_vo
                                .create_time
                                .as_ref()
                                .map(|t| t.year() >= 2000)
                                .unwrap_or(false);
                            if create_time_usable {
                                active.create_time =
                                    ActiveValue::Set(blog_vo.create_time.unwrap_or_default());
                            } else if let Some(existing) = &existing {
                                active.create_time = ActiveValue::Set(existing.create_time);
                            }
                            // 更新时间由服务端设置，不信任客户端传入值
                            active.update_time = ActiveValue::Set(Local::now().naive_local());
                            let model = active.update(conn).await?;

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
                                        .filter(blog_tag::Column::BlogId.eq(model.id))
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
        Self::clear_blog_cache().await;
        RedisService::try_del_key(RedisKeyConstant::TAG_CLOUD_LIST).await;
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
                    // 连同删掉该文章下的评论，避免产生孤儿评论
                    crate::entity::comment::Entity::delete_many()
                        .filter(crate::entity::comment::Column::BlogId.eq(id))
                        .exec(conn)
                        .await?;
                    Ok(())
                })
            })
            .await?;

        Self::clear_blog_cache().await;
        RedisService::try_del_key(RedisKeyConstant::TAG_CLOUD_LIST).await;
        Ok(result)
    }

    pub async fn clear_blog_cache() {
        RedisService::try_del_key(RedisKeyConstant::HOME_BLOG_INFO_LIST).await;
        RedisService::try_del_key(RedisKeyConstant::BLOG_DETAIL_MAP).await;
        RedisService::try_del_key(RedisKeyConstant::CATEGORY_BLOG_LIST).await;
        RedisService::try_del_key(RedisKeyConstant::TAG_BLOG_LIST).await;
        RedisService::try_del_key(RedisKeyConstant::RANDOM_BLOG_LIST).await;
        RedisService::try_del_key(RedisKeyConstant::NEW_BLOG_LIST).await;
        RedisService::try_del_key(RedisKeyConstant::ARCHIVE_BLOG_MAP).await;
    }

    /**
     * 搜索博文
     */
    pub async fn search_content(
        content: String,
        db: &DatabaseConnection,
    ) -> Result<Vec<SearchBlog>, DataBaseError> {
        let keyword = content.trim();
        if keyword.is_empty() {
            return Err(DataBaseError::Custom("搜索关键词不能为空".to_string()));
        }
        // LIKE 通配符转义（% _ \），防止扩大匹配范围
        let escaped_like = keyword
            .replace('\\', "\\\\")
            .replace('%', "\\%")
            .replace('_', "\\_");
        let like = format!("%{}%", escaped_like);
        // 关键词作为字面量构建正则，防止正则注入与 ReDoS
        let keyword_escaped = regex::escape(keyword);
        let pattern = format!(
            r"[\u4E00-\u9FA5A-Za-z0-9_,，。\n\s*\r\t]{{0,10}}{}[\u4E00-\u9FA5A-Za-z0-9_,，。\n\s*\r\t]{{0,10}}",
            keyword_escaped
        );
        let regex_builder = regex::RegexBuilder::new(&pattern)
            .case_insensitive(true)
            .build()?;
        let mut models = blog::Entity::find()
            .filter(blog::Column::IsPublished.eq(true))
            .filter(blog::Column::Content.contains(like))
            // 密码保护文章（password 非空）不进公开搜索，避免标题与摘要泄露
            .filter(
                Condition::any()
                    .add(blog::Column::Password.is_null())
                    .add(blog::Column::Password.eq("")),
            )
            // 限制返回数量，避免全表扫描导致内存/响应过大
            .limit(Some(20))
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
                    tracing::info!("search_blog 未找到关键词:{:?}", keyword);
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
                tracing::error!("分类下 {} 没有检索到文章", category_id);
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
            Err(DataBaseError::Custom(format!(
                "没有检索到文章ID:{blog_id}的关联评论"
            )))
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
        //         tracing::error!("search_blog Index 获取失败:{:?}", find_str);
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
    //         let date_str = value
    //             .as_str()
    //             .ok_or_else(|| DataBaseError::Custom("归档日期缺失".to_string()))?;
    //         let date_time = NaiveDate::from_str(date_str)
    //             .map_err(|e| DataBaseError::Custom(format!("归档日期解析失败:{e}")))?;
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
