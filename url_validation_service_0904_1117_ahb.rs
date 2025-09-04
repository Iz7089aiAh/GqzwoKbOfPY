use actix_web::{get, HttpResponse, Responder, web};
use std::error::Error;
use url::Url;

/// 检查URL是否有效
/// 路由为 /validate_url/{url}
#[get("/validate_url/{url}")]
async fn validate_url(url: web::Path<String>) -> impl Responder {
    // 尝试解析URL
    match Url::parse(&url.into_inner()) {
        Ok(_url) => HttpResponse::Ok().json({"valid": true}),
        Err(_) => HttpResponse::BadRequest().json({"valid": false}),
    }
}

/// 定义APP的主函数
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 启动服务
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .service(validate_url)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
