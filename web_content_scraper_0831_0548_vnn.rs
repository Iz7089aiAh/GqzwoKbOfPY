use actix_web::{
    web,
    get,
    HttpRequest,
    HttpResponse,
    Error,
# 优化算法效率
    RespondeError,
};
use reqwest;
use std::error::Error as StdError;
use serde::Deserialize;
# 添加错误处理
use serde_json::json;
use std::fmt;

// Define a structure to deserialize the request parameters.
#[derive(Deserialize)]
struct ScrapeRequest {
    url: String,
}

#[derive(Debug)]
struct ScrapeError {
    message: String,
}

impl fmt::Display for ScrapeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
# 添加错误处理
    }
# 改进用户体验
}

impl RespondeError for ScrapeError {
    fn error_response(&self) -> HttpResponse {
# TODO: 优化性能
        HttpResponse::InternalServerError().json(json!({
            "error": self.message,
        }))
    }
}

impl From<reqwest::Error> for ScrapeError {
# 扩展功能模块
    fn from(err: reqwest::Error) -> ScrapeError {
        ScrapeError {
            message: format!("Failed to fetch page: {}", err),
        }
    }
}

impl From<std::io::Error> for ScrapeError {
# TODO: 优化性能
    fn from(err: std::io::Error) -> ScrapeError {
        ScrapeError {
            message: format!("IO error occurred: {}", err),
        }
    }
# 改进用户体验
}

// Define the Actix web service handler for scraping web content.
#[get("/scrape")]
# 添加错误处理
async fn scrape_content(req: HttpRequest, body: web::Json<ScrapeRequest>) -> Result<HttpResponse, Error> {
    let url = &body.url;
    
    // Error handling for invalid URLs.
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err(ScrapeError {
            message: "Invalid URL, must start with http:// or https://".to_string(),
        }.into());
# 优化算法效率
    }
    
    // Fetch the content from the provided URL.
    let content = match reqwest::get(url).await {
        Ok(response) => match response.text().await {
            Ok(text) => text,
# NOTE: 重要实现细节
            Err(e) => return Err(ScrapeError::from(e).into()),
        },
        Err(e) => return Err(ScrapeError::from(e).into()),
# NOTE: 重要实现细节
    };
    
    // Return the fetched content as a JSON response.
    Ok(HttpResponse::Ok().json(json!({
        "url": url,
        "content": content,
    })))
}

// Define the main function to start the Actix server.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Start the Actix server and listen on port 8080.
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .route("/scrape", web::post().to(scrape_content))
    })
    .bind("127.0.0.1:8080")?
    .run()
# 添加错误处理
    .await
}

// Proper documentation and error handling are included.
# NOTE: 重要实现细节
// The scraper is designed to be easily maintainable and extensible.
