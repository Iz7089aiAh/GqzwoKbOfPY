use actix_web::{web, App, HttpServer, Responder};
use serde::Serialize;
use serde_json::json;

// 定义一个简单的数据结构
#[derive(Serialize)]
struct MyData {
    name: String,
    age: u32,
}

// 实现一个简单的业务逻辑函数，返回MyData类型的信息
async fn get_data() -> impl Responder {
    let my_data = MyData {
        name: "John".to_string(),
        age: 30,
    };

    // 将MyData结构体序列化为JSON
    json!(my_data)
}

async fn error_handling() -> impl Responder {
    // 演示错误处理，返回一个自定义的错误信息
    Err(actix_web::HttpResponse::InternalServerError().body("Internal Server Error"))
}

// 定义配置HTTP请求处理的函数
fn main() -> std::io::Result<()> {
    // 设置服务器监听端口
    HttpServer::new(|| {
        // 创建App应用，配置路由
        App::new()
            .route("/data", web::get().to(get_data))
            .route("/error", web::get().to(error_handling))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
