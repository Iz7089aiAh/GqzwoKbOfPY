use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use flate2::read::GzDecoder;
use std::fs::File;
use std::io::{self, Read};
use tar::Archive;

// 主函数，启动Actix Web服务器
#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/decompress", web::post().to(decompress_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// 处理解压请求的函数
async fn decompress_handler() -> impl Responder {
    // 这里应该添加实际的文件上传处理逻辑，例如从请求中读取文件
    // 此处省略了具体的文件上传和读取代码
    // 假设我们已经有了一个.gz文件的路径
    let file_path = "path/to/your/file.gz";

    // 尝试解压文件
    match decompress_file(file_path) {
        Ok(_) => HttpResponse::Ok().body("Decompressed successfully"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

// 解压文件的函数
fn decompress_file(file_path: &str) -> Result<()> {
    let file = File::open(file_path).map_err(|e| anyhow::anyhow!("Failed to open file: {}", e))?;
    let mut archive = GzDecoder::new(file);
    let mut tar = Archive::new(archive);

    // 解压到当前目录
    let dest_path = std::path::Path::new(".");
    if let Err(e) = tar.unpack(dest_path) {
        return Err(anyhow::anyhow!("Failed to decompress file: {}", e));
    }

    Ok(())
}

/// 错误处理模块
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;
    use actix_web::http::StatusCode;

    #[actix_web::main]
    async fn main() {
        test::init_service(App::new().service(web::resource("/decompress").to(decompress_handler))).await;
    }

    #[test]
    async fn test_decompress_handler() {
        let app = test::init_service(App::new().service(web::resource("/decompress").to(decompress_handler))).await;
        let req = test::TestRequest::with_uri("/decompress").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status() == StatusCode::OK);
    }
}
