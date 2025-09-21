use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::fs;
use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use serde_json::json;
use actix_web::error::ErrorBadRequest;

// 定义备份文件的路径和备份文件名
const BACKUP_DIR: &str = "./backups";
const BACKUP_FILE_NAME: &str = "data_backup.bin";

// 定义备份数据结构
#[derive(Serialize, Deserialize, Debug)]
struct BackupData {
    data: String,
}

// 实现备份和恢复的业务逻辑
struct BackupRestoreService;

impl BackupRestoreService {
    // 备份数据
    fn backup_data(&self, data: &str) -> Result<(), std::io::Error> {
        let backup_path = Path::new(BACKUP_DIR).join(BACKUP_FILE_NAME);
        fs::write(backup_path, data)
    }

    // 恢复数据
    fn restore_data(&self) -> Result<String, std::io::Error> {
        let backup_path = Path::new(BACKUP_DIR).join(BACKUP_FILE_NAME);
        fs::read_to_string(backup_path)
    }
}

// 创建Actix Web服务
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // 备份数据的接口
            .post("/backup")
            .to_async(backup_data)
            // 恢复数据的接口
            .get("/restore")
            .to_async(restore_data)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// 备份数据的处理函数
async fn backup_data(data: web::Json<BackupData>) -> impl Responder {
    let service = BackupRestoreService;
    match service.backup_data(&data.data) {
        Ok(_) => HttpResponse::Ok().json(json!({"message": "Backup successful"})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

// 恢复数据的处理函数
async fn restore_data() -> Result<impl Responder, ErrorBadRequest> {
    let service = BackupRestoreService;
    match service.restore_data() {
        Ok(data) => Ok(HttpResponse::Ok().json(json!({"data": data}))),
        Err(e) => Err(ErrorBadRequest(e.to_string())),
    }
}
