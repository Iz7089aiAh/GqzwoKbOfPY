use actix_web::{web, App, HttpResponse, HttpServer, Responder, get};
use serde::{Deserialize};
use serde_json::json;
use std::sync::Mutex;
use lazy_static::lazy_static;

// 定义全局可变的图表数据
lazy_static! {
    static ref CHART_DATA: Mutex<Vec<f64>> = Mutex::new(vec![]);
}

// 定义接收图表数据的请求体结构
#[derive(Deserialize)]
struct ChartInput {
    data: Vec<f64>,
}

// 定义图表数据的处理函数
async fn handle_chart_input(data: web::Json<ChartInput>) -> impl Responder {
    // 将接收到的数据添加到全局图表数据中
    let mut chart_data = CHART_DATA.lock().unwrap();
    chart_data.extend(data.data);

    // 返回成功响应
    HttpResponse::Ok().json(json!({"status": "success", "message": "Data added to chart"}))
}

// 定义获取图表数据的函数
#[get("/chart")]
async fn get_chart() -> impl Responder {
    // 获取全局图表数据的副本
    let chart_data = CHART_DATA.lock().unwrap().clone();

    // 返回图表数据
    HttpResponse::Ok().json(json!({"data": chart_data}))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 设置服务器监听的端口和地址
    HttpServer::new(|| {
        App::new()
            .service(handle_chart_input)
            .service(get_chart)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}