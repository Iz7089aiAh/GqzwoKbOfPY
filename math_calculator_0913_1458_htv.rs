use actix_web::{web, App, HttpServer, Responder, HttpResponse, Post};

// 定义数学计算的错误类型
#[derive(Debug)]
enum MathError {
    InvalidInput,
    DivisionByZero,
}

// 实现错误转换为响应的trait
impl actix_web::error::ResponseError for MathError {
    fn error_response(&self) -> HttpResponse {
        match self {
            MathError::InvalidInput => HttpResponse::BadRequest().json("Invalid input"),
            MathError::DivisionByZero => HttpResponse::BadRequest().json("Division by zero"),
        }
    }
}

// 定义数学计算的请求体
#[derive(serde::Deserialize)]
struct MathRequest {
    a: f64,
    b: f64,
    op: String,
}

// 定义数学计算接口的响应体
#[derive(serde::Serialize)]
struct MathResponse {
    result: f64,
}

// 实现数学计算的服务
async fn calculate(req: web::Json<MathRequest>) -> Result<impl Responder, MathError> {
    let MathRequest { a, b, op } = req.into_inner();

    match op.as_str() {
        "add" => Ok(MathResponse { result: a + b }),
        "sub" => Ok(MathResponse { result: a - b }),
        "mul" => Ok(MathResponse { result: a * b }),
        "div" => {
            if b == 0.0 {
                Err(MathError::DivisionByZero)
            } else {
                Ok(MathResponse { result: a / b })
            }
        },
        _ => Err(MathError::InvalidInput),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/calculate").route(web::post().to(calculate)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
