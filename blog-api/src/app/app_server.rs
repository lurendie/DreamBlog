/*
 * @Author: lurendie 549700459@qq.com
 * @Date: 2024-03-26 00:08:12
 * @LastEditors: lurendie
 */
use super::app_config::CONFIG;
use super::app_state::{self, AppState};
use super::JobRunner;
use crate::controller::{
    about_controller,
    admin::{self, tag_controller},
    archive_controller, blog_controller, comment_controller, friend_controller, index_controller,
    moment_controller, sitemap_controller, user_controller,
};
use crate::middleware::build_session_storage;
use crate::middleware::{ExceptionLog, OperationLog, VisiLog};
use actix_cors::Cors;
use actix_jwt_session::{Duration, JwtTtl, RefreshTtl};
use actix_web::web::Data;
use actix_web::http::header::{self, HeaderName};
use actix_web::middleware::Condition;
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
        let bind_host = server_config.host.clone();
        let bind_port = server_config.port;
        //创建JWT TTL
        let jwt_ttl = JwtTtl(Duration::days(server_config.token_expires));
        let refresh_ttl = RefreshTtl(Duration::days(server_config.token_expires));

        //Appstate
        let mut app_state = AppState::new(app_state::get_connection().await);
        //访问日志异步写入器：请求路径只入队，后台批量落库
        app_state.visit_log_writer = Some(crate::service::VisitLogWriter::start(
            app_state.get_mysql_pool().clone(),
        ));
        let scheduler_state = app_state.clone();
        tokio::spawn(async move {
            JobRunner::start(scheduler_state).await;
        });
        let (session_storage, factory) = build_session_storage();
        HttpServer::new(move || {
            let cors = Self::build_cors(&server_config);
            //创建App
            App::new()
                .wrap(ExceptionLog::default())
                .wrap(Condition::new(server_config.cors_enabled, cors))
                .app_data(Data::new(jwt_ttl))
                .app_data(Data::new(refresh_ttl))
                .app_data(Data::new(app_state.clone()))
                .app_data(Data::new(session_storage.clone()))
                .service(
                    web::scope("/blog")
                        .wrap(VisiLog::default())
                        .wrap(factory.clone())
                        .configure(Self::view_router),
                )
                .service(
                    web::scope("/admin")
                        .wrap(OperationLog::default())
                        .wrap(factory.clone())
                        .configure(Self::cms_router),
                )
                .default_service(web::to(index_controller::default))
        })
        .bind_auto_h2c(format!("{}:{}", bind_host, bind_port))?
        .run()
        .await
    }

    fn build_cors(server_config: &super::app_config::ServerConfig) -> Cors {
        let mut cors = Cors::default()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                header::AUTHORIZATION,
                header::ACCEPT,
                header::CONTENT_TYPE,
                header::ORIGIN,
                HeaderName::from_static("identification"),
            ])
            .supports_credentials()
            .max_age(3600);

        for origin in server_config.cors_origins() {
            cors = cors.allowed_origin(&origin);
        }

        cors
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
            .service(sitemap_controller::robots)
            .service(sitemap_controller::sitemap)
            .service(comment_controller::get_comments)
            .service(blog_controller::check_blog_password)
            .service(user_controller::login)
            .service(blog_controller::search_blog)
            .service(moment_controller::moment_like)
            .service(comment_controller::save_comment)
            .service(user_controller::logout);
    }

    /**
     * 后台路由
     */
    fn cms_router(cfg: &mut web::ServiceConfig) {
        cfg.service(user_controller::login)
            .service(user_controller::logout)
            .service(admin::dashboard_controller::dashboard) //.default_service(web::to(adminIndexController::default)),
            .service(admin::about_controller::get_about)
            .service(admin::about_controller::update_about)
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
            .service(admin::moment_controller::create_and_update)
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
            .service(admin::comment_controller::update_comment_published)
            .service(admin::comment_controller::update_comment_notice)
            .service(admin::account_controller::change_account)
            .service(admin::friend_controller::get_friend_info)
            .service(admin::friend_controller::get_friends_by_query)
            .service(admin::friend_controller::update_friend_published)
            .service(admin::friend_controller::update_friend)
            .service(admin::friend_controller::delete_friend_by_id)
            .service(admin::friend_controller::save_friend)
            .service(admin::friend_controller::update_friend_comment_enabled)
            .service(admin::friend_controller::update_friend_content)
            .service(admin::schedule_job_controller::get_job_list)
            .service(admin::schedule_job_controller::update_job_status)
            .service(admin::schedule_job_controller::run_job_once)
            .service(admin::schedule_job_controller::edit_job)
            .service(admin::schedule_job_controller::delete_job_by_id)
            .service(admin::schedule_job_controller::add_job)
            .service(admin::schedule_job_controller::get_job_log_list)
            .service(admin::schedule_job_controller::delete_job_log_by_log_id)
            .service(admin::site_setting_controller::get_site_setting_data)
            .service(admin::site_setting_controller::update_site_settings)
            .service(admin::site_setting_controller::get_web_title_suffix)
            .service(admin::exception_log_controller::get_exception_log_list)
            .service(admin::exception_log_controller::delete_exception_log_by_id)
            .service(admin::login_log_controller::get_login_log_list)
            .service(admin::login_log_controller::delete_login_log_by_id)
            .service(admin::operation_log_controller::get_operation_log_list)
            .service(admin::operation_log_controller::delete_operation_log_by_id)
            .service(admin::visit_log_controller::get_visit_log_list)
            .service(admin::visit_log_controller::delete_visit_log_by_id)
            .service(admin::visitor_controller::get_visitor_list)
            .service(admin::visitor_controller::delete_visitor)
            .service(admin::picture_hosting_controller::get_configs)
            .service(admin::picture_hosting_controller::github_user)
            .service(admin::picture_hosting_controller::save_github_config)
            .service(admin::picture_hosting_controller::save_upyun_config)
            .service(admin::picture_hosting_controller::save_txyun_config)
            .service(admin::picture_hosting_controller::delete_config)
            .service(admin::picture_hosting_controller::github_repos)
            .service(admin::picture_hosting_controller::github_contents)
            .service(admin::picture_hosting_controller::github_delete)
            .service(admin::picture_hosting_controller::github_upload)
            .service(admin::picture_hosting_controller::upyun_contents)
            .service(admin::picture_hosting_controller::upyun_delete)
            .service(admin::picture_hosting_controller::upyun_upload)
            .service(admin::picture_hosting_controller::txyun_contents)
            .service(admin::picture_hosting_controller::txyun_delete)
            .service(admin::picture_hosting_controller::txyun_upload);
    }
}
