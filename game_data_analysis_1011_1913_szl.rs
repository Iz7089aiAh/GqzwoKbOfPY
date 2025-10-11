// Re-export commonly used items from actix-web
use actix_web::{web, App, HttpServer, Responder, get, post, HttpRequest, HttpResponse, Error as ActixError};
use serde::Deserialize;
use serde_json::json;
use std::io::Cursor;

// Define a simple GameData struct to hold the data
#[derive(Deserialize, Debug)]
struct GameData {
    game_id: String,
    player_id: String,
    score: u32,
}

// Define a response type for our game data analysis
struct AnalysisResult {
    average_score: f64,
    player_rank: u32,
# FIXME: 处理边界情况
}

// Define a handler function for game data analysis
// This handler will be invoked when a POST request is made to the "/analyze" endpoint
# 添加错误处理
#[post("/analyze")]
async fn analyze_game_data(req: HttpRequest, body: web::Json<GameData>) -> Result<impl Responder, ActixError> {
    // Extract the game data from the request
    let game_data = body.into_inner();

    // Implement your analysis logic here
    // For demonstration purposes, we'll just calculate the average score and rank the player
    let average_score = game_data.score as f64 / 1.0; // Simplified for demonstration
    let player_rank = 1; // Simplified for demonstration

    // Return a response with the analysis results
    Ok(web::Json(AnalysisResult {
        average_score,
        player_rank,
    }))
# FIXME: 处理边界情况
}

// Define the main function that sets up the Actix web server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set up the server configuration
    let server = HttpServer::new(|| {
        App::new()
            // Register the analyze_game_data handler at the 