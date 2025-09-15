use actix_web::{web, App, HttpServer, HttpResponse, Responder, post, get};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use csv::ReaderBuilder;
use serde::Deserialize;
use serde_json::json;

// 定义记录结构体，并自动从CSV文件的标题行获取字段名称
#[derive(Debug, Deserialize)]
struct Record {
    // 示例字段，实际字段根据CSV文件结构定义
    field1: String,
    field2: String,
}

// 实现CSV记录处理函数
fn process_csv_record(record: Record) -> Result<String, String> {
    // 这里添加具体的处理逻辑
    // 例如，你可以添加一些字段的验证，或者进行一些计算
    // 暂时返回一个简单的String作为示例
    Ok(format!("Processed record: {} {}", record.field1, record.field2))
}

// 实现处理整个CSV文件的函数
async fn process_csv_file(file: &str) -> impl Responder {
    let file = File::open(file).map_err(|e| e.to_string())?;
    let mut rdr = ReaderBuilder::new().delimiter(b',').from_reader(file);
    let mut results = Vec::new();

    for result in rdr.deserialize() {
        let record: Record = result.map_err(|e| e.to_string())?;
        match process_csv_record(record) {
            Ok(message) => results.push(message),
            Err(e) => return HttpResponse::InternalServerError().json(json!{"error": e}),
        }
    }

    HttpResponse::Ok().json(json!{"results": results})
}

// Actix Web服务入口
#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // 定义一个POST路由，用于上传CSV文件
            .service(post "/process_csv")
            // 定义一个GET路由，用于获取处理结果
            .service(get "/results")
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
