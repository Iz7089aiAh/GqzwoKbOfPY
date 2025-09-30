use actix_web::{web, App, HttpResponse, HttpServer, Responder, get, post, put, delete, Responder as ActixResponder, error::ErrorInternalServerError, Error as ActixError};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Mutex;
use std::collections::HashMap;

// 模拟数据库存储订单
lazy_static::lazy_static! {
    static ref ORDERS: Mutex<HashMap<u64, Order>> = Mutex::new(HashMap::new());
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Order {
    id: u64,
    product_id: u64,
    quantity: u32,
    status: OrderStatus,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
enum OrderStatus {
    Pending,
    Processing,
    Fulfilled,
    Cancelled,
}

#[get("/orders/{id}")]
async fn get_order(id: web::Path<u64>) -> ActixResponder {
    let order_id = id.into_inner();
    let orders = ORDERS.lock().unwrap();
    match orders.get(&order_id) {
        Some(order) => HttpResponse::Ok().json(order),
        None => HttpResponse::NotFound().json(json!({"error": "Order not found"})),
    }
}

#[post("/orders")]
async fn create_order(order: web::Json<Order>) -> ActixResponder {
    let mut orders = ORDERS.lock().unwrap();
    let order_id = orders.keys().max().map_or(0, |id| id + 1);
    let new_order = Order {
        id: order_id,
        product_id: order.product_id,
        quantity: order.quantity,
        status: OrderStatus::Pending,
    };
    orders.insert(new_order.id, new_order.clone());
    HttpResponse::Created().json(new_order)
}

#[put("/orders/{id}")]
async fn update_order(id: web::Path<u64>, order: web::Json<Order>) -> ActixResponder {
    let mut orders = ORDERS.lock().unwrap();
    if let Some(existing_order) = orders.get_mut(&id.into_inner()) {
        *existing_order = order.clone().into_inner();
        HttpResponse::Ok().json(existing_order)
    } else {
        HttpResponse::NotFound().json(json!({"error": "Order not found"}))
    }
}

#[delete("/orders/{id}")]
async fn delete_order(id: web::Path<u64>) -> ActixResponder {
    let mut orders = ORDERS.lock().unwrap();
    if orders.remove(&id.into_inner()).is_some() {
        HttpResponse::Ok().json(json!({"message": "Order deleted"}))
    } else {
        HttpResponse::NotFound().json(json!({"error": "Order not found"}))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_order)
            .service(create_order)
            .service(update_order)
            .service(delete_order)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
