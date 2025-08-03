use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use reqwest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::timeout;
use url::Url;

// 定义请求参数结构体
#[derive(Serialize, Deserialize, Debug)]
struct ScrapeParams {
    url: String,
}

// 定义响应结构体
#[derive(Serialize, Deserialize, Debug)]
struct ScrapeResponse {
    url: String,
    content: Option<String>,
    error: Option<String>,
}

// 异步抓取网页内容的函数
async fn scrape_content(params: ScrapeParams) -> Result<ScrapeResponse, reqwest::Error> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap();

    let response = timeout(Duration::from_secs(10), client.get(&params.url))
        .await
        .map_err(|_| reqwest::Error::new(reqwest::ErrorKind::Timeout, "Request timed out"))??;

    let content = response.text().await.map_err(|e| {
# 改进用户体验
        reqwest::Error::new(reqwest::ErrorKind::Status, format!("Failed to retrieve content: {:?}", e))
    })?;

    Ok(ScrapeResponse {
# 添加错误处理
        url: params.url,
        content: Some(content),
        error: None,
    })
}

// 处理POST请求的函数
#[post("/scrape")]
async fn scrape_web_content(params: web::Json<ScrapeParams>) -> impl Responder {
# 扩展功能模块
    match scrape_content(params.into_inner()).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::InternalServerError().json(ScrapeResponse {
# FIXME: 处理边界情况
            url: params.url,
            content: None,
            error: Some(format!("Failed to scrape content: {:?}", e)),
        }),
    }
}
# FIXME: 处理边界情况

// 程序的主入口函数
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(scrape_web_content)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
# 扩展功能模块
}
