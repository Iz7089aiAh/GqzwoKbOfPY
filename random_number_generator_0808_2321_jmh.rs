use actix_web::{get, HttpResponse, Responder, web, App, HttpServer};
use rand::Rng;
use serde::Deserialize;
use serde_json::json;

/// RandomNumberGeneratorHandler 负责处理随机数生成的请求
#[derive(Deserialize)] // 允许从请求中反序列化参数
struct GenerateRandomNumber {
    /// 指定随机数的范围
    range: Option<(u32, u32)>,
}

/// 随机数生成器的处理函数
#[get("/random")]
async fn random_number_handler() -> impl Responder {
    let mut rng = rand::thread_rng();
    let number: u32 = rng.gen();
    HttpResponse::Ok().json(json!{"random_number": number})
}

/// 带范围的随机数生成器的处理函数
#[get("/random")]
async fn random_number_range_handler(payload: web::Json<GenerateRandomNumber>) -> impl Responder {
    match payload.range {
        Some((min, max)) => {
            if max < min {
                return HttpResponse::BadRequest().json(json!{"error": "Invalid range"});
            }
            let mut rng = rand::thread_rng();
            let number: u32 = rng.gen_range(min..max);
            HttpResponse::Ok().json(json!{"random_number": number})
        },
        None => HttpResponse::BadRequest().json(json!{"error": "Range is required"}),
    }
}

/// 程序的入口点
#[actix_web::main] // 标记为程序入口点
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(random_number_handler)
            .service(random_number_range_handler)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
