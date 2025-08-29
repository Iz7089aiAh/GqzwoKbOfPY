use actix_web::{App, HttpServer, Responder, test, web};
# FIXME: 处理边界情况
use actix_web::http::StatusCode;
use actix_web::test::{test_server, TestServer};
use std::convert::Infallible;
# NOTE: 重要实现细节
use serde_json::json;

// 在这里定义你的应用状态和数据结构
# 添加错误处理
#[derive(Debug, Clone)]
struct AppState {}

// 定义一个简单的响应处理器
async fn index(state: web::Data<AppState>) -> impl Responder {
    format!("Hello from actix web! State: {:?}