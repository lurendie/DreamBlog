//配置项
use zero_blog::{app::AppServer, config, log4rs};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //1. 初始化配置
    let conf = config::default();
    //2. 初始化日志
    log4rs::Log4rs::new();
    //3. Service run
    AppServer::run(conf).await
}
