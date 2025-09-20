// cache_service.rs

use actix::{Actor, Context, Handler, Message, Supervised};
use actix_web::{web, HttpResponse, Responder};
use std::collections::HashMap;
use std::time::{Duration, Instant};

// Define a message to update cache
#[derive(Message)]
#[rtype(result = "Result<(), &'static str>")]
struct UpdateCache {
    key: String,
    value: String,
    ttl: Option<Duration>,
}

// Define a message to get cache value
#[derive(Message, Clone, Copy)]
#[rtype(result = "Result<Option<String>, &'static str>")]
struct GetCacheValue {
    key: String,
}

// CacheActor handles cache operations
struct CacheActor {
    cache: HashMap<String, (String, Instant)>,
}

impl Actor for CacheActor {
    type Context = Context<Self>;
}

impl Supervised for CacheActor {
    fn restarting(&mut self, _: &mut Self::Context) {
        // Optionally perform some action when cache is restarted
    }
}

impl CacheActor {
    fn new() -> Self {
        CacheActor {
            cache: HashMap::new(),
        }
    }
}

impl Handler<UpdateCache> for CacheActor {
    type Result = Result<(), &'static str>;

    fn handle(&mut self, msg: UpdateCache, _: &mut Self::Context) -> Self::Result {
        let key = msg.key;
        let value = msg.value;
        let ttl = msg.ttl;

        // Clean up expired cache entries
        self.cache.retain(|_, (_, timestamp)| Instant::now().duration_since(*timestamp) < ttl.unwrap_or(Duration::from_secs(300)));

        // Update cache with new value
        self.cache.insert(key, (value, Instant::now()));

        Ok(())
    }
}

impl Handler<GetCacheValue> for CacheActor {
    type Result = Result<Option<String>, &'static str>;

    fn handle(&mut self, msg: GetCacheValue, _: &mut Self::Context) -> Self::Result {
        let key = msg.key;

        // Return Some(value) if the key exists, None otherwise
        Ok(self.cache.get(&key).map(|(value, _)| value.clone()))
    }
}

// Define a struct to handle HTTP requests
struct CacheService;

impl CacheService {
    async fn update_cache(&self, key: String, value: String, ttl: Option<u64>) -> impl Responder {
        let ttl = ttl.map(Duration::from_secs);
        CacheActor::do_send(UpdateCache { key, value, ttl });
        HttpResponse::Ok()
    }

    async fn get_cache_value(&self, key: String) -> impl Responder {
        let response = CacheActor::do_send(GetCacheValue { key });
        match response.await {
            Ok(Some(value)) => HttpResponse::Ok().json(value),
            _ => HttpResponse::NotFound().finish(),
        }
    }
}

// Define a route for updating cache
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app = web::new()
        .service(web::resource("/update_cache/{key}")
            .route(web::post().to_async(CacheService::update_cache)))
        .service(web::resource("/get_cache_value/{key}")
            .route(web::get().to_async(CacheService::get_cache_value)));

    // Run the server
    actix_web::HttpServer::new(|| app)
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
