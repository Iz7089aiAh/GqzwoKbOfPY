// resize_images.rs
// 使用Actix框架实现图片尺寸批量调整器
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use image::DynamicImage;
use std::path::Path;
use std::fs;
use std::io::Result;
use imageproc::rect::Rect;
use serde::Deserialize;
use actix_web::get;
use image::{ImageError};
use actix_files as fs;
use actix_web::http::StatusCode;

// 定义请求结构体
#[derive(Deserialize)]
pub struct ResizeConfig {
    width: u32,
    height: u32,
    directory: String,
}

// 定义错误结构体
#[derive(Debug)]
pub enum ImageResizerError {
    ImageError(ImageError),
    IoError(std::io::Error),
    NotFound,
}

impl From<ImageError> for ImageResizerError {
    fn from(err: ImageError) -> Self {
        ImageResizerError::ImageError(err)
    }
}

impl From<std::io::Error> for ImageResizerError {
    fn from(err: std::io::Error) -> Self {
        ImageResizerError::IoError(err)
    }
}

// 图片调整尺寸函数
fn resize_images(config: ResizeConfig) -> Result<Vec<String>, ImageResizerError> {
    let mut resized_images = Vec::new();

    for entry in fs::read_dir(Path::new(&config.directory))? {
        let entry = entry?;
        let path = entry.path();

        // 只处理图片文件
        if path.is_file() && path.extension().and_then(std::ffi::OsStr::to_str).map_or(false, |ext| ext.ends_with(".png") || ext.ends_with(".jpg") || ext.ends_with(".jpeg")) {
            let img = DynamicImage::open(&path)?;
            let resized_img = img.resize(config.width, config.height, image::imageops::FilterType::Nearest);
            let new_path = path.with_extension("resized");
            resized_img.save(&new_path).map_err(ImageResizerError::from)?;
            resized_images.push(new_path.to_str().unwrap().to_string());
        }
    }

    Ok(resized_images)
}

// 创建HTTP服务端点
#[get("/resize")]
async fn resize_endpoint(config: web::Query<ResizeConfig>) -> impl Responder {
    match resize_images(config.into_inner()) {
        Ok(resized_images) => {
            HttpResponse::Ok().json(resized_images)
        },
        Err(e) => match e {
            ImageResizerError::NotFound => HttpResponse::NotFound().body("Directory not found"),
            ImageResizerError::IoError(e) => HttpResponse::InternalServerError().body(format!("IO error: {}", e)),
            ImageResizerError::ImageError(e) => HttpResponse::InternalServerError().body(format!("Image error: {}", e)),
        },
    }
}

// 主函数
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 启动HTTP服务器
    HttpServer::new(|| {
        App::new()
            .service(resize_endpoint)
            // 允许静态文件服务，用于返回调整后的图片
            .service(fs::Files::new("/", "."))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
