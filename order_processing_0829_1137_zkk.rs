use actix_web::{web, App, HttpServer, Responder, HttpResponse, error::ErrorInternalServerError};

// 定义订单结构体
#[derive(Debug, Clone)]
struct Order {
    id: u32,
    product_id: u32,
    quantity: u32,
    status: OrderStatus,
}

// 定义订单状态枚举
#[derive(Debug, Clone)]
enum OrderStatus {
    Pending,
    Processing,
    Shipped,
    Delivered,
    Cancelled,
}

// 定义订单处理服务
struct OrderService;

impl OrderService {
    // 创建新订单
    async fn create_order(order: Order) -> Result<Order, ErrorInternalServerError> {
        // 这里是创建订单的逻辑，例如数据库操作
        // 假设操作成功，返回订单
        Ok(order)
    }

    // 更新订单状态
    async fn update_order_status(order_id: u32, new_status: OrderStatus) -> Result<Order, ErrorInternalServerError> {
        // 这里是更新订单状态的逻辑，例如数据库操作
        // 假设操作成功，返回更新后的订单
        Ok(Order {
            id: order_id,
            product_id: 1,
            quantity: 1,
            status: new_status,
        })
    }
}

// 定义错误处理结构体
#[derive(Debug)]
struct ErrorInternalServerError;

impl actix_web::error::ResponseError for ErrorInternalServerError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::InternalServerError().finish()
    }
}

// 定义HTTP处理函数
async fn create_order_endpoint(order: web::Json<Order>) -> impl Responder {
    match OrderService::create_order(order.into_inner()).await {
        Ok(order) => HttpResponse::Ok().json(order),
        Err(_) => Err(ErrorInternalServerError),
    }
}

async fn update_order_status_endpoint(order_id: web::Path<u32>, status: web::Json<OrderStatus>) -> impl Responder {
    match OrderService::update_order_status(order_id.into_inner(), status.into_inner()).await {
        Ok(order) => HttpResponse::Ok().json(order),
        Err(_) => Err(ErrorInternalServerError),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/order").route(web::post().to(create_order_endpoint)))
            .service(web::resource("/order/{id}").route(web::patch().to(update_order_status_endpoint)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
