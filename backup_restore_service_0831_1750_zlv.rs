// backup_restore_service.rs
// 该服务提供数据备份和恢复功能
use actix_web::{get, post, web, HttpResponse, Responder};
use actix_web::dev::ServiceRequest;
use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::error::Error;

// 定义备份和恢复服务结构
struct BackupRestoreService;

// 实现备份文件的方法
#[get("/backup")]
async fn backup(req: ServiceRequest) -> impl Responder {
    let path = req.match_info().get("path").unwrap();
    let file_path = format!("{}/{}", "./data", path);
    let file = File::create(&file_path).expect("创建文件失败");

    let data = "备份数据";
    file.write_all(data.as_bytes()).expect("写入文件失败");

    HttpResponse::Ok().body("备份成功")
}

// 实现恢复文件的方法
#[post("/restore")]
async fn restore(req: ServiceRequest) -> impl Responder {
    let path = req.match_info().get("path").unwrap();
    let file_path = format!("{}/{}", "./data", path);
    let file = File::open(&file_path).expect("打开文件失败");
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).expect("读取文件失败");

    HttpResponse::Ok().body(contents)
}

// main函数，启动actix web服务器
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .service(backup)
            .service(restore)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
