use actix_web::{get, HttpResponse, HttpServer, Responder};
use image::{open, ImageOutputFormat, ImageResult};
use std::path::Path;
use std::fs;
use std::io::BufReader;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use actix_files as fs;
use actix_web::web::Data;
use imageproc::resize::resize;
use imageproc::color::Luma;
use imageproc::image::{Image, GenericImageView};
use anyhow::Result;
use glob::glob;
use std::sync::Mutex;
use std::sync::Arc;

// A struct to hold configurations like the target image size.
struct Config {
    width: u32,
    height: u32,
}

// A service to handle image resizing tasks.
struct ImageResizer;

impl ImageResizer {
    // Resizes images in a directory
    async fn resize_images(&self, config: Data<Arc<Mutex<Config>>>, path: String) -> Result<HttpResponse, actix_web::Error> {
        let resized_images = self.resize_images_in_directory(&config, &path).await?;
        Ok(HttpResponse::Ok().json(resized_images))
    }

    // Asynchronously resize images in a directory
    async fn resize_images_in_directory(&self, config: &Data<Arc<Mutex<Config>>>, path: &str) -> Result<Vec<String>, anyhow::Error> {
        let resized_images: Vec<String> = glob(path)?
            .filter_map(|path| {
                if let Ok(path) = path {
                    let image = open(&path)?;
                    let dimensions = image.dimensions();
                    let resized = resize(&image, config.lock().unwrap().width, config.lock().unwrap().height, Luma::default());
                    let output_path = format!("{}/resized_{}.{}", path.parent().unwrap().to_str().unwrap(), path.file_stem().unwrap().to_str().unwrap(), path.extension().unwrap().to_str().unwrap());
                    resized.save_with_format(&Path::new(&output_path), ImageOutputFormat::default())?;
                    Ok(output_path)
                } else {
                    Err(anyhow::anyhow!("Failed to open image"))
                }
            })
            .collect();
        Ok(resized_images)
    }
}

#[get("/resize")]
async fn resize(
    config: web::Data<Arc<Mutex<Config>>>,
    params: web::Query<web::Json<Config>>,
    req: HttpRequest,
) -> impl Responder {
    ImageResizer.resize_images(config, params.into_inner().into_inner().into_inner().width.to_string()).await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(resize)
            // Serve static files from `public` directory
            .service(fs::Files::new("/", ".\/public"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// Documentation for struct Config
/// Configuration for image resizing.
///
/// This struct holds the target width and height for images.
///
/// # Examples
///
/// ```rust
/// let config = Config { width: 800, height: 600 };
/// ```
struct Config {
    width: u32,
    height: u32,
}

// Documentation for struct ImageResizer
/// Service to handle image resizing tasks.
///
/// This struct provides functionality to resize images in a directory.
///
/// # Examples
///
/// ```rust
/// let image_resizer = ImageResizer;
/// let config = Config { width: 800, height: 600 };
/// let resized_images = image_resizer.resize_images(&config, "./images").await.unwrap();
/// ```
struct ImageResizer;

impl ImageResizer {
    // Resizes images in a directory
    /// Resizes images in a directory based on the provided configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration for image resizing.
    /// * `path` - Path to the directory containing images to be resized.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of resized image paths or an error if resizing fails.
    async fn resize_images(&self, config: Data<Arc<Mutex<Config>>>, path: String) -> Result<HttpResponse, actix_web::Error> {
        let resized_images = self.resize_images_in_directory(&config, &path).await?;
        Ok(HttpResponse::Ok().json(resized_images))
    }

    // Asynchronously resize images in a directory
    /// Asynchronously resize images in a directory.
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration for image resizing.
    /// * `path` - Path to the directory containing images to be resized.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of resized image paths or an error if resizing fails.
    async fn resize_images_in_directory(&self, config: &Data<Arc<Mutex<Config>>>, path: &str) -> Result<Vec<String>, anyhow::Error> {
        let resized_images: Vec<String> = glob(path)?
            .filter_map(|path| {
                if let Ok(path) = path {
                    let image = open(&path)?;
                    let dimensions = image.dimensions();
                    let resized = resize(&image, config.lock().unwrap().width, config.lock().unwrap().height, Luma::default());
                    let output_path = format!("{}/resized_{}.{}", path.parent().unwrap().to_str().unwrap(), path.file_stem().unwrap().to_str().unwrap(), path.extension().unwrap().to_str().unwrap());
                    resized.save_with_format(&Path::new(&output_path), ImageOutputFormat::default())?;
                    Ok(output_path)
                } else {
                    Err(anyhow::anyhow!("Failed to open image"))
                }
            })
            .collect();
        Ok(resized_images)
    }
}

#[get("/resize")]
/// Handles requests to resize images.
///
/// This handler takes a JSON configuration with width and height
/// and a path to the directory containing images to be resized.
async fn resize(
    config: web::Data<Arc<Mutex<Config>>>,
    params: web::Query<web::Json<Config>>,
    req: HttpRequest,
) -> impl Responder {
    ImageResizer.resize_images(config, params.into_inner().into_inner().into_inner().width.to_string()).await
}

#[actix_web::main]
/// Main entry point for the application.
///
/// Sets up the HTTP server and routes.
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(resize)
            // Serve static files from `public` directory
            .service(fs::Files::new("/", "./public"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
