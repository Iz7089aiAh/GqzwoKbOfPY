use actix_web::{get, HttpResponse, Responder};
use serde::Serialize;
use std::collections::HashMap;

// 定义用户界面组件的结构
#[derive(Serialize)]
struct UiComponent {
    id: String,
    name: String,
    description: String,
}

// 创建一个包含用户界面组件的HashMap
// 用于模拟数据库或存储层
fn create_ui_components() -> HashMap<String, UiComponent> {
    let mut components = HashMap::new();
    components.insert(
        "1\.to_string(),
        UiComponent {
            id: "1".to_string(),
            name: "Button".to_string(),
            description: "Clickable button".to_string(),
        },
    );
    components.insert(
        "2\.to_string(),
        UiComponent {
            id: "2".to_string(),
            name: "TextField".to_string(),
            description: "Input field for text".to_string(),
        },
    );
    components
}

// 一个简单的路由处理函数，返回所有UI组件
#[get("/components")]
async fn get_components() -> impl Responder {
    let components = create_ui_components();
    HttpResponse::Ok().json(components)
}

// 程序入口点
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 配置Actix Web服务器
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            // 注册路由
            .service(get_components)
    })
    // 绑定服务器到端口8080
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
