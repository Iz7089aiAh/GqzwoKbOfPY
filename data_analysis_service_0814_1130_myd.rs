use actix_web::{
    get,
    HttpResponse,
    Responder,
    web,
    App,
    HttpServer,
};
use serde::Deserialize;
use serde_json::json;

// 定义输入数据结构
#[derive(Deserialize)]
pub struct DataInput {
    pub data: Vec<f64>,
}

// 定义统计分析器结构
pub struct DataAnalysis;

// 实现分析功能
impl DataAnalysis {
    pub fn analyze(&self, input: &DataInput) -> Result<serde_json::Value, actix_web::Error> {
        let sum: f64 = input.data.iter().sum();
        let mean = sum / input.data.len() as f64;
        let variance = input.data.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / input.data.len() as f64;
        let std_deviation = variance.sqrt();

        Ok(json!({
            "sum": sum,
            "mean": mean,
            "variance": variance,
            "std_deviation": std_deviation,
        }))
    }
}

// 实现 Actix Web 控制器
#[get("/analyze")]
async fn analyze_data(data_analysis: web::Data<DataAnalysis>, input: web::Json<DataInput>) -> impl Responder {
    match data_analysis.analyze(&input) {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(_) => HttpResponse::InternalServerError().body("Failed to analyze data"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 创建 DataAnalysis 实例
    let data_analysis = web::Data::new(DataAnalysis);
    
    HttpServer::new(move || {
        App::new()
            .app_data(data_analysis.clone())
            .route("/analyze", web::post().to(analyze_data))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
