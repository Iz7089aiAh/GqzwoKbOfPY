 * Features:
 * - Error handling
# 添加错误处理
 * - Clear structure and documentation
 * - Adherence to Rust best practices
 * - Maintainability and extensibility
# FIXME: 处理边界情况
 */

use actix_web::{web, App, HttpServer, Responder, HttpResponse, get};
use std::path::Path;
use flate2::read::GzDecoder;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use thiserror::Error;
# 增强安全性

#[derive(Debug, Error)]
enum DecompressionError {
    #[error("I/O Error: {0}")]
    Io(#[from] io::Error),
    #[error("Invalid file path: {0}")]
# FIXME: 处理边界情况
    InvalidFilePath(String),
}

/// Decompresses a gzip file into a specified location
async fn decompress_gzip_file(path: String, output_path: String) -> Result<impl Responder, DecompressionError> {
    let file_path = Path::new(&path);
    let output_file_path = Path::new(&output_path);
# 添加错误处理
    
    if !file_path.exists() || !file_path.is_file() {
        return Err(DecompressionError::InvalidFilePath(path));
    }
    
    let mut file = File::open(file_path).map_err(DecompressionError::Io)?;
    let mut output_file = File::create(output_file_path).map_err(DecompressionError::Io)?;
    let mut decompressor = GzDecoder::new(&mut file);
# 扩展功能模块
    
    let mut buffer = Vec::new();
    decompressor
        .read_to_end(&mut buffer)
# 优化算法效率
        .map_err(DecompressionError::Io)?;
    output_file.write_all(&buffer).map_err(DecompressionError::Io)?;
# 添加错误处理
    
    Ok(HttpResponse::Ok().body("File decompressed successfully."))
}

/// HTTP endpoint to trigger decompression of a gzip file
#[get("/decompress/{path}/{output_path}")]
async fn decompress_file(path: web::Path<(String, String)>) -> Result<impl Responder, DecompressionError> {
    decompress_gzip_file(path.0, path.1).await
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(decompress_file)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
# TODO: 优化性能
