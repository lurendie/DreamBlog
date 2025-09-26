/*
 * @Author: lurendie 549700459@qq.com
 * @Date: 2024-03-26 00:08:12
 * @LastEditors: lurendie
 */
use super::app_config::CONFIG;
use super::app_state::{self, AppState};
use super::RedisClient;
use crate::controller::{
    about_controller,
    admin::{self, tag_controller},
    archive_controller, blog_controller, comment_controller, friend_controller, index_controller,
    moment_controller, user_controller,
};
use crate::middleware::{AppClaims, VisiLog};
use actix_jwt_session::{Duration, Extractors, JwtTtl, RefreshTtl, UseJwt, JWT_HEADER_NAME};
//use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};

pub struct AppServer;

/**
 * Application Server Implementation
 *
 */
impl AppServer {
    /**
     * run 服务启动
     */
    pub async fn run() -> std::io::Result<()> {
        let server_config = CONFIG.get_server_config();
        //创建JWT
        let jwt_ttl = JwtTtl(Duration::days(server_config.token_expires));
        let refresh_ttl = RefreshTtl(Duration::days(server_config.token_expires));

        //Appstate
        let app_state = AppState::new(
            app_state::get_connection().await,
            //,
            // CONFIG.clone(),
        );
        let redis_pool = RedisClient::get_redis_pool().await;
        let app_data = Data::new(app_state.clone());
        HttpServer::new(move || {
            //创建App
            App::new()
                .app_data(Data::new(jwt_ttl))
                .app_data(Data::new(refresh_ttl))
                .app_data(app_data.clone())
                .use_jwt::<AppClaims>(
                    Extractors::default().with_jwt_header(JWT_HEADER_NAME),
                    Some(redis_pool.clone()),
                )
                .wrap(VisiLog::default())
                //.wrap(Logger::default())
                //.wrap(ErrorHandler::default())
                .configure(Self::view_router)
                //admin
                .service(web::scope("/admin/").configure(Self::admin_router))
                .default_service(web::to(index_controller::default))
        })
        .bind_auto_h2c(format!("{}:{}", server_config.host, server_config.port))?
        .run()
        .await
    }
    /**
     * 前台路由
     */
    fn view_router(cfg: &mut web::ServiceConfig) {
        //service层
        cfg.service(index_controller::site)
            .service(blog_controller::blogs)
            .service(blog_controller::category)
            .service(blog_controller::blog)
            .service(blog_controller::tag)
            .service(archive_controller::archives)
            .service(moment_controller::moments)
            .service(about_controller::about)
            .service(friend_controller::get_friend)
            .service(comment_controller::get_comments)
            .service(blog_controller::check_blog_password)
            .service(user_controller::login)
            .service(blog_controller::search_blog)
            .service(moment_controller::moment_like)
            .service(comment_controller::save_comment);
    }

    /**
     * 后台路由
     */
    fn admin_router(cfg: &mut web::ServiceConfig) {
        cfg.service(user_controller::login)
            .service(admin::dashboard_controller::dashboard) //.default_service(web::to(adminIndexController::default)),
            .service(admin::blog_controller::blogs)
            .service(admin::blog_controller::visibility)
            .service(admin::blog_controller::top)
            .service(admin::blog_controller::recommend)
            .service(admin::blog_controller::category_and_tag)
            .service(admin::blog_controller::blog)
            .service(admin::blog_controller::update_blog)
            .service(admin::blog_controller::create_blog)
            .service(admin::blog_controller::delete_blog)
            .service(admin::moment_controller::moments)
            .service(admin::moment_controller::moment_published)
            .service(admin::moment_controller::delete_moment)
            .service(admin::moment_controller::get_moment_by_id)
            .service(admin::moment_controller::update_moment)
            .service(admin::moment_controller::create_moment)
            .service(admin::category_controller::categories)
            .service(admin::category_controller::update_category)
            .service(admin::category_controller::delete_category)
            .service(admin::tag_controller::get_all_tags)
            .service(tag_controller::insert_or_update)
            .service(tag_controller::delete_by_id)
            .service(admin::comment_controller::find_comments)
            .service(admin::comment_controller::find_blog_id_and_title)
            .service(admin::comment_controller::delete_comment)
            .service(admin::comment_controller::update_comment)
            .service(admin::account_controller::change_account)
            .service(admin::friend_controller::get_friend_info)
            .service(admin::friend_controller::update_friend)
            .service(admin::friend_controller::delete_friend_by_id)
            .service(admin::friend_controller::save_friend)
            .service(admin::schedule_controller::get_job_list)
            .service(admin::schedule_controller::update_job_status)
            .service(admin::schedule_controller::delete_job_by_id)
            .service(admin::schedule_controller::add_job)
            .service(admin::site_setting_controller::get_site_setting_data)
            .service(admin::site_setting_controller::update_site_settings)
            .service(admin::visit_log_controller::get_visit_log_list)
            .service(admin::visit_log_controller::delete_visit_log_by_id)
            .service(admin::visitor_controller::get_visitor_list)
            .service(admin::visitor_controller::delete_visitor);
    }
}
