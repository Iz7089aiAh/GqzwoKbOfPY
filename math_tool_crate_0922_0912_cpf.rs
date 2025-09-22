use actix_web::{get, post, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use std::fmt;

// 定义一个结构体来封装数学计算请求的结果
#[derive(Serialize, Deserialize, Debug)]
struct MathResult<T> {
    result: T,
}

// 定义一个结构体来封装加法请求的数据
#[derive(Serialize, Deserialize, Debug)]
struct AddRequest {
    a: f64,
    b: f64,
}

// 定义一个结构体来封装减法请求的数据
#[derive(Serialize, Deserialize, Debug)]
struct SubtractRequest {
    a: f64,
    b: f64,
}

// 定义一个结构体来封装乘法请求的数据
#[derive(Serialize, Deserialize, Debug)]
struct MultiplyRequest {
    a: f64,
    b: f64,
}

// 定义一个结构体来封装除法请求的数据
#[derive(Serialize, Deserialize, Debug)]
struct DivideRequest {
    a: f64,
    b: f64,
}

// 实现加法逻辑
#[post("/add")]
async fn add(req: actix_web::web::Json<AddRequest>) -> impl Responder {
    MathResult {
        result: req.a + req.b,
    }
}

// 实现减法逻辑
#[post("/subtract")]
async fn subtract(req: actix_web::web::Json<SubtractRequest>) -> impl Responder {
    if req.b == 0.0 {
        return HttpResponse::BadRequest().json("Cannot subtract by zero");
    }
    MathResult {
        result: req.a - req.b,
    }
}

// 实现乘法逻辑
#[post="/multiply"]
async fn multiply(req: actix_web::web::Json<MultiplyRequest>) -> impl Responder {
    MathResult {
        result: req.a * req.b,
    }
}

// 实现除法逻辑
#[post("/divide")]
async fn divide(req: actix_web::web::Json<DivideRequest>) -> impl Responder {
    if req.b == 0.0 {
        return HttpResponse::BadRequest().json("Cannot divide by zero");
    }
    MathResult {
        result: req.a / req.b,
    }
}

// 程序的入口点
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(add)
            .service(subtract)
            .service(multiply)
            .service(divide)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}