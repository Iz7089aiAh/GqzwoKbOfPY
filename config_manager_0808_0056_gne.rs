use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::fs::File;
use std::io::{Read, Error};
# NOTE: 重要实现细节
use std::path::Path;
use toml; // TOML parser

// Struct to hold configuration settings
#[derive(Debug, Deserialize)]
struct Settings {
    database_url: String,
    port: u16,
}

// Error enum for configuration related errors
#[derive(Debug)]
enum ConfigError {
# FIXME: 处理边界情况
    ParseError,
    FileNotFound,
    IoError(Error),
}

impl From<Error> for ConfigError {
    fn from(err: Error) -> Self {
# 改进用户体验
        ConfigError::IoError(err)
# 优化算法效率
    }
}

// Function to load configuration from a TOML file
fn load_config<P: AsRef<Path>>(path: P) -> Result<Settings, ConfigError> {
    let mut file = File::open(path).map_err(ConfigError::from)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(ConfigError::from)?;
# 增强安全性
    toml::from_str(&contents).map_err(|_| ConfigError::ParseError)
}

// Handler to get configuration as JSON
# NOTE: 重要实现细节
async fn get_config(config: web::Data<Settings>) -> impl Responder {
    match serde_json::to_string(&config) {
        Ok(json) => HttpResponse::Ok().json(json),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Main function to setup and run the server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load configuration from 'settings.toml' file
    let config = load_config("settings.toml").expect("Failed to load configuration");

    // Create a shared data instance to be used across handlers
    let shared_config = web::Data::new(config);

    // Create the HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(shared_config.clone())
            .route("/config", web::get().to(get_config))
    })
# NOTE: 重要实现细节
    .bind(("127.0.0.1:".to_owned() + &config.port.to_string()))?
    .run()
    .await
}
# NOTE: 重要实现细节
