use crate::app::AppState;
use crate::error::WebErrorCode;
use crate::model::ApiResponse;
use crate::service::{BlogService, CategoryService, SiteSettingService, TagService};
use actix_web::{routes, web, Responder};
use rbs::value;
/**
   Site 数据
*/
#[routes]
#[get("/site")]
#[options("/site")]
pub async fn site(app: web::Data<AppState>) -> impl Responder {
    let connect = app.get_mysql_pool();

    // 获取站点信息
    let mut map = match SiteSettingService::find_site_info(connect).await {
        Ok(data) => data,
        Err(e) => {
            return ApiResponse::<String>::error_with_code(
                WebErrorCode::DATABASE_ERROR,
                e.to_string(),
            )
            .json()
        }
    };

    // 获取分类列表
    let category_list = match CategoryService::get_list(connect).await {
        Ok(data) => data,
        Err(e) => {
            return ApiResponse::<String>::error_with_code(
                WebErrorCode::DATABASE_ERROR,
                e.to_string(),
            )
            .json()
        }
    };

    // 获取随机博客列表
    let random_list = match BlogService::find_list_random(connect).await {
        Ok(data) => data,
        Err(e) => {
            return ApiResponse::<String>::error_with_code(
                WebErrorCode::DATABASE_ERROR,
                e.to_string(),
            )
            .json()
        }
    };

    // 获取最新博客列表
    let new_list = match BlogService::find_list_new(connect).await {
        Ok(data) => data,
        Err(e) => {
            return ApiResponse::<String>::error_with_code(
                WebErrorCode::DATABASE_ERROR,
                e.to_string(),
            )
            .json()
        }
    };

    // 获取标签列表
    let tag_list = match TagService::get_tags(connect).await {
        Ok(data) => data,
        Err(e) => {
            return ApiResponse::<String>::error_with_code(
                WebErrorCode::DATABASE_ERROR,
                e.to_string(),
            )
            .json()
        }
    };

    // 组合数据
    map.insert(value!("newBlogList"), value!(new_list));
    map.insert(value!("categoryList"), value!(category_list));
    map.insert(value!("tagList"), value!(tag_list));
    map.insert(value!("randomBlogList"), value!(random_list));

    ApiResponse::success(Some(value!(map))).json()
}

pub async fn default() -> impl Responder {
    //error!("404,找不到页面");
    ApiResponse::<String>::error_with_code(WebErrorCode::NOT_FOUND, "Error Not Found".to_string())
        .json()
}
