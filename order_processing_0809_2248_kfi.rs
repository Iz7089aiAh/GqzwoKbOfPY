rError, Error};

// 定义订单结构体
struct Order {
    id: u32,
    status: String,
}

// 定义错误类型
#[derive(Debug)]
enum OrderProcessingError {
    OrderNotFound,
    InvalidStatus,
}

// 实现错误处理
impl std::fmt::Display for OrderProcessingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            OrderProcessingError::OrderNotFound => write!(f, "Order not found"),
            OrderProcessingError::InvalidStatus => write!(f, "Invalid status"),
        }
    }
}

impl actix_web::error::ResponseError for OrderProcessingError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            OrderProcessingError::OrderNotFound => HttpResponse::NotFound()
                .json("Order not found"),
            OrderProcessingError::InvalidStatus => HttpResponse::BadRequest()
                .json("Invalid status"),
        }
    }
}

#[get("/orders/{id}")]
async fn get_order(id: web::Path<u32>) -> Result<HttpResponse, OrderProcessingError> {
    let order_id = id.into_inner();

    // 模拟订单数据库
    let orders = vec![Order { id: 1, status: "pending".to_string() },
                      Order { id: 2, status: "shipped".to_string() },
                      Order { id: 3, status: "delivered".to_string() }];

    // 查找订单
    let order = orders.into_iter().find(|order| order.id == order_id).ok_or(OrderProcessingError::OrderNotFound)?;

    // 返回订单信息
    Ok(HttpResponse::Ok().json(order))
}

#[get("/orders/{id}/update")]
async fn update_order_status(id: web::Path<u32>, body: web::Json<(String)>) -> Result<HttpResponse, OrderProcessingError> {
    let order_id = id.into_inner();
    let new_status = body.into_inner();

    // 模拟订单数据库
    let mut orders = vec![Order { id: 1, status: "pending".to_string() },
                         Order { id: 2, status: "shipped".to_string() },
                         Order { id: 3, status: "delivered".to_string() }];

    if let Some(order) = orders.iter_mut().find(|order| order.id == order_id) {
        order.status = new_status.clone();
    } else {
        return Err(OrderProcessingError::OrderNotFound);
    }

    // 验证状态是否有效
    if !vec!["pending", "shipped", "delivered"].contains(&new_status.as_str()) {
        return Err(OrderProcessingError::InvalidStatus);
    }

    // 返回更新后的订单信息
    let order = orders.into_iter().find(|order| order.id == order_id).ok_or(OrderProcessingError::OrderNotFound)?;
    Ok(HttpResponse::Ok().json(order))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 启动HTTP服务器
    HttpServer::new(|| {
        App::new()
            .route("/orders/{id}", web::get().to(get_order))
            .route("/orders/{id}/update", web::post().to(update_order_status))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}