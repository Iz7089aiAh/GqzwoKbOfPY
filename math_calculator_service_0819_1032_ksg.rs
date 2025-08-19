use actix_web::{get, HttpResponse, Responder, web};

// 定义数学计算工具集结构体
struct MathCalculator;

// 实现数学计算功能
impl MathCalculator {
    #[get("/add/{a}/{b}")]
    async fn add(a: f64, b: f64) -> impl Responder {
        Ok(HttpResponse::Ok().body(format!("{}", a + b)))
    }

    #[get("/subtract/{a}/{b}")]
    async fn subtract(a: f64, b: f64) -> impl Responder {
        Ok(HttpResponse::Ok().body(format!("{}", a - b)))
    }

    #[get("/multiply/{a}/{b}")]
    async fn multiply(a: f64, b: f64) -> impl Responder {
        Ok(HttpResponse::Ok().body(format!("{}", a * b)))
    }

    #[get("/divide/{a}/{b}")]
    async fn divide(a: f64, b: f64) -> Result<impl Responder, actix_web::Error> {
        if b == 0.0 {
            Err(actix_web::error::ErrorBadRequest("Cannot divide by zero"))
        } else {
            Ok(HttpResponse::Ok().body(format!("{}", a / b)))
        }
    }
}

// 主函数，设置路由并启动服务器
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化日志
    env_logger::init();

    // 设置路由
    let app = actix_web::App::new()
        .service(MathCalculator::add)
        .service(MathCalculator::subtract)
        .service(MathCalculator::multiply)
        .service(MathCalculator::divide);

    // 启动服务器
    actix_web::HttpServer::new(|| app)
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
