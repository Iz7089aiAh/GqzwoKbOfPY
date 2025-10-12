use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

/// PaymentService 结构体用于处理支付流程相关数据
struct PaymentService;

/// PaymentRequest 结构体定义支付请求的数据结构
struct PaymentRequest {
    amount: f64,
    currency: String,
}

impl PaymentService {
    /// process_payment 方法用于处理支付请求
    async fn process_payment(&self, req: PaymentRequest) -> Result<impl Responder, HttpResponse> {
        // 简单的支付验证逻辑
        if req.amount <= 0.0 {
            return Err(HttpResponse::BadRequest().body("Amount must be greater than zero").await);
        }

        // 假设这里是支付逻辑，我们只是简单地返回一个成功响应
        Ok(HttpResponse::Ok().body("Payment processed successfully").await)
    }
}

/// 创建一个异步函数来初始化并启动HTTP服务器
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::resource("/pay")
                    .route(web::post().to_async(|req: web::Json<PaymentRequest>| {
                        let service = PaymentService;
                        service.process_payment(req.into_inner())
                    })).unwrap(),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

/// PaymentRequest 的反序列化实现
impl actix_web::Json<PaymentRequest> for PaymentRequest {
    fn from_json(json: &actix_web::web::Json<PaymentRequest>) -> Result<Self, actix_web::error::JsonPayloadError> {
        // 这里可以添加额外的验证逻辑
        json.into_inner()
    }
}
