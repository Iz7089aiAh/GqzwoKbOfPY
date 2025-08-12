use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::collections::HashMap;

// Define a struct to represent an item in the inventory
#[derive(Serialize, Deserialize, Debug, Clone)]
struct InventoryItem {
    id: u32,
    name: String,
    quantity: u32,
}

// Define a struct to hold the inventory data
struct Inventory {
    data: Mutex<HashMap<u32, InventoryItem>>,
}

impl Inventory {
    // Constructor to create a new Inventory
    fn new() -> Self {
        Inventory {
            data: Mutex::new(HashMap::new()),
        }
    }

    // Function to add an item to the inventory
    fn add_item(&self, item: InventoryItem) -> Result<(), &'static str> {
        let mut data = self.data.lock().unwrap();
        if data.contains_key(&item.id) {
            Err("Item already exists in the inventory")
        } else {
            data.insert(item.id, item);
            Ok(())
        }
    }

    // Function to get an item from the inventory by its ID
    fn get_item(&self, id: u32) -> Result<InventoryItem, &'static str> {
        let data = self.data.lock().unwrap();
        data.get(&id).cloned().ok_or("Item not found in the inventory")
    }

    // Function to update an item in the inventory
    fn update_item(&self, id: u32, new_quantity: u32) -> Result<(), &'static str> {
        let mut data = self.data.lock().unwrap();
        if let Some(item) = data.get_mut(&id) {
            item.quantity = new_quantity;
            Ok(())
        } else {
            Err("Item not found in the inventory")
        }
    }
}

// Define a handler for adding an item to the inventory
#[post("/add")]
async fn add_item_to_inventory(item: web::Json<InventoryItem>, inventory: web::Data<Mutex<Inventory>>) -> impl Responder {
    if inventory.lock().unwrap().add_item(item.into_inner()).is_ok() {
        HttpResponse::Ok()
    } else {
        HttpResponse::Conflict()
    }
}

// Define a handler for getting an item from the inventory
#[get("/items/{item_id}")]
async fn get_item_from_inventory(item_id: web::Path<u32>, inventory: web::Data<Mutex<Inventory>>) -> impl Responder {
    match inventory.lock().unwrap().get_item(*item_id) {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(_) => HttpResponse::NotFound()
    }
}

// Define a handler for updating an item in the inventory
#[post("/items/{item_id}")]
async fn update_item_in_inventory(item_id: web::Path<u32>, quantity: web::Json<u32>, inventory: web::Data<Mutex<Inventory>>) -> impl Responder {
    if inventory.lock().unwrap().update_item(*item_id, quantity.into_inner()).is_ok() {
        HttpResponse::Ok()
    } else {
        HttpResponse::NotFound()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Mutex::new(Inventory::new())))
            .service(add_item_to_inventory)
            .service(get_item_from_inventory)
            .service(update_item_in_inventory)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
