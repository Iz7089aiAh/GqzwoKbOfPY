use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

/// 定义一个响应数据结构
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ApiResponse<T> {
    /// 状态码
    status: i32,
    /// 消息
    message: String,
    /// 数据
    data: T,
}

/// 定义一个响应格式化工具
pub struct ResponseFormatter;

impl ResponseFormatter {
    /// 创建成功的响应
    pub fn success<T: serde::Serialize>(data: T) -> impl Responder {
        ApiResponse {
            status: 200,
            message: "Success".to_string(),
            data,
        }
    }

    /// 创建错误的响应
    pub fn error<T: serde::Serialize>(message: &str) -> impl Responder {
        ApiResponse {
            status: 400,
            message: message.to_string(),
            data: serde_json::json!({}),
        }
    }
}

/// 定义一个测试用的端点
#[get("/test")]
async fn test() -> impl Responder {
    ResponseFormatter::success("Hello, World!")
}

/// 定义一个错误测试用的端点
#[post("/error")]
async fn error_test() -> impl Responder {
    ResponseFormatter::error("Something went wrong")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(test)
            .service(error_test)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
