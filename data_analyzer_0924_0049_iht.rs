use actix_web::{get, HttpResponse, Responder, web, App, HttpServer, Responder as ActixResponder};
use serde::{Deserialize, Serialize};
use serde_json::json;

// 定义一个用于存放数据的结构体
# 添加错误处理
#[derive(Serialize, Deserialize, Debug)]
struct DataPoint {
    value: f64,
}

// 定义一个用于存放分析结果的结构体
#[derive(Serialize, Deserialize, Debug)]
struct AnalysisResult {
    mean: f64,
    median: f64,
    max: f64,
    min: f64,
}

// 数据分析器函数，计算数据的平均值、中位数、最大值和最小值
fn analyze_data(data: Vec<DataPoint>) -> Result<AnalysisResult, &'static str> {
    if data.is_empty() {
        return Err("No data provided for analysis");
    }

    let mut values: Vec<f64> = data.into_iter().map(|dp| dp.value).collect();
    values.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mean = values.iter().sum::<f64>() / values.len() as f64;
# NOTE: 重要实现细节
    let median = if values.len() % 2 == 0 {
        (values[values.len() / 2 - 1] + values[values.len() / 2]) / 2.0
    } else {
# FIXME: 处理边界情况
        values[values.len() / 2]
    };
    let max = *values.last().unwrap();
    let min = *values.first().unwrap();

    Ok(AnalysisResult { mean, median, max, min })
}
# TODO: 优化性能

// 创建一个Actix Web服务来处理HTTP请求
# 扩展功能模块
#[get("/analyze")]
async fn analyze_data_endpoint(data: web::Json<Vec<DataPoint>>) -> ActixResponder {
# 优化算法效率
    match analyze_data(data.into_inner()) {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(err) => HttpResponse::BadRequest().body(err),
# NOTE: 重要实现细节
    }
}
# 优化算法效率

#[actix_web::main]
async fn main() -> std::io::Result<()> {
# 扩展功能模块
    HttpServer::new(|| {
        App::new()
# 改进用户体验
            .service(analyze_data_endpoint)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}