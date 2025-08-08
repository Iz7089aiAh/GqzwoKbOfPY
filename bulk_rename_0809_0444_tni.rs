use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::fs;
use std::path::Path;
use regex::Regex;

// 定义批量重命名请求的结构体
struct BulkRenameRequest {
    path: String,
    pattern: String,
    replace: String,
}

// 实现批量重命名逻辑
async fn bulk_rename(req: web::Json<BulkRenameRequest>) -> impl Responder {
    let path = Path::new(&req.path);
    let pattern = Regex::new(&req.pattern).expect("Invalid regex pattern");
    let replace = req.replace;
    
    if !path.is_dir() {
        return HttpResponse::BadRequest().body("Provided path is not a directory");
    }
    
    let mut success = 0;
    let mut failures = 0;
    
    for entry in fs::read_dir(path).expect("Failed to read directory") {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();
        let file_name = path.file_name().expect("Failed to get file name").to_str().expect("Invalid file name");
        
        let new_name = pattern.replace_all(file_name, &replace).into_owned();
        let new_path = path.with_file_name(new_name.as_str().expect("Invalid new file name"));
        
        if path.file_name() != new_path.file_name() { // Avoid renaming if the file name remains the same
            if fs::rename(&path, &new_path).is_err() {
                eprintln!("Failed to rename file: {}", path.display());
                failures += 1;
            } else {
                success += 1;
            }
        }
    }
    
    HttpResponse::Ok().body(format!("Renaming complete. Success: {}, Failures: {}", success, failures))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/bulk_rename").route(web::post().to(bulk_rename)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
