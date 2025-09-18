use actix_web::{get, HttpResponse, Responder, web};
use serde::Deserialize;
use serde_json::Value;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::sync::Mutex;
use lazy_static::lazy_static;
use once_cell::sync::Lazy;

// Define a lazy static variable to hold the configuration data.
// This ensures that the configuration is loaded only once and is thread-safe.
lazy_static! {
    static ref CONFIG: Mutex<Value> = Mutex::new(Value::Null);
}

// Define a struct to hold the configuration data, if using a more complex data structure.
#[derive(Deserialize)]
pub struct Config {
    // Define the structure of the configuration here.
    // Example: pub field: String,
}

#[get("/config/reload")]
async fn reload_config() -> impl Responder {
    // Reload the configuration file.
    let mut config = CONFIG.lock().unwrap();
    *config = load_config("config.json").expect("Failed to load configuration file.");
    HttpResponse::Ok().body("Configuration reloaded.")
}

#[get("/config")]
async fn get_config() -> impl Responder {
    // Return the current configuration.
    let config = CONFIG.lock().unwrap();
    HttpResponse::Ok().json(config.clone())
}

// Function to load the configuration file.
fn load_config<P: AsRef<Path>>(path: P) -> io::Result<Value> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(serde_json::from_str(&contents)?)
}

// Main function to start the server.
#[actix_web::main]
async fn main() -> io::Result<()> {
    // Load the configuration file on startup.
    let mut config = CONFIG.lock().unwrap();
    *config = load_config("config.json").expect("Failed to load configuration file.");

    // Start the Actix server.
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .service(reload_config)
            .service(get_config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// Unit tests for the configuration manager.
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web};

    #[actix_rt::test]
    async fn test_config_reload() {
        let app = test::init_service(App::new().service(reload_config)).await;
        let req = test::TestRequest::with_uri("/config/reload").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn test_get_config() {
        let app = test::init_service(App::new().service(get_config)).await;
        let req = test::TestRequest::with_uri("/config").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}
