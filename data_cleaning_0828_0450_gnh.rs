use actix_web::{get, HttpResponse, Responder, web};
use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;

// 定义请求参数结构体
#[derive(Deserialize)]
pub struct CleanParams {
    data: String,
}

// 定义数据清洗操作的结果结构体
#[derive(Serialize)]
pub struct CleanResult {
    cleaned_data: String,
}

// 数据清洗函数
fn clean_data(input: &str) -> Result<String, String> {
    // 这里可以添加具体的数据清洗逻辑，例如去除空格、特殊字符等
    Ok(input.trim().to_string())
}

// Actix Web 服务
#[get("/clean")]
async fn clean_data_endpoint(params: web::Json<CleanParams>) -> impl Responder {
    match clean_data(&params.data) {
        Ok(cleaned_data) => HttpResponse::Ok().json(CleanResult {
            cleaned_data,
        }),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": e,
        })),
    }
}

// 定义Actix Web服务
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 设置服务配置
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            // 注册服务端点
            .service(clean_data_endpoint)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
