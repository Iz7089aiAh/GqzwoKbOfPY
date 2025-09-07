use actix_web::{get, HttpResponse, Responder, web};
use actix_web::{App, HttpServer};
use serde::Deserialize;
use std::cmp::Ordering;

/// 定义一个结构体，用于接收请求数据
#[derive(Deserialize, Debug)]
struct SortRequest {
    data: Vec<i32>,
}

/// 实现排序服务
struct SortingService;

impl SortingService {
    /// 执行排序算法
    fn sort(&self, data: Vec<i32>) -> Vec<i32> {
        data.sort();
        data
    }
}

/// 定义一个Handler，用于处理排序请求
#[handler]
async fn sort_handler(sort_req: web::Json<SortRequest>) -> impl Responder {
    let service = SortingService;
    let sorted_data = service.sort(sort_req.into_inner().data);
    HttpResponse::Ok().json(sorted_data)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(sort_handler)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

/// 以下是单元测试用例
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web};
    use serde_json::json;

    #[actix_web::test]
    async fn test_sort_handler() {
        let app = test::init_service(App::new().service(sort_handler)).await;
        let req = test::TestRequest::with_header("content-type", "application/json")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}
