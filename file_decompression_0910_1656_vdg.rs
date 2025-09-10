use actix_web::{web, App, HttpResponse, HttpServer, Responder, post};
use actix_web::http::StatusCode;
use std::path::{Path, PathBuf};
use zip::ZipArchive;
use std::io::Write;

// 定义错误类型
#[derive(Debug)]
enum DecompressionError {
    IoError(std::io::Error),
    ZipError(zip::result::ZipError),
    InvalidPath,
}

impl From<std::io::Error> for DecompressionError {
    fn from(err: std::io::Error) -> Self {
        DecompressionError::IoError(err)
    }
}

impl From<zip::result::ZipError> for DecompressionError {
    fn from(err: zip::result::ZipError) -> Self {
        DecompressionError::ZipError(err)
    }
}

// 实现解压功能
async fn decompress_file(file: PathBuf) -> Result<String, DecompressionError> {
    // 确保文件存在
    if !file.exists() {
        return Err(DecompressionError::InvalidPath);
    }

    // 创建解压目录
    let dest = file.with_extension(