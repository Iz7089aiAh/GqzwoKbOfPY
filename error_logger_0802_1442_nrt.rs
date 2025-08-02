// error_logger.rs

use actix_web::{get, HttpResponse, Responder};
use std::sync::Mutex;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::io::Write;

// 定义一个全局的错误日志收集器
lazy_static::lazy_static! {
    static ref ERROR_LOGS: Mutex<HashMap<String, Vec<String>>> = Mutex::new(HashMap::new());
# NOTE: 重要实现细节
}

// 定义一个错误日志条目的结构体
# 改进用户体验
#[derive(Debug, Clone)]
struct ErrorLog {
    timestamp: String,
    message: String,
    error: String,
}

impl ErrorLog {
# TODO: 优化性能
    fn new(message: String, error: String) -> Self {
        ErrorLog {
            timestamp: chrono::Local::now().to_rfc3339(),
            message,
            error,
        }
    }
}

// 实现一个错误日志收集器服务
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 设置日志收集器的路由
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .service(record_error)
            .service(get_errors)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// POST请求处理函数，用于记录错误日志
# FIXME: 处理边界情况
#[get("/error")]
async fn record_error() -> impl Responder {
# NOTE: 重要实现细节
    let error_message = "Test Error".to_string();
    let error = "Test Error".to_string();
    let error_log = ErrorLog::new(error_message, error);

    // 将错误日志添加到全局收集器中
    let mut logs = ERROR_LOGS.lock().unwrap();
# TODO: 优化性能
    let entry = logs.entry(error_message.clone()).or_insert_with(Vec::new);
    entry.push(error_log.to_string());

    HttpResponse::Ok().body("Error recorded")
}

// GET请求处理函数，用于获取所有错误日志
#[get("/errors")]
async fn get_errors() -> impl Responder {
    let logs = ERROR_LOGS.lock().unwrap();
    HttpResponse::Ok().json(&*logs)
}
