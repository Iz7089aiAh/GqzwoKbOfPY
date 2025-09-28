use actix_web::{web, App, HttpServer, Responder, HttpRequest, HttpResponse, Error, Result};
use actix_web::test::TestServer;
use serde::Deserialize;
use serde_json::json;

// 定义一个简单的请求体结构体
#[derive(Deserialize)]
struct MyRequestBody {
    message: String,
}

// 定义一个简单的响应体结构体
#[derive(Serialize)]
struct MyResponseBody {
    message: String,
}

// 定义一个简单的服务，处理POST请求
async fn my_service(req_body: web::Json<MyRequestBody>) -> Result<impl Responder> {
    let body = req_body.into_inner();
    Ok(HttpResponse::Ok().json(MyResponseBody {
        message: format!("Received message: {}", body.message),
    }))
}

// 单元测试函数
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;

    #[actix_web::test]
    async fn test_my_service() {
        let mut app = test::init_service(
            App::new()
                .service(web::resource("/test").route(web::post().to(my_service))),
        )
        .await;

        let req_body = MyRequestBody {
            message: "Hello, world!".to_string(),
        };

        let req = test::TestRequest::post()
            .uri("/test")
            .set_json(&req_body)
            .to_request();
        let resp = app.call(req).await.unwrap();

        assert!(resp.status().is_success());
        assert!(resp.body().is_json());
    }
}