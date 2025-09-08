use actix_web::{get, HttpResponse, Responder};
use url::Url;
use std::env;

/// 定义一个简单的HTTP服务，用于验证URL链接的有效性
#[get("/validate_url/{url}")]
async fn validate_url(url: String) -> impl Responder {
    // 尝试解析传入的URL字符串
    match Url::parse(&url) {
        Ok(_) => HttpResponse::Ok().json({"message": "URL is valid"}),
        Err(_) => HttpResponse::BadRequest().json({"message": "Invalid URL"}),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 设置环境变量以启用日志记录
    env::set_var("RUST_LOG", "actix_web=info");
    env::set_var("RUST_BACKTRACE", "1");

    // 初始化日志记录器
    env_logger::init();

    // 启动服务器，监听localhost的8080端口
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            // 注册validate_url路由处理器
            .service(validate_url)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}