//配置项
use zero_blog::AppServer;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //2. Service run
    AppServer::run().await
}
