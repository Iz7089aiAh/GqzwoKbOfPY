use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

// Define a struct to hold ETL configurations
#[derive(Serialize, Deserialize)]
struct EtlConfig {
    source: String,
    transformation: String,
    destination: String,
}

// Define a struct to hold state of ETL process
struct EtlState {
    configs: Arc<Mutex<HashMap<String, EtlConfig>>>,
}

// Implement methods for EtlState
impl EtlState {
    fn new() -> Self {
        EtlState {
            configs: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    // Function to add ETL configuration
    fn add_etl_config(&self, config: EtlConfig) -> Result<(), &'static str> {
        let mut configs = self.configs.lock().map_err(|_| "Mutex lock failed")?;
        configs.insert(config.source.clone(), config);
        Ok(())
    }

    // Function to process ETL operations
    async fn process_etl(&self, source: &str) -> impl Responder {
        let configs = self.configs.lock().map_err(|_| HttpResponse::InternalServerError().finish())?;
        if let Some(config) = configs.get(source) {
            // Here you would add your actual ETL logic
            // For demonstration, we assume the transformation and destination are just a simple string replacement
            let transformed_data = format!("{}", &config.transformation); // Replace with actual transformation logic
            let final_data = format!("{}", &config.destination); // Replace with actual destination logic
            HttpResponse::Ok().body(final_data)
        } else {
            HttpResponse::NotFound().body("ETL configuration not found")
        }
    }
}

// Define the handler for adding ETL configurations
async fn add_config(state: web::Data<EtlState>, config: web::Json<EtlConfig>) -> impl Responder {
    match state.add_etl_config(config.into_inner()) {
        Ok(_) => HttpResponse::Created().body("ETL configuration added"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to add ETL configuration"),
    }
}

// Define the handler for processing ETL operations
async fn run_etl(state: web::Data<EtlState>, source: web::Path<String>) -> impl Responder {
    state.process_etl(&source.into_inner()).await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(EtlState::new());

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/add_config", web::post().to(add_config))
            .route("/run_etl/{source}", web::get().to(run_etl))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}