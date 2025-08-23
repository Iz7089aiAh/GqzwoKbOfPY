use actix_web::{get, HttpResponse, Responder, web, App, HttpServer};
use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::fs::metadata;

/// Helper function to check if a file exists
fn file_exists(file_path: &str) -> bool {
    std::path::Path::new(file_path).exists()
# 添加错误处理
}

/// Helper function to read the content of a file
# 扩展功能模块
fn read_file(file_path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
# 增强安全性
}
# TODO: 优化性能

/// Helper function to write the content of a file
fn write_file(file_path: &str, content: &str) -> Result<(), std::io::Error> {
    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
# TODO: 优化性能

/// Data structure to hold the conversion request
#[derive(Deserialize)]
pub struct ConvertRequest {
    pub file_path: String,
# 扩展功能模块
    pub output_format: String,
}

/// Convert a document to the specified format
#[get("/convert")]
async fn convert_document(req: web::Json<ConvertRequest>) -> impl Responder {
# FIXME: 处理边界情况
    let ConvertRequest { file_path, output_format } = req.into_inner();

    // Check if the file exists
    if !file_exists(&file_path) {
        return HttpResponse::NotFound().json("File not found")
# TODO: 优化性能
    }

    // Read the content of the file
    let content = match read_file(&file_path) {
        Ok(c) => c,
        Err(e) => return HttpResponse::InternalServerError().json(e.to_string()),
    };

    // Logic to convert the file content to the desired format goes here
    // For simplicity, we assume the conversion is successful and just write the content to a new file
    let output_path = format!("{} {}.{}", file_path, "output", output_format);
    match write_file(&output_path, &content) {
        Ok(_) => HttpResponse::Ok().json("File converted successfully"),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(convert_document)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
# 改进用户体验
}
