use actix_web::{get, HttpResponse, Responder, web, App, HttpServer};
use std::fs;
use std::io::{self, ErrorKind};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::Mutex;
use serde_json::json;

// 结构体用于存储文件的源路径和目标路径
struct FileBackupConfig {
    src_path: PathBuf,
    dst_path: PathBuf,
}

// 异步函数用于备份文件
async fn backup_file(config: web::Data<Arc<Mutex<FileBackupConfig>>>) -> impl Responder {
    let config = config.lock().await;
    match fs::copy(&config.src_path, &config.dst_path) {
        Ok(_) => HttpResponse::Ok().json(json!({"message": "File backup successful"})),
        Err(e) => match e.kind() {
            ErrorKind::NotFound => HttpResponse::NotFound().json(json!({"error": "Source file not found"})),
            _ => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
        },
    }
}

// 异步函数用于同步文件
async fn sync_file(config: web::Data<Arc<Mutex<FileBackupConfig>>>) -> impl Responder {
    let config = config.lock().await;
    // 这里可以添加更复杂的同步逻辑，例如检查文件差异等
    match fs::copy(&config.src_path, &config.dst_path) {
        Ok(_) => HttpResponse::Ok().json(json!({"message": "File sync successful"})),
        Err(e) => match e.kind() {
            ErrorKind::NotFound => HttpResponse::NotFound().json(json!({"error": "Source file not found"})),
            _ => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
        },
    }
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    // 设置源路径和目标路径
    let config = FileBackupConfig {
        src_path: PathBuf::from("./src_file.txt"),
        dst_path: PathBuf::from("./dst_file.txt"),
    };
    let config = Arc::new(Mutex::new(config));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .route("/backup", get().to(backup_file))
            .route("/sync", get().to(sync_file))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
