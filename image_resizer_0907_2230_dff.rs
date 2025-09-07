// image_resizer.rs

// 引入所需的库
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use image::imageops::resize;
use image::{open, ImageOutputFormat};
use std::path::Path;
use std::fs;

// 定义错误类型
#[derive(Debug)]
enum ImageResizerError {
    OpenError(std::io::Error),
    ResizeError(image::ImageError),
    SaveError(std::io::Error),
}

impl From<std::io::Error> for ImageResizerError {
    fn from(err: std::io::Error) -> Self {
        ImageResizerError::OpenError(err)
    }
}

impl From<image::ImageError> for ImageResizerError {
    fn from(err: image::ImageError) -> Self {
        ImageResizerError::ResizeError(err)
    }
}

// 定义一个处理图片尺寸调整的函数
async fn resize_image(path: web::Path<String>, new_width: web::Query<u32>, new_height: web::Query<u32>) -> impl Responder {
    // 尝试打开图片文件
    let img = match open(&path.into_inner()) {
        Ok(img) => img,
        Err(_) => return HttpResponse::InternalServerError().json(ImageResizerError::OpenError(std::io::Error::new(std::io::ErrorKind::NotFound, "Image not found"))),
    };

    // 调整图片尺寸
    let resized_img = match resize(&img, new_width.into_inner(), new_height.into_inner(), resize::Type::Nearest) {
        Ok(resized) => resized,
        Err(_) => return HttpResponse::InternalServerError().json(ImageResizerError::ResizeError(image::ImageError::UnsupportedOperation)),
    };

    // 保存调整后的图片文件
    let mut output_path = Path::new(&path.into_inner()).to_path_buf();
    output_path.set_extension("png"); // 可以根据需要更改输出格式
    match resized_img.save(output_path) {
        Ok(_) => HttpResponse::Ok().json("Image resized successfully"),
        Err(_) => HttpResponse::InternalServerError().json(ImageResizerError::SaveError(std::io::Error::new(std::io::ErrorKind::Other, "Failed to save resized image"))),
    }
}

// 主函数
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 启动HTTP服务器
    HttpServer::new(|| {
        App::new()
            .route("/resize", web::post().to(resize_image))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
