//配置项
use blog_api::AppServer;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //2. Service run
    AppServer::run().await
}
