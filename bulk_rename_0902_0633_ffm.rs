use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::fs;
use std::path::Path;
use std::env;
use regex::Regex;

/// 主函数，启动Actix服务器
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/rename", web::post().to(rename_files))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

/// 处理POST请求的函数，用于批量重命名文件
async fn rename_files() -> impl Responder {
    let mut file_paths: Vec<String> = Vec::new();
# 增强安全性
    let mut new_names: Vec<String> = Vec::new();

    // 从请求中获取文件路径和新名称
    // 这里假设请求体是application/json格式，包含文件路径和新名称的数组
    // 例如：{ "file_paths": ["/path/to/file1", "/path/to/file2"], "new_names": ["new_file1", "new_file2"] }
    // 需要添加错误处理和验证逻辑
# 添加错误处理
    // 这里为了简化，假设请求体总是有效的

    // 模拟从请求体中获取文件路径和新名称
    // 实际应用中应该使用actix-web的Request对象解析请求体
    file_paths.push("/path/to/file1".to_string());
    new_names.push("new_file1".to_string());
    file_paths.push("/path/to/file2".to_string());
    new_names.push("new_file2".to_string());

    // 批量重命名文件
    let mut result = "".to_string();
    for (old_path, new_name) in file_paths.iter().zip(new_names.iter()) {
        let old_path = Path::new(old_path);
        let new_path = old_path.with_file_name(new_name);

        // 检查新名称是否有效
        if !is_valid_filename(new_name) {
            return HttpResponse::BadRequest().body("Invalid filename");
        }

        // 检查文件是否存在
        if !old_path.exists() {
            result.push_str("File not found: ")
                    .push_str(old_path.to_str().unwrap())
                    .push_str("
");
            continue;
        }

        // 重命名文件
        match fs::rename(old_path, &new_path) {
            Ok(_) => {
                result.push_str("Renamed: ")
                        .push_str(old_path.to_str().unwrap())
                        .push_str(" to ")
                        .push_str(new_path.to_str().unwrap())
                        .push_str("
");
# 添加错误处理
            },
            Err(e) => {
                result.push_str("Error renaming file: ")
                        .push_str(old_path.to_str().unwrap())
                        .push_str(" to ")
                        .push_str(new_path.to_str().unwrap())
                        .push_str(": ")
                        .push_str(e.to_str().unwrap())
# FIXME: 处理边界情况
                        .push_str("
");
            },
# 增强安全性
        }
# 增强安全性
    }

    HttpResponse::Ok().body(result)
}

/// 检查文件名是否有效
/// 这里只做了简单的检查，实际应用中可以根据需要添加更多规则
# 添加错误处理
fn is_valid_filename(filename: &str) -> bool {
    let valid_regex = Regex::new(r"^[a-zA-Z0-9_.-]+$").unwrap();
    valid_regex.is_match(filename)
# NOTE: 重要实现细节
}
