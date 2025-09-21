use actix_web::{get, HttpResponse, Responder, web, App, HttpServer};
use sha2::{Sha256, Digest};
use std::str;
use std::io::{self, Read};
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;

// 定义结构体，包含哈希计算功能
struct HashCalculator;

#[get("/hash")]
async fn calculate_hash(content: web::Query<Content>) -> impl Responder {
    let content = content.into_inner();
    let hash_result = calculate_sha256(content.text);
    match hash_result {
        Ok(hash) => HttpResponse::Ok().json(HashResponse { hash }),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

impl HashCalculator {
    // 计算SHA-256哈希值
    fn calculate_sha256(input: &str) -> Result<String, io::Error> {
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        Ok(format!("%x", hasher.finalize()))
    }
}

// 用于查询参数的结构体
#[derive(serde::Deserialize)]
struct Content {
    text: String,
}

// 用于响应的结构体
#[derive(serde::Serialize)]
struct HashResponse {
    hash: String,
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(calculate_hash)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
