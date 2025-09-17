use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

// Define a struct to hold chart configurations
#[derive(Serialize, Deserialize, Debug, Clone)]
struct ChartConfig {
    title: String,
    data: HashMap<String, Vec<f64>>,
}

// Define a struct to hold chart data points
#[derive(Serialize, Deserialize, Debug, Clone)]
struct ChartDataPoint {
    x: f64,
    y: f64,
}

// Define a struct to hold the response for the generated chart
#[derive(Serialize, Deserialize, Debug)]
struct ChartResponse {
    chart_url: String,
    configuration: ChartConfig,
}

// Define the error enum for different error cases
#[derive(Debug)]
enum ChartError {
    BadRequest(String),
    InternalError(String),
}

// Implement Responder trait for ChartError to convert it to a proper HttpResponse
impl Responder for ChartError {
    fn respond_to(self, _: &web::HttpRequest) -> HttpResponse {
        match self {
            ChartError::BadRequest(msg) => HttpResponse::BadRequest().json(json!({ "error": msg })),
            ChartError::InternalError(msg) => HttpResponse::InternalServerError().json(json!({ "error": msg })),
        }
    }
}

// Define the handler function for generating charts
async fn generate_chart(config: web::Json<ChartConfig>) -> Result<impl Responder, ChartError> {
    // Check if the configuration is valid
    if config.title.is_empty() || config.data.is_empty() {
        return Err(ChartError::BadRequest("Invalid chart configuration".to_string()));
    }

    // Simulate chart generation and return a URL and the configuration
    // In a real scenario, this would involve more complex logic and possibly calls to an external charting service
    let chart_url = format!("https://chart-service.com/chart?config={}", serde_json::to_string(&config)?);
    let response = ChartResponse {
        chart_url,
        configuration: config.into_inner(),
    };

    Ok(HttpResponse::Ok().json(response))
}

// Define the main function to start the server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/generate_chart", web::post().to(generate_chart))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// Add necessary documentation for the code
/// This is a simple interactive chart generator using Actix framework.
/// It accepts chart configurations via JSON and returns a simulated chart URL and configuration.
/// The chart generation is simulated for the sake of this example and would need to be replaced with
/// actual chart generation logic for a real-world application.
