use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_web::{web, Error, dev::ServiceResponse};
use actix_web::test::{self, TestServer};
use std::time::Duration;
use tokio::time::timeout;
use std::future::Future;

// 定义一个简单的响应结构体
struct ResponseData {
    message: String,
}

// 为这个结构体实现Responder，这样它就可以直接返回HTTP响应
impl Responder for ResponseData {
    type Error = Error;
    type Future = Result<Self, Self::Error>;

    fn respond_to(self, _: &HttpRequest) -> Self::Future {
        Ok(HttpResponse::Ok().json(self))
    }
}

// 定义路由和处理函数
#[get("/test")]
async fn test_route() -> impl Responder {
    ResponseData {
        message: "Test route is working!".to_string(),
    }
}

// 定义自动化测试函数
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use actix_web::{test, web};
    use serde_json::json;

    #[actix_web::test]
    async fn test_test_route() -> test::TestResult {
        let mut app = test::init_service(App::new()
            .service(test_route))
            .await;

        // 构造请求
        let req = test::TestRequest::with_uri("/test").to_request();

        // 执行请求
        let resp = app.call(req).await.unwrap();

        // 检查状态码和响应体
        assert_eq!(resp.status(), StatusCode::OK);
        assert!(resp.headers().has("group"));
        assert!(test::read_response(resp).await.text().await == "Test route is working!");

        Ok(())
    }
}

// 启动服务器
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(test_route)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
