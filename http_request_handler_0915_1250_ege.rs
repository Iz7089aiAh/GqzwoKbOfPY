use actix_web::{get, HttpResponse, Responder, web};

// 定义一个结构体来表示HTTP请求处理器
struct HttpRequestHandler;

// 为结构体实现方法
impl HttpRequestHandler {
    // 定义一个GET请求处理器
    #[get("/")]
    async fn index(&self) -> impl Responder {
        HttpResponse::Ok().body("Welcome to the HTTP request handler!")
    }

    // 添加其他必要的处理器方法
    // ...
}

// 启动HTTP服务器的配置
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 启动服务并监听端口8080
    actix_web::HttpServer::new(|| {
        // 绑定HttpRequestHandler的路由
        actix_web::App::new()
            .service(HttpRequestHandler::index)
            // 绑定其他服务到路由
            // ...
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
