use actix_web::{get, HttpResponse, Responder, web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process::Command;
use anyhow::Result;

// Define the structure for the ETL configuration
#[derive(Serialize, Deserialize)]
struct EtlConfig {
    extract_path: String,
    transform_command: String,
    load_path: String,
}

// Define a struct to hold the ETL data
#[derive(Serialize, Deserialize)]
struct EtlData {
    data: String,
}

#[get("/etl")]
async fn etl() -> impl Responder {
    let etl_config = EtlConfig {
        extract_path: "./data/source.txt".to_string(),
        transform_command: "cat".to_string(),
        load_path: "./data/target.txt".to_string(),
    };

    match run_etl_pipeline(etl_config) {
        Ok(result) => HttpResponse::Ok().json(json!({
            "status": "success",
            "result": result.data,
        })),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

// Function to run the ETL pipeline
fn run_etl_pipeline(config: EtlConfig) -> Result<EtlData> {
    // Extract phase: Read data from the file
    let extract_result = extract_data(&config.extract_path)?;

    // Transform phase: Run a command on the extracted data
    let transform_result = transform_data(&config.transform_command, &extract_result)?;

    // Load phase: Write the transformed data to a file
    let load_result = load_data(&config.load_path, &transform_result)?;

    Ok(load_result)
}

// Extract data from a file
fn extract_data(path: &str) -> Result<String> {
    let file = File::open(path).map_err(|e| anyhow::anyhow!("Failed to open file: {}", e))?;
    let reader = BufReader::new(file);
    let data: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    Ok(data.join("\
"))
}

// Transform data by running a command
fn transform_data(command: &str, data: &str) -> Result<String> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .map_err(|e| anyhow::anyhow!("Failed to run command: {}", e))?;
    if !output.status.success() {
        return Err(anyhow::anyhow!("Command failed"));
    }
    Ok(String::from_utf8(output.stdout).map_err(|e| anyhow::anyhow!("Failed to parse command output: {}", e))?)
}

// Load data into a file
fn load_data(path: &str, data: &str) -> Result<EtlData> {
    let mut file = File::create(path).map_err(|e| anyhow::anyhow!("Failed to create file: {}", e))?;
    file.write_all(data.as_bytes()).map_err(|e| anyhow::anyhow!("Failed to write to file: {}", e))?;
    Ok(EtlData { data: data.to_string() })
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(etl)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
