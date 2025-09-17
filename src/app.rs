/*
 * @Author: lurendie 549700459@qq.com
 * @Date: 2024-03-26 00:08:12
 * @LastEditors: lurendie
 */
use crate::config::Config;
use crate::controller::{
    about_controller, admin, archive_controller, blog_controller, comment_controller,
    friend_controller, index_controller, moment_controller, user_controller,
};
use crate::middleware::{AppClaims, VisiLog, JWT};
//use actix_cors::Cors;
use actix_jwt_session::{Duration, JwtTtl, RefreshTtl};
// use actix_web::http::header::{
//     ACCEPT, ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_ORIGIN,
//     ACCESS_CONTROL_EXPOSE_HEADERS, ACCESS_CONTROL_MAX_AGE, ACCESS_CONTROL_REQUEST_HEADERS,
//     AUTHORIZATION, CONTENT_TYPE,
// };
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
pub struct AppServer;
/**
 * Application
 *
 */
impl AppServer {
    /**
     * run 服务启动
     */
    pub async fn run(conf: &'static Config) -> std::io::Result<()> {
        let (storage, factory) = JWT::create::<AppClaims>();
        //创建JWT
        let jwt_ttl = JwtTtl(Duration::days(1));
        let refresh_ttl = RefreshTtl(Duration::days(1));
        HttpServer::new(move || {
            // 配置 CORS
            // let cors = Cors::default()
            //     //.allowed_origin(conf.server.front_adderss.as_str()) // 只允许 example.com 域进行跨域请求,暂时不开启
            //     .allow_any_origin() //允许所有域访问
            //     .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"]) // 允许的 HTTP 方法
            //     .allowed_headers(vec![
            //         AUTHORIZATION,
            //         ACCEPT,
            //         ACCESS_CONTROL_ALLOW_ORIGIN,
            //         CONTENT_TYPE,
            //         ACCESS_CONTROL_REQUEST_HEADERS,
            //         ACCESS_CONTROL_ALLOW_HEADERS,
            //         ACCESS_CONTROL_MAX_AGE,
            //         ACCESS_CONTROL_EXPOSE_HEADERS,
            //         HeaderName::from_str("Identification").unwrap(), //自定义请求头
            //                                                          //HeaderName::from_str("Authorization").unwrap(),
            //     ])
            //     .max_age(3600); // 设置 preflight 缓存的最大时间
            //创建App
            App::new()
                .app_data(Data::new(jwt_ttl))
                .app_data(Data::new(refresh_ttl))
                //中间件
                //.wrap(cors)
                .wrap(VisiLog::default())
                .wrap(Logger::default()) //允许跨域请求
                //service层
                .service(index_controller::site)
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
                .service(blog_controller::moment_like)
                //admin
                .service(
                    web::scope("/admin")
                        .app_data(Data::new(storage.clone()))
                        .wrap(factory.clone())
                        .service(user_controller::login)
                        .service(admin::dashboard_controller::dashboard) //.default_service(web::to(adminIndexController::default)),
                        .service(admin::blog_controller::blogs)
                        .service(admin::blog_controller::visibility)
                        .service(admin::blog_controller::top)
                        .service(admin::blog_controller::recommend)
                        .service(admin::blog_controller::category_and_tag)
                        .service(admin::blog_controller::blog)
                        .service(admin::blog_controller::update_blog)
                        .service(admin::blog_controller::create_blog)
                        .service(admin::blog_controller::delete_blog),
                )
                .default_service(web::to(index_controller::default))
        })
        .bind(format!("{}:{}", conf.server.host, conf.server.port))?
        .run()
        .await
    }
}
