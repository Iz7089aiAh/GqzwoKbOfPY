// homework_management_service.rs
//
// This module provides a simple homework management service using Actix framework.

use actix_web::{web, App, HttpServer, Responder, get, post, HttpResponse, error::ErrorInternalServerError};
use serde::{Deserialize, Serialize};
# NOTE: 重要实现细节
use serde_json::json;
use std::sync::Mutex;
use std::collections::HashMap;

// Define a struct to represent a homework assignment.
#[derive(Serialize, Deserialize, Debug)]
struct Homework {
    pub id: u32,
# 增强安全性
    pub title: String,
# NOTE: 重要实现细节
    pub description: String,
}

// Define a struct to handle homework data with thread-safe mutable access.
struct HomeworkStore {
    assignments: Mutex<HashMap<u32, Homework>>,
# 增强安全性
    next_id: Mutex<u32>,
}

// Define the API responses for adding and fetching homework assignments.
#[derive(Serialize, Deserialize, Debug)]
struct HomeworkResponse {
    success: bool,
    message: Option<String>,
    homework: Option<Homework>,
}
# FIXME: 处理边界情况

impl Default for HomeworkStore {
    fn default() -> Self {
# NOTE: 重要实现细节
        HomeworkStore {
            assignments: Mutex::new(HashMap::new()),
            next_id: Mutex::new(1),
# 优化算法效率
        }
    }
}

// Implement functions to interact with the homework store.
impl HomeworkStore {
    async fn add_homework(&self, title: String, description: String) -> HomeworkResponse {
        let mut next_id = self.next_id.lock().unwrap();
# NOTE: 重要实现细节
        let homework = Homework {
            id: *next_id,
            title,
            description,
        };
        let mut assignments = self.assignments.lock().unwrap();
# 优化算法效率
        assignments.insert(*next_id, homework.clone());
# TODO: 优化性能
        *next_id += 1;
        HomeworkResponse {
            success: true,
# 优化算法效率
            message: Some("Homework added successfully.".to_string()),
# 扩展功能模块
            homework: Some(homework),
        }
    }

    async fn get_homework(&self, id: u32) -> HomeworkResponse {
        let assignments = self.assignments.lock().unwrap();
        match assignments.get(&id) {
            Some(homework) => HomeworkResponse {
                success: true,
                message: None,
                homework: Some(homework.clone()),
            },
            None => HomeworkResponse {
                success: false,
# 改进用户体验
                message: Some("Homework not found.".to_string()),
# 扩展功能模块
                homework: None,
            },
# 增强安全性
        }
    }
}

// Define the Actix web service handlers.
async fn add_homework_handler(store: web::Data<HomeworkStore>, title: web::Json<String>, description: web::Json<String>) -> impl Responder {
    let response = store.add_homework(title.into_inner(), description.into_inner()).await;
    HttpResponse::Ok().json(response)
}

async fn get_homework_handler(store: web::Data<HomeworkStore>, web::Path(id): web::Path<u32>) -> impl Responder {
    let response = store.get_homework(id).await;
    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
# TODO: 优化性能
    // Initialize the homework store.
    let homework_store = web::Data::new(HomeworkStore::default());
# 添加错误处理

    // Start the HTTP server and define the routes.
    HttpServer::new(move || {
        App::new()
            .app_data(homework_store.clone())
            .service(add_homework)
            .service(get_homework)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// Define the Actix routes.
#[get("/homework/{id}")]
async fn get_homework(store: web::Data<HomeworkStore>, id: web::Path<u32>) -> impl Responder {
    get_homework_handler(store, id).await
}
# 改进用户体验

#[post("/homework")]
async fn add_homework(store: web::Data<HomeworkStore>, title: web::Json<String>, description: web::Json<String>) -> impl Responder {
    add_homework_handler(store, title, description).await
}
