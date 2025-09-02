use actix_web::{get, HttpResponse, Responder, web};
use serde::Serialize;
use serde_json::json;

/// API响应工具，用于格式化响应
#[derive(Serialize)]
pub struct ApiResponse<T> {
    status: String,
    data: T,
}

/// 控制器模块，包含API端点
pub mod controller {
    use super::*;
    use actix_web::{get, HttpResponse, Responder};

    #[get("/format")]
    async fn format_response() -> impl Responder {
        let data = "Hello, World!".to_string();
        let response = ApiResponse {
            status: "success".to_string(),
            data,
        };

        HttpResponse::Ok().json(response)
    }
}

/// 启动函数，用于启动服务器
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 设置日志
    env_logger::init();

    // 构建服务并启动服务器
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .route("/format", get(controller::format_response))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
