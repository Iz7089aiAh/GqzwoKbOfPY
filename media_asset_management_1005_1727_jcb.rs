use actix::prelude::*;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

// Define a struct to represent a Media Asset
#[derive(Serialize, Deserialize, Debug, Clone)]
struct MediaAsset {
    id: u32,
    title: String,
    description: String,
    tags: Vec<String>,
}

// Define a struct to manage the assets in memory
struct MediaAssetManager {
    assets: Mutex<HashMap<u32, MediaAsset>>,
    next_id: Mutex<u32>,
}

impl MediaAssetManager {
    fn new() -> Self {
        MediaAssetManager {
            assets: Mutex::new(HashMap::new()),
            next_id: Mutex::new(1),
        }
    }

    // Add a new media asset
    fn add_asset(&self, asset: MediaAsset) -> u32 {
        let mut next_id = self.next_id.lock().unwrap();
        let id = *next_id;
        *next_id += 1;
        let mut assets = self.assets.lock().unwrap();
        assets.insert(id, asset);
        id
    }

    // Get a media asset by id
    fn get_asset(&self, id: u32) -> Option<MediaAsset> {
        self.assets.lock().unwrap().get(&id).cloned()
    }

    // List all media assets
    fn list_assets(&self) -> Vec<MediaAsset> {
        self.assets.lock().unwrap().values().cloned().collect()
    }
}

// Define a handler to add a new media asset
async fn add_asset(asset: web::Json<MediaAsset>, manager: web::Data<Mutex<MediaAssetManager>>) -> impl Responder {
    let manager = manager.lock().unwrap();
    let id = manager.add_asset(asset.into_inner());
    HttpResponse::Ok().json(MediaAsset {
        id,
        ..asset.into_inner()
    })
}

// Define a handler to get a media asset by id
async fn get_asset(id: web::Path<u32>, manager: web::Data<Mutex<MediaAssetManager>>) -> impl Responder {
    let manager = manager.lock().unwrap();
    match manager.get_asset(*id) {
        Some(asset) => HttpResponse::Ok().json(asset),
        None => HttpResponse::NotFound().finish(),
    }
}

// Define a handler to list all media assets
async fn list_assets(manager: web::Data<Mutex<MediaAssetManager>>) -> impl Responder {
    let manager = manager.lock().unwrap();
    HttpResponse::Ok().json(manager.list_assets())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let manager = MediaAssetManager::new();
        App::new()
            .app_data(web::Data::new(Mutex::new(manager)))
            .route("/add", web::post().to(add_asset))
            .route("/get/{id}", web::get().to(get_asset))
            .route("/list", web::get().to(list_assets))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
