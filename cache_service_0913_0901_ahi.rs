use actix_web::{get, HttpResponse, Responder, web, App, HttpServer};
use std::collections::HashMap;
use std::sync::Mutex;

/// CacheService is responsible for caching data and providing it to the request handlers.
struct CacheService {
    data: Mutex<HashMap<String, String>>,
}

impl CacheService {
    /// Creates a new CacheService with an empty cache.
    fn new() -> Self {
        CacheService {
            data: Mutex::new(HashMap::new()),
        }
    }

    /// Sets a value in the cache.
    fn set(&self, key: String, value: String) {
        let mut cache = self.data.lock().unwrap();
        cache.insert(key, value);
    }

    /// Gets a value from the cache. If the value is not present, returns None.
    fn get(&self, key: &str) -> Option<String> {
        let cache = self.data.lock().unwrap();
        cache.get(key).cloned()
    }
}

/// Handles GET requests to fetch cached data.
#[get(