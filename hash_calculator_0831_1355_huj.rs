use actix_web::{get, HttpResponse, Responder, web, App, HttpServer, Responder as ActixResponder};
use sha2::{Sha256, Digest};
use hex::encode as hex_encode;

// 哈希值计算工具结构体
struct HashCalculator;

// 实现哈希值计算工具的方法
impl HashCalculator {
    #[get("/hash/{input}")]
    async fn calculate(&self, input: web::Path<String>) -> ActixResponder {
        let input_str = input.into_inner();
        let hash = HashCalculator::compute_hash(&input_str);
        HttpResponse::Ok().json(hash)
    }

    // 计算 SHA-256 哈希值
    fn compute_hash(input: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(input);
        let result = hasher.finalize();
        hex_encode(result)
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(HashCalculator::calculate)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// 哈希值计算工具的文档
#[doc = "哈希值计算工具"]
struct HashCalculator;

#[doc = "计算给定输入的 SHA-256 哈希值"]
impl HashCalculator {
    #[doc = "计算给定输入的 SHA-256 哈希值"]
    fn compute_hash(input: &str) -> String {
        // 省略实现细节...
    }
}
