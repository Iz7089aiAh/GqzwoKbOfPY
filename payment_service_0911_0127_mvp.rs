use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Mutex;
use std::sync::Arc;
use std::collections::HashMap;

// 定义支付请求的数据结构
#[derive(Serialize, Deserialize, Debug)]
struct PaymentRequest {
    amount: f64,
    currency: String,
    description: String,
}

// 定义支付响应的数据结构
#[derive(Serialize, Deserialize, Debug)]
struct PaymentResponse {
    status: String,
    transaction_id: String,
    message: String,
}

// 模拟数据库存储交易记录
#[derive(Clone)]
struct PaymentDatabase {
    transactions: Mutex<HashMap<String, PaymentResponse>>,
}

// 支付服务，包含数据库
struct PaymentService {
    database: Arc<PaymentDatabase>,
}

impl PaymentService {
    // 创建新的支付服务
    fn new() -> Self {
        Self {
            database: Arc::new(PaymentDatabase {
                transactions: Mutex::new(HashMap::new()),
            }),
        }
    }

    // 处理支付请求
    async fn process_payment(&self, req: PaymentRequest) -> impl Responder {
        let transaction_id = uuid::Uuid::new_v4().to_string();
        let mut transactions = self.database.transactions.lock().unwrap();
        
        // 模拟支付处理逻辑
        let payment_result = if req.amount > 0.0 {
            transactions.insert(
                transaction_id.clone(),
                PaymentResponse {
                    status: "success".to_string(),
                    transaction_id: transaction_id.clone(),
                    message: format!("Payment of ${}", req.amount),
                },
            );
            Some(PaymentResponse {
                status: "success".to_string(),
                transaction_id: transaction_id.clone(),
                message: format!("Payment of ${}", req.amount),
            })
        } else {
            transactions.insert(
                transaction_id.clone(),
                PaymentResponse {
                    status: "failed".to_string(),
                    transaction_id: transaction_id.clone(),
                    message: "Invalid amount".to_string(),
                },
            );
            None
        };
        
        if let Some(response) = payment_result {
            HttpResponse::Ok().json(response)
        } else {
            HttpResponse::BadRequest().json(json!({
                "status": "error",
                "message": "Invalid payment amount."
            }))
        }
    }
}

// 定义HTTP路由
async fn payment_route(service: web::Data<Arc<PaymentService>>, req_body: web::Json<PaymentRequest>) -> impl Responder {
    service.process_payment(req_body.into_inner()).await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(PaymentService::new()))
            .service(web::resource("/payment").route(post().to(payment_route)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
