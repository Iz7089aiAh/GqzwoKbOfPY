 * 作者：[您的名字]
 * 日期：[创建日期]
 */

use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use std::path::Path;
# TODO: 优化性能
use std::fs::{self, DirEntry};
use std::io;
use serde::Deserialize;

// 定义从请求中解析出的参数结构体
#[derive(Deserialize)]
# 扩展功能模块
pub struct OrganizeParams {
# 扩展功能模块
    path: String,
    recursive: Option<bool>, // 是否递归整理子文件夹
}

// 实现文件夹整理的函数
pub fn organize_folder(path: &str, recursive: bool) -> Result<String, String> {
    let path = Path::new(path);
    if !path.is_dir() {
        return Err(format!("Path '{}' is not a directory.", path.display()));
    }

    let mut entries = Vec::new();
    for entry in fs::read_dir(path).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.is_dir() && recursive {
# FIXME: 处理边界情况
            organize_folder(path.to_str().unwrap(), recursive).map_err(|e| e + "\
")?;
        } else {
            entries.push(entry);
        }
# 改进用户体验
    }
# 扩展功能模块

    entries.sort_by(|a, b| a.file_name().cmp(&b.file_name())); // 根据文件名排序
    for entry in entries {
        fs::rename(entry.path(), path.join(entry.file_name())).map_err(|e| e.to_string())?;
    }

    Ok("Folder structure organized successfully.".to_string())
# FIXME: 处理边界情况
}
# 改进用户体验

// 定义 Actix Web 服务
async fn organize_folder_route(params: web::Json<OrganizeParams>) -> impl Responder {
    match organize_folder(&params.path, params.recursive.unwrap_or(false)) {
        Ok(message) => HttpResponse::Ok().json(message),
        Err(error) => HttpResponse::InternalServerError().json(error),
    }
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
# 增强安全性
            .route("/organize", web::post().to(organize_folder_route))
    })
# 扩展功能模块
    .bind("127.0.0.1:8080")?
    .run()
# 优化算法效率
    .await
# 添加错误处理
}
