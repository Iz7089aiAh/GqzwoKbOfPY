use actix_web::{get, HttpResponse, Responder, web, App, HttpServer};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use serde_json::json;

/// 定义分析文本文件的功能
#[get("/analyze/{filepath}")]
async fn analyze_file(filepath: web::Path<String>) -> impl Responder {
    let path = Path::new(&filepath.into_inner());
    match analyze_text_file(path) {
        Ok(content) => HttpResponse::Ok().json(json!(content)),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": e.to_string(),
        })),
    }
}

/// 分析文本文件并返回分析结果
fn analyze_text_file(path: &Path) -> io::Result<serde_json::Value> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut content = Vec::new();
    for line in reader.lines() {
        let line = line?;
        content.push(line);
    }
    Ok(json!({
        "lines": content,
    }))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new().service(analyze_file)
    }).
    listen("127.0.0.1:8080")?.
    run().await
}
