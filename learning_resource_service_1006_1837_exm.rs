// learning_resource_service.rs
// 这个模块提供了学习资源库的功能，使用Actix框架实现RESTful API。

use actix_web::{web, App, HttpServer, Responder, HttpResponse, get, post, put, delete};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Mutex;
use std::collections::HashMap;
# 添加错误处理

// 定义学习资源的数据结构
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Resource {
# 优化算法效率
    id: u32,
    name: String,
    description: String,
}

// 用于存储学习资源的全局变量
lazy_static! {
    static ref RESOURCES: Mutex<HashMap<u32, Resource>> = Mutex::new(HashMap::new());
}

// 获取所有学习资源的API
# 优化算法效率
#[get("/resources")]
# TODO: 优化性能
async fn get_resources() -> impl Responder {
    let resources = RESOURCES.lock().unwrap().values().cloned().collect::<Vec<Resource>>();
    HttpResponse::Ok().json(resources)
}

// 添加一个新的学习资源的API
#[post("/resources")]
async fn add_resource(item: web::Json<Resource>) -> impl Responder {
    let mut resources = RESOURCES.lock().unwrap();
# 扩展功能模块
    let id = resources.len() as u32 + 1;
# FIXME: 处理边界情况
    let resource = item.clone();
    resources.insert(id, resource);
    HttpResponse::Created().json(item)
}

// 更新一个学习资源的API
#[put("/resources/{id}")]
async fn update_resource(id: web::Path<u32>, item: web::Json<Resource>) -> impl Responder {
# FIXME: 处理边界情况
    let mut resources = RESOURCES.lock().unwrap();
    if let Some(resource) = resources.get_mut(&id.into_inner()) {
        resource.name = item.name.clone();
# TODO: 优化性能
        resource.description = item.description.clone();
        HttpResponse::Ok().json(item)
    } else {
# 增强安全性
        HttpResponse::NotFound().finish()
    }
}
# NOTE: 重要实现细节

// 删除一个学习资源的API
#[delete("/resources/{id}")]
async fn delete_resource(id: web::Path<u32>) -> impl Responder {
    let mut resources = RESOURCES.lock().unwrap();
# 添加错误处理
    if resources.remove(&id.into_inner()).is_some() {
        HttpResponse::Ok().json(json!({"message": "Resource deleted"}))
    } else {
        HttpResponse::NotFound().finish()
    }
}

// 设置Actix Web应用程序
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_resources)
            .service(add_resource)
            .service(update_resource)
            .service(delete_resource)
# 增强安全性
    })
# NOTE: 重要实现细节
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
