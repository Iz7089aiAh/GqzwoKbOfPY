use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::fs;
use std::path::Path;
use std::io;
use regex::Regex;

// 定义一个结构体来处理批量重命名请求
struct BatchRename {
    directory: String,
    prefix: String,
    regex: Regex,
}

// 为BatchRename结构体实现方法
impl BatchRename {
    // 构造函数
    fn new(directory: &str, prefix: &str, regex: Regex) -> BatchRename {
        BatchRename {
            directory: directory.to_string(),
            prefix: prefix.to_string(),
            regex,
        }
    }

    // 执行批量重命名操作
    fn execute(&self) -> io::Result<()> {
        // 读取目录内容
        let entries = fs::read_dir(&self.directory)?;
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                // 使用正则表达式匹配文件名
                let new_name = self.regex.replace_all(&path.file_name().unwrap().to_string_lossy(), &self.prefix).into_owned();
                let new_path = [Path::new(&self.directory), Path::new(&new_name)].iter().collect();
                // 重命名文件
                fs::rename(path, new_path)?;
            }
        }
        Ok(())
    }
}

// 实现actix_web的Responder trait，用于返回JSON响应
impl Responder for BatchRename {
    type Error = io::Error;
    fn respond_to(self, _: &HttpRequest) -> Result<HttpResponse, Self::Error> {
        match self.execute() {
            Ok(_) => HttpResponse::Ok().json({"message": "Batch rename completed successfully"}),
            Err(e) => HttpResponse::InternalServerError().json({"error": format!("Error during batch rename: {}", e)}),
        }
    }
}

// 设置actix-web服务器
#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/rename", web::post().to(batch_rename_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// 处理POST请求的函数
async fn batch_rename_handler() -> impl Responder {
    // 这里需要添加逻辑来获取请求参数，如directory, prefix, 和正则表达式
    // 例如，从JSON请求体中解析参数
    // 这里只是一个示例，不包含实际的请求解析逻辑
    let directory = "./files".to_string();
    let prefix = "new_".to_string();
    let regex_pattern = "^old_".to_string();
    let regex = Regex::new(&regex_pattern).unwrap();
    let renamer = BatchRename::new(&directory, &prefix, regex);
    renamer
}
