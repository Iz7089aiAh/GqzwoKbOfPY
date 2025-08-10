use actix_web::{get, web, App, HttpServer, Responder};
use serde::Deserialize;
use actix_web::HttpResponse;
use plotters::prelude::*;
use std::fs::File;
use std::io::BufWriter;

// 定义图表配置的请求体结构
#[derive(Deserialize)]
pub struct ChartConfig {
# FIXME: 处理边界情况
    width: u32,
    height: u32,
    data: Vec<(f64, f64)>,
}

// 定义图表生成器的错误类型
#[derive(Debug)]
enum ChartError {
    MissingData,
    PlottingError,
}

// 实现图表生成的函数
fn generate_chart(config: ChartConfig) -> Result<HttpResponse, ChartError> {
    if config.data.is_empty() {
        return Err(ChartError::MissingData);
    }

    let width = config.width as u32;
    let height = config.height as u32;
    let drawing_area = (0..width).into();
    let root_area = drawing_area.titled("",
# 增强安全性
        &FontDesc::new("sans-serif").bold().size(20));
    let mut chart = ChartBuilder::new(BufWriter::new(Vec::new()))
        .set_root_area(root_area)
        .build_ranged(0.0..1.0, 0.0..1.0)?;

    chart.draw_series(PointSeries::of_element(
        config.data,
        &|coord, size, color| {
            Rectangle::new([coord.0 * size.width as f64, coord.1 * size.height as f64],
                [size.width as f64, size.height as f64],
# FIXME: 处理边界情况
                color.filled())
        },
        &|coord, size, color| {
# NOTE: 重要实现细节
            Circle::new((coord.0 * size.width as f64, coord.1 * size.height as f64),
                0.1,
                color.filled())
        }).unwrap();

    Ok(HttpResponse::Ok().content_type("image/png").body(chart.into_inner().into_inner()))
# 添加错误处理
}

// 实现HTTP服务的配置
#[get("/generate_chart")]
async fn chart_endpoint(config: web::Json<ChartConfig>) -> impl Responder {
    match generate_chart(config.into_inner()) {
        Ok(chart_response) => chart_response,
        Err(ChartError::MissingData) => HttpResponse::BadRequest().json("Data is required"),
        Err(ChartError::PlottingError) => HttpResponse::InternalServerError().json("Failed to plot chart"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
# 添加错误处理
    HttpServer::new(|| {
        App::new()
            .service(chart_endpoint)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// 错误处理的实现
impl actix_web::error::ResponseError for ChartError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            ChartError::MissingData => actix_web::http::StatusCode::BAD_REQUEST,
            ChartError::PlottingError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
# 优化算法效率
        }
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            ChartError::MissingData => HttpResponse::BadRequest().json("Data is required"),
# 扩展功能模块
            ChartError::PlottingError => HttpResponse::InternalServerError().json("Failed to plot chart"),
# 改进用户体验
        }
    }
}

// 对代码的解释：
// - ChartConfig结构体用于解析来自客户端的图表配置信息。
// - generate_chart函数根据提供的配置信息生成图表，并返回HTTP响应。
// - chart_endpoint是一个HTTP端点，它接收图表配置并调用generate_chart函数。
# FIXME: 处理边界情况
// - main函数是程序的入口点，设置并运行HTTP服务器。
// - ChartError枚举定义了图表生成过程中可能遇到的错误。
# 改进用户体验
// - impl actix_web::error::ResponseError为ChartError提供错误响应实现。

// 注意：为了确保代码的可维护性和可扩展性，所有功能都被封装在适当的函数和结构体中。错误处理被清晰地定义，并遵循RUST最佳实践。