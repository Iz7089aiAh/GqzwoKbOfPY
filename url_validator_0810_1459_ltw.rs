// url_validator.rs

use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use url::Url;
use std::fmt;

// 定义UrlValidationError用于表示URL验证错误
#[derive(Debug)]
struct UrlValidationError(&'static str);

// 实现Responder特性，以便UrlValidationError可以被Actix处理
impl Responder for UrlValidationError {
    fn respond_to(self, _req: &HttpRequest) -> HttpResponse {
        HttpResponse::BadRequest().json(self.0)
    }
}

// URL验证函数
async fn validate_url(url_str: web::Json<String>) -> Result<HttpResponse, UrlValidationError> {
    match Url::parse(&url_str.into_inner()) {
        Ok(_) => Ok(HttpResponse::Ok().json("URL is valid".to_string())),
        Err(_) => Err(UrlValidationError("Invalid URL")),
    }
}

// 主函数，设置HTTP服务器
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 定义路由和处理函数
    HttpServer::new(|| {
        App::new()
            .route("/validate", web::post().to(validate_url))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
