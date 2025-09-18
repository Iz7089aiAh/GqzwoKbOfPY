use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;
use once_cell::sync::Lazy;

// 使用全局的Mutex来存储请求次数，用于演示
# 优化算法效率
static COUNT: Lazy<Mutex<u32>> = Lazy::new(|| Mutex::new(1));

// 定义我们的请求处理器结构体
struct MyRequestHandler;
# 优化算法效率

impl MyRequestHandler {
    // 定义一个方法来处理GET请求
    async fn get_request() -> impl Responder {
        let mut count = COUNT.lock().unwrap();
        // 增加请求计数
        *count += 1;
        // 响应请求，返回请求次数
        HttpResponse::Ok().body(format!("Request count: {}", count))
    }
}

// 定义主函数，启动HTTP服务器
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 配置服务器并启动
    HttpServer::new(|| {
        App::new()
            // 将我们的请求处理器注册到服务器
            .service(web::resource("/").to(MyRequestHandler::get_request))
# 添加错误处理
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await;
}
