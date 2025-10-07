use actix_web::{get, HttpResponse, Responder, web, App, HttpServer};
use std::collections::HashMap;
use std::sync::Mutex;

/// CacheService结构体用于管理缓存
struct CacheService {
    cache: Mutex<HashMap<String, String>>,
}

impl CacheService {
    /// 创建一个新的CacheService实例
    fn new() -> Self {
        CacheService {
            cache: Mutex::new(HashMap::new()),
        }
    }

    /// 获取缓存值
    fn get(&self, key: &str) -> Option<String> {
        let cache = self.cache.lock().unwrap();
        cache.get(key).cloned()
    }

    /// 将值添加到缓存
    fn put(&self, key: String, value: String) {
        let mut cache = self.cache.lock().unwrap();
        cache.insert(key, value);
    }
}

/// 获取缓存值的路由处理函数
#[get("/cache/{key}")]
async fn get_cache_item(service: web::Data<Mutex<CacheService>>, key: web::Path<String>) -> impl Responder {
    let service = service.lock().unwrap();
    match service.get(&key) {
        Some(value) => HttpResponse::Ok().body(value),
        None => HttpResponse::NotFound().finish(),
    }
}

/// 将值添加到缓存的路由处理函数
#[get("/cache/{key}/{value}"