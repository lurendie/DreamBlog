//配置项
use blog_api::AppServer;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //1. 初始化日志（tracing：stdout + logs/blog-api.log 滚动文件）
    blog_api::init_logger();
    //2. Service run
    AppServer::run().await
}
