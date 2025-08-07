use actix_web::{web, App, HttpServer, HttpResponse, Responder};

// 定义一个简单的结构体，用于处理HTTP请求
# 添加错误处理
struct MyService;
# 优化算法效率

// 实现actix_web::Service trait，定义如何处理HTTP请求
# NOTE: 重要实现细节
impl actix_web::Service for MyService {
    type Request = actix_web::HttpRequest;
    type Response = actix_web::HttpResponse;
    type Error = actix_web::Error;
    type Future = actix_web::future::Ready<actix_web::Result<Self::Response, Self::Error>>;

    // 定义如何处理请求和返回结果
    fn call(&self, _req: Self::Request) -> Self::Future {
        actix_web::future::ok(HttpResponse::Ok().body("This is a response from Actix Web!"))
    }
}

// 定义启动HTTP服务器的函数
async fn start_server() -> std::io::Result<()> {
    // 创建HTTP服务器并监听localhost的8080端口
    HttpServer::new(|| {
        // 配置应用，添加路由和中间件
        App::new()
            // 添加一条路由，将GET请求映射到MyService处理函数
            .route("/", web::get().to(MyService))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// main函数，程序入口
#[actix_web::main]
# NOTE: 重要实现细节
async fn main() -> std::io::Result<()> {
    // 调用start_server函数启动服务器
    start_server().await
}
