use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use serde_json::{self, Value};

// 定义JSON数据格式转换器的请求结构体
#[derive(Deserialize, Serialize, Debug)]
struct JsonRequest {
    #[serde(flatten)]
    json_data: Value,
}

// 定义JSON数据格式转换器的响应结构体
#[derive(Serialize, Debug)]
struct JsonResponse {
    json_data: Value,
}

// 实现JSON数据格式转换器的请求处理函数
#[post("/convert")]
async fn convert_json(data: web::Json<JsonRequest>) -> impl Responder {
    // 尝试将请求体中的JSON数据进行格式化
    let formatted_json = match serde_json::to_string_pretty(&data.json_data) {
        Ok(json) => json,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };

    // 创建响应体
    let response = JsonResponse {
        json_data: serde_json::from_str(&formatted_json).unwrap_or_else(|_| Value::Null),
    };

    // 返回响应体
    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 启动HTTP服务器
    HttpServer::new(|| {
        App::new()
            // 配置转换器的路由
            .service(convert_json)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
