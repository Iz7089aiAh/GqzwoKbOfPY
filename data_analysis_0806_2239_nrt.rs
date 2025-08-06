use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;
use std::collections::HashMap;

// 定义请求的 JSON 数据结构
#[derive(Deserialize, Debug)]
struct AnalysisRequest {
    data: HashMap<String, Vec<f64>>,
}

// 定义响应结构
#[derive(Serialize, Debug)]
struct AnalysisResponse {
    analysis_results: Vec<f64>,
}

// 数据分析器服务
#[post("/analyze")]
async fn analyze_data(req_body: web::Json<AnalysisRequest>) -> impl Responder {
    // 错误处理
    if req_body.data.is_empty() {
        return HttpResponse::BadRequest().json("No data provided");
    }

    // 进行数据分析
    let analysis_results = perform_analysis(&req_body.data);

    // 创建响应
    let response = AnalysisResponse {
        analysis_results,
    };
    HttpResponse::Ok().json(response)
}

// 模拟的数据分析函数
fn perform_analysis(data: &HashMap<String, Vec<f64>>) -> Vec<f64> {
    let mut results = Vec::new();
    for values in data.values() {
        // 这里可以添加实际的数据分析逻辑，例如计算平均值、中位数等
        results.push(values.iter().sum::<f64>() / values.len() as f64);
    }
    results
}

fn main() -> std::io::Result<()> {
    // 启动 Actix 服务器
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .service(analyze_data)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
