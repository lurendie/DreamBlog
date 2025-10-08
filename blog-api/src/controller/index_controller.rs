use crate::app::AppState;
use crate::error::{AppError, WebError};
use crate::model::ApiResponse;
use crate::service::{BlogService, CategoryService, SiteSettingService, TagService};
use actix_web::{routes, web};
use rbs::{value, Value};
/**
   Site 数据
*/
#[routes]
#[get("/site")]
#[options("/site")]
pub async fn site(app: web::Data<AppState>) -> Result<ApiResponse<Value>, AppError> {
    let connect = app.get_mysql_pool();
    // 获取站点信息
    let mut map = SiteSettingService::find_site_info(connect).await?;
    // 获取分类列表
    let category_list = CategoryService::get_list(connect).await?;
    // 获取随机博客列表
    let random_list = BlogService::find_list_random(connect).await?;
    // 获取最新博客列表
    let new_list = BlogService::find_list_new(connect).await?;
    // 获取标签列表
    let tag_list = TagService::get_tags(connect).await?;
    // 组合数据
    map.insert(value!("newBlogList"), value!(new_list));
    map.insert(value!("categoryList"), value!(category_list));
    map.insert(value!("tagList"), value!(tag_list));
    map.insert(value!("randomBlogList"), value!(random_list));

    Ok(ApiResponse::success(Some(value!(map))))
}

pub async fn default() -> Result<ApiResponse<Value>, AppError> {
    //error!("404,找不到页面");
    Err(AppError::WebError(WebError::NotFound("404".to_string())))
}
