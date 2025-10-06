// 文件夹结构整理器
// 使用 Rust 和 Actix 框架实现
#[macro_use]
extern crate actix_web;

use actix_web::{
    HttpResponse,
    HttpServer,
    web,
    get,
    App,
    Responder,
};
use std::fs::{self, DirEntry};
use std::path::Path;
use std::io;

/// 整理文件夹结构的功能
/// 检查给定路径的文件夹，并对其进行整理
async fn organize_folder_structure(path: web::Path<String>) -> impl Responder {
    match organize_folder_structure_impl(path.into_inner()) {
        Ok(_) => HttpResponse::Ok().json("Folder structure organized successfully."),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

/// 实现整理文件夹结构的逻辑
fn organize_folder_structure_impl(path: String) -> io::Result<()> {
    let path = Path::new(&path);
    if !path.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Path not found"));
    }
    
    if !path.is_dir() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Provided path is not a directory"));
    }
    
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            // 这里可以添加更多的整理逻辑，例如按文件类型排序等
            continue;
        }
        // 这里可以添加更多的文件处理逻辑
    }
    
    Ok(())
}

/// Actix 服务的主入口点
#[actix_web::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/organize/{path}").to(organize_folder_structure))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
