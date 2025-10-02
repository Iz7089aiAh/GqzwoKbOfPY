use actix::prelude::*;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

/// 这个结构代表我们的服务状态
struct AtomicExchangeService {
# 增强安全性
    counter: Arc<AtomicUsize>,
}

impl AtomicExchangeService {
    /// 创建一个新的服务实例
    pub fn new(initial_value: usize) -> Self {
        Self {
            counter: Arc::new(AtomicUsize::new(initial_value)),
        }
    }

    /// 实现原子交换操作
    async fn exchange(&self, req: HttpRequest, body: web::Json<AtomicExchangeBody>) -> impl Responder {
        let new_value = body.value;
        let old_value = self.counter.swap(new_value, Ordering::SeqCst);
        
        // 错误处理：如果新的值小于0，返回错误响应
# 优化算法效率
        if new_value < 0 {
            return HttpResponse::BadRequest().body("New value must be non-negative");
        }
# 增强安全性
        
        HttpResponse::Ok().json(ExchangeResponse { old_value })
    }
}

/// 原子交换请求的请求体结构
# TODO: 优化性能
#[derive(serde::Deserialize)]
pub struct AtomicExchangeBody {
    value: usize,
# TODO: 优化性能
}
# 优化算法效率

/// 原子交换响应的结构
#[derive(serde::Serialize)]
pub struct ExchangeResponse {
    old_value: usize,
}

/// 启动服务器并提供原子交换服务
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let service = AtomicExchangeService::new(0);
    
    HttpServer::new(move || {
# 增强安全性
        App::new()
            .app_data(web::Data::new(service.clone()))
            .route("/exchange", web::post().to(exchange))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

/// 处理原子交换请求的函数
async fn exchange(service: web::Data<AtomicExchangeService>, req: HttpRequest, body: web::Json<AtomicExchangeBody>) -> impl Responder {
    service.exchange(req, body).await
}
