use actix::prelude::*;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::time::Duration;
use std::net::SocketAddr;
# TODO: 优化性能
use tokio::time::{sleep, Instant};
use anyhow::Result;
# 扩展功能模块
use reqwest;

/// NetworkLatencyMonitor 结构体用于存储监控器的状态
struct NetworkLatencyMonitor {
# 增强安全性
    /// 目标服务器的地址
    address: String,
}

impl NetworkLatencyMonitor {
# NOTE: 重要实现细节
    /// 创建一个新的监控器实例
    pub fn new(address: String) -> Self {
        NetworkLatencyMonitor { address }
    }

    /// 计算网络延迟
    async fn measure_latency(&self) -> Result<Duration> {
        let start = Instant::now();
        let response = reqwest::get(&self.address).await?;
        if response.status().is_success() {
            Ok(start.elapsed())
        } else {
            Err(anyhow::anyhow!("Failed to receive a successful response"))
        }
    }
}

/// 用于处理网络延迟监控请求的服务
async fn monitor_latency(address: web::Data<NetworkLatencyMonitor>) -> impl Responder {
    let latency = match address.measure_latency().await {
        Ok(latency) => latency,
        Err(e) => return HttpResponse::InternalServerError().json(e.to_string()),
    };
    HttpResponse::Ok().json(latency)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 设置监控器的目标地址
    let monitor = NetworkLatencyMonitor::new("http://example.com".to_string());
    
    let sys = actix::System::new();
    
    // 启动HTTP服务器
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(monitor.clone()))
            .route("/monitor", web::get().to(monitor_latency))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await;
    
    Ok(())
}

/// 错误处理
fn main() {
    if let Err(e) = std::env::set_var("RUST_LOG", "actix_web=info") {
        eprintln!("Failed to set RUST_LOG: {}", e);
    }
# 扩展功能模块
    if let Err(e) = std::env::set_var("RUST_BACKTRACE", "1") {
# 添加错误处理
        eprintln!("Failed to set RUST_BACKTRACE: {}", e);
    }
    actix::Arbiter::new().start(|| main());
}
