// backup_sync_tool.rs
// 文件备份和同步工具
// 使用 Rust 和 Actix 框架实现

use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use std::fs;
use std::path::Path;
use std::io::{self, ErrorKind};
use std::sync::Arc;
use tokio::sync::Mutex;

// 文件备份和同步的配置结构
struct BackupConfig {
    src_path: String,  // 源路径
    dst_path: String,  // 目标路径
}

// 备份和同步文件的异步函数
async fn backup_and_sync_file(config: web::Data<Arc<Mutex<BackupConfig>>>) -> impl Responder {
    let config = config.lock().await;
    let source_path = Path::new(&config.src_path);
    let destination_path = Path::new(&config.dst_path);

    // 检查源文件是否存在
    if !source_path.exists() {
        return HttpResponse::BadRequest().body("Source file does not exist.");
    }

    // 尝试复制文件到目标路径
    match fs::copy(&source_path, &destination_path) {
        Ok(_) => HttpResponse::Ok().body("File successfully backed up and synced."),
        Err(e) => match e.kind() {
            ErrorKind::NotFound => HttpResponse::NotFound().body("Destination path not found."),
            _ => HttpResponse::InternalServerError().body("An error occurred during backup and sync."),
        },
    }
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    // 设置 Actix Web 服务器
    let config = BackupConfig {
        src_path: "./src_file.txt".to_string(),
        dst_path: "./dst_file.txt".to_string(),
    };
    let config = Arc::new(Mutex::new(config));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .route("/backup", web::get().to(backup_and_sync_file))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
