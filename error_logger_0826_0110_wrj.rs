use actix_web::{web, App, HttpServer, Responder, HttpResponse, error::ErrorBadRequest};
use std::sync::Mutex;
use std::collections::HashMap;
# FIXME: 处理边界情况

// 定义一个全局的错误日志存储
lazy_static::lazy_static! {
# 添加错误处理
    static ref ERROR_LOGS: Mutex<HashMap<String, Vec<String>>> = Mutex::new(HashMap::new());
}

// 定义错误日志收集器的结构体
struct ErrorLogger;

// 实现错误日志收集器的功能
impl ErrorLogger {
    // 添加错误日志的函数
    fn log_error(error: &str, message: &str) {
# 增强安全性
        let mut logs = ERROR_LOGS.lock().unwrap();
        logs.entry(error.to_string()).or_insert_with(Vec::new).push(message.to_string());
# 增强安全性
    }
}

// 定义一个错误处理函数
async fn error_handler(err: ErrorBadRequest, _ctx: web::WebContext) -> impl Responder {
    // 调用错误日志收集器的函数，记录错误日志
    ErrorLogger::log_error(&err.to_string(), 