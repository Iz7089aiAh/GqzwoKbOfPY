use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use serde_json::json;
use anyhow::Result;

// 定义文件属性结构
#[derive(Debug, Serialize, Deserialize)]
struct FileInfo {
    path: String,
    is_dir: bool,
    size: u64,
}

// 实现 FolderOrganizer 结构体
struct FolderOrganizer;

// 定义 FolderOrganizer 行为
impl FolderOrganizer {
    // 整理文件夹内容
    fn organize_folder(path: &Path) -> Result<Vec<FileInfo>> {
        let mut files_info = Vec::new();

        if path.is_dir() {
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let path = entry.path();
                let is_dir = path.is_dir();
                let size = if is_dir { 0 } else { path.metadata()?.len() };
                files_info.push(FileInfo {
                    path: path.to_str().unwrap_or_default().to_string(),
                    is_dir,
                    size,
                });
            }
        }

        Ok(files_info)
    }
}

// 实现 API 处理逻辑
async fn organize_folder_api(path_param: web::Path<String>) -> impl Responder {
    let path = Path::new(&path_param);
    match FolderOrganizer::organize_folder(path) {
        Ok(files_info) => HttpResponse::Ok().json(json!({
            "files": files_info,
        })),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/organize/{path}", web::get().to(organize_folder_api))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
