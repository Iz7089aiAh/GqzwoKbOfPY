use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

// 库存项目数据结构
#[derive(Serialize, Deserialize, Clone, Debug)]
struct InventoryItem {
    id: u32,
    name: String,
    quantity: u32,
}

// 库存管理系统
struct InventoryManager {
    items: Mutex<HashMap<u32, InventoryItem>>,
}

impl InventoryManager {
    // 创建新的库存管理系统
    fn new() -> Self {
        InventoryManager {
            items: Mutex::new(HashMap::new()),
        }
    }

    // 添加库存项
    fn add_item(&self, item: InventoryItem) -> Result<(), String> {
        let mut items = self.items.lock().unwrap();
        match items.insert(item.id, item) {
            Some(_) => Err("Item already exists".to_string()),
            None => Ok(()),
        }
    }

    // 更新库存项数量
    fn update_quantity(&self, item_id: u32, quantity: u32) -> Result<InventoryItem, String> {
        let mut items = self.items.lock().unwrap();
        match items.get_mut(&item_id) {
            Some(item) => {
                item.quantity = quantity;
                Ok(item.clone())
            },
            None => Err("Item not found".to_string()),
        }
    }

    // 获取库存项
    fn get_item(&self, item_id: u32) -> Result<InventoryItem, String> {
        let items = self.items.lock().unwrap();
        match items.get(&item_id).cloned() {
            Some(item) => Ok(item),
            None => Err("Item not found".to_string()),
        }
    }

    // 删除库存项
    fn remove_item(&self, item_id: u32) -> Result<(), String> {
        let mut items = self.items.lock().unwrap();
        match items.remove(&item_id) {
            Some(_) => Ok(()),
            None => Err("Item not found".to_string()),
        }
    }
}

// HTTP 请求处理器
async fn add_item_handler(manager: web::Data<Mutex<InventoryManager>>, item: web::Json<InventoryItem>) -> impl Responder {
    match manager.lock().unwrap().add_item(item.into_inner()) {
        Ok(_) => HttpResponse::Ok().json("Item added successfully"),
        Err(e) => HttpResponse::BadRequest().json(e),
    }
}

async fn update_quantity_handler(manager: web::Data<Mutex<InventoryManager>>, item_id: web::Path<u32>, quantity: web::Json<u32>) -> impl Responder {
    match manager.lock().unwrap().update_quantity(item_id.into_inner(), quantity.into_inner()) {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(e) => HttpResponse::BadRequest().json(e),
    }
}

async fn get_item_handler(manager: web::Data<Mutex<InventoryManager>>, item_id: web::Path<u32>) -> impl Responder {
    match manager.lock().unwrap().get_item(item_id.into_inner()) {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(e) => HttpResponse::BadRequest().json(e),
    }
}

async fn remove_item_handler(manager: web::Data<Mutex<InventoryManager>>, item_id: web::Path<u32>) -> impl Responder {
    match manager.lock().unwrap().remove_item(item_id.into_inner()) {
        Ok(_) => HttpResponse::Ok().json("Item removed successfully"),
        Err(e) => HttpResponse::BadRequest().json(e),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let manager = web::Data::new(Mutex::new(InventoryManager::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(manager.clone())
            .route("/add", web::post().to(add_item_handler))
            .route("/update/{id}", web::put().to(update_quantity_handler))
            .route("/get/{id}", web::get().to(get_item_handler))
            .route("/remove/{id}", web::delete().to(remove_item_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
