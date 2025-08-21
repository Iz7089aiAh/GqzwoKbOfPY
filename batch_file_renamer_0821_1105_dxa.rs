use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::path::Path;
use std::fs;
use std::io::{self, Error};
use regex::Regex;

/// 批量文件重命名工具结构体
struct BatchRenamer {
    directory: String,
    old_pattern: Regex,
    new_pattern: String,
}

/// 实现BatchRenamer的方法
impl BatchRenamer {
    /// 构造函数
    fn new(directory: String, old_pattern: Regex, new_pattern: String) -> BatchRenamer {
        BatchRenamer {
            directory,
            old_pattern,
            new_pattern,
        }
    }

    /// 执行批量重命名
    fn rename_files(&self) -> io::Result<()> {
        for entry in fs::read_dir(&self.directory)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let new_file_name = self.old_pattern.replace_all(file_name, &self.new_pattern);
                fs::rename(path, path.with_file_name(new_file_name)?)?;
            }
        }
        Ok(())
    }
}

/// 处理POST请求的函数
async fn rename_files_handler(req: HttpRequest) -> impl Responder {
    let body: String = req.body().await?;
    let params: web::Json<web::JsonValue> = web::Json::from_str(&body).unwrap();
    let directory = params.get("directory").unwrap().as_str().unwrap().to_string();
    let old_pattern = params.get("old_pattern").unwrap().as_str().unwrap().to_string();
    let new_pattern = params.get("new_pattern").unwrap().as_str().unwrap().to_string();
    let regex = Regex::new(&old_pattern).unwrap();

    match BatchRenamer::new(directory, regex, new_pattern).rename_files() {
        Ok(_) => HttpResponse::Ok().json("Files renamed successfully"),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}

/// 程序入口函数
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                rename_files_handler,
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
