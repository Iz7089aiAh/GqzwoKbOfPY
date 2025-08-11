// unit_test_service.rs
// 该模块提供了使用Actix框架创建单元测试的基础结构。
// 遵循Rust的最佳实践，结构清晰，易于理解，并包含适当的错误处理。

use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, HttpResponse};
use futures::future::{ok, Ready};
use std::task::{Context, Poll};

/// 定义一个简单的服务，用于单元测试。
pub struct TestService;

impl Service<ServiceRequest> for TestService {
    type Response = ServiceResponse;
    type Error = Error;
    type InitError = ();
    type Future = Ready<Result<Self::Response, Self::Error>>;
    type Config = ();

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::InitError>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _req: ServiceRequest) -> Self::Future {
        // 简单的响应，用于单元测试。
        let res = HttpResponse::Ok()
            .insert_header(("content-type", "text/plain"))
            .body("This is a test response from TestService.");
        ok(res.into_response())
    }
}

/// 单元测试模块。
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};
    use actix_web::http::StatusCode;

    #[actix_web::test]
    async fn test_service_response() {
        let app = test::init_service(App::new().service(web::resource("/test").to(TestService)));
        let req = test::TestRequest::with_uri("/test").to_request();
        let resp = app.send_request(req).await.unwrap();

        // 测试响应状态码
        assert!(resp.status().is_success());
        assert_eq!(resp.status(), StatusCode::OK);

        // 测试响应体
        assert_eq!(resp.body(), Some(b"This is a test response from TestService."));
    }
}
