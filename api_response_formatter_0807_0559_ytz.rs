use actix_web::{get, HttpResponse, Responder, web};
# 改进用户体验
use serde::Serialize;
use serde_json::json;

/// APIResponse 是为了格式化API响应而设计的。
# 增强安全性
/// 它包含了状态码和返回的消息。
#[derive(Serialize)]
struct APIResponse<T> {
    code: u16,
    message: String,
# FIXME: 处理边界情况
    data: Option<T>,
# 添加错误处理
}

/// 定义一个简单的数据结构，用于展示API响应。
#[derive(Serialize)]
struct ExampleResponse {
# 扩展功能模块
    example: String,
}

/// 定义一个简单的API端点，返回格式化的JSON响应。
#[get("/format_response")]
async fn format_response() -> impl Responder {
    let response_data = ExampleResponse {
        example: "This is a formatted response".to_string(),
    };

    // 创建APIResponse实例，并设置状态码和消息。
    let api_response = APIResponse {
# NOTE: 重要实现细节
        code: 200,
        message: "Success".to_string(),
        data: Some(response_data),
    };

    // 将APIResponse序列化为JSON，并返回响应。
    HttpResponse::Ok().json(api_response)
}

/// 定义一个处理错误的API端点，返回格式化的错误响应。
#[get("/format_error")]
async fn format_error() -> impl Responder {
# TODO: 优化性能
    // 创建APIResponse实例，并设置错误状态码和消息。
# 改进用户体验
    let api_response = APIResponse {
        code: 400,
        message: "Bad Request".to_string(),
        data: None,
    };

    // 将APIResponse序列化为JSON，并返回响应。
    HttpResponse::BadRequest().json(api_response)
# 扩展功能模块
}

/// 主函数，启动Actix Web服务器。
#[actix_web::main]
# NOTE: 重要实现细节
async fn main() -> std::io::Result<()> {
    // 定义服务并启动服务器。
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .service(format_response)
# TODO: 优化性能
            .service(format_error)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
# TODO: 优化性能
}
