use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::collections::HashMap;

// 购物车中的商品项
#[derive(Serialize, Deserialize, Clone)]
struct CartItem {
    id: i32,
    quantity: i32,
}

// 购物车结构
#[derive(Serialize, Deserialize, Clone)]
struct ShoppingCart {
    items: Vec<CartItem>,
}

// 购物车服务
struct CartService {
    carts: Mutex<HashMap<i32, ShoppingCart>>,
}

impl CartService {
    // 创建新的购物车服务
    fn new() -> CartService {
        CartService {
            carts: Mutex::new(HashMap::new()),
        }
    }

    // 添加商品到购物车
    fn add_item_to_cart(&self, cart_id: i32, item: CartItem) -> bool {
        let mut carts = self.carts.lock().unwrap();
        let cart = carts.entry(cart_id).or_insert(ShoppingCart { items: vec![] });
        if let Some(existing_item) = cart.items.iter_mut().find(|i| i.id == item.id) {
            existing_item.quantity += item.quantity;
        } else {
            cart.items.push(item);
        }
        true
    }

    // 获取购物车
    fn get_cart(&self, cart_id: i32) -> Option<ShoppingCart> {
        let carts = self.carts.lock().unwrap();
        carts.get(&cart_id).cloned()
    }
}

// 路由处理函数
#[post("/add_to_cart/{cart_id}")]
async fn add_to_cart(cart_id: web::Path<i32>, cart_item: web::Json<CartItem>, service: web::Data<Mutex<CartService>>) -> impl Responder {
    let mut service = service.lock().unwrap();
    if service.add_item_to_cart(cart_id.into_inner(), cart_item.into_inner()) {
        HttpResponse::Ok()
    } else {
        HttpResponse::InternalServerError()
    }
}

#[get("/cart/{cart_id}")]
async fn get_cart(cart_id: web::Path<i32>, service: web::Data<Mutex<CartService>>) -> impl Responder {
    let service = service.lock().unwrap();
    match service.get_cart(cart_id.into_inner()) {
        Some(cart) => HttpResponse::Ok().json(cart),
        None => HttpResponse::NotFound()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(Mutex::new(CartService::new()))
            .service(add_to_cart)
            .service(get_cart)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
