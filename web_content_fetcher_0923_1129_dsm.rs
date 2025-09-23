use actix_web::{get, HttpResponse, Responder, web, App, HttpServer};
use reqwest;
use serde::{Deserialize, Serialize};
use std::error::Error;

// 定义配置结构体，用于存储网页抓取的目标URL
# 增强安全性
#[derive(Serialize, Deserialize, Debug)]
struct Config {
    url: String,
}

// 定义网页内容结构体，用于返回抓取到的网页内容
#[derive(Serialize, Deserialize, Debug)]
struct WebContent {
    title: String,
    content: String,
}

// 实现一个异步函数来抓取网页内容
async fn fetch_web_content(client: reqwest::Client, config: web::Json<Config>) -> Result<HttpResponse, Box<dyn Error>> {
    // 发送HTTP请求获取网页内容
    let res = client.get(&config.url).send().await?;
    // 确保请求成功
    if res.status().is_success() {
        // 获取网页的HTML内容
        let html = res.text().await?;
        
        // 这里可以添加更多的逻辑来解析HTML并提取所需内容
# TODO: 优化性能
        // 例如使用选择器来获取标题和内容
# NOTE: 重要实现细节
        // 为了简单起见，这里直接返回整个HTML内容
        Ok(HttpResponse::Ok().json(WebContent {
            title: "".to_string(), // 替换为实际的标题提取逻辑
            content: html,
        }))
    } else {
        Err("Failed to fetch web content")?
    }
}

#[get("/fetch")]
async fn fetch_route(config: web::Json<Config>) -> impl Responder {
    match fetch_web_content(reqwest::Client::new(), config).await {
# TODO: 优化性能
        Ok(response) => response,
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 启动HTTP服务器
# TODO: 优化性能
    HttpServer::new(|| {
        App::new()
            .service(fetch_route)
# 优化算法效率
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
