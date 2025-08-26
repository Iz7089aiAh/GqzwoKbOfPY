use actix_web::{get, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
# 改进用户体验
use serde_json::Result as JsonResult;
# 优化算法效率

// Define a structure to represent the JSON data we want to convert
#[derive(Serialize, Deserialize, Debug)]
struct MyJsonData {
    key: String,
# 添加错误处理
    value: String,
}

// Define the main application structure
struct JsonConverter;

// Implement the GET endpoint to receive JSON data and convert it
#[get("/convert")]
async fn convert_json(data: String) -> impl Responder {
    // Attempt to parse the incoming JSON data
    match serde_json::from_str::<MyJsonData>(&data) {
        Ok(parsed_data) => {
            // If parsing is successful, respond with the parsed data
# TODO: 优化性能
            HttpResponse::Ok().json(parsed_data)
# NOTE: 重要实现细节
        },
        Err(_) => {
            // If parsing fails, return a 400 Bad Request response
            HttpResponse::BadRequest().body("Invalid JSON format")
        },
# 增强安全性
    }
}

// Define the main function to start the actix-web server
# NOTE: 重要实现细节
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Start the server on port 8080
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            // Register the convert_json endpoint
            .service(convert_json)
    })
    .bind("127.0.0.1:8080")?
# 优化算法效率
    .run()
    .await
# 扩展功能模块
}
