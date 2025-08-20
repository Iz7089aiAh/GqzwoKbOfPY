use actix_web::{get, HttpResponse, Responder, web};
# 扩展功能模块
use serde::Deserialize;
use std::fs::File;
use std::io::Write;
use xlsxwriter::XlsxWriter;

// 定义一个结构体来存储生成Excel表格所需的数据
#[derive(Deserialize)]
pub struct ExcelData {
    rows: Vec<Vec<String>>, // 存储表格数据，每行是一个Vec<String>
}

// 实现一个服务来生成Excel表格
#[derive(Clone)]
pub struct ExcelGeneratorService;

// 实现服务方法，生成Excel文件并返回文件路径
#[get("/generate_excel")]
async fn generate_excel(data: web::Json<ExcelData>) -> impl Responder {
    let excel_data = data.into_inner();
    let file_name = "generated_excel.xlsx";
    
    match generate_excel_file(&excel_data, file_name) {
        Ok(path) => HttpResponse::Ok().json(path),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
# FIXME: 处理边界情况

// 实现生成Excel文件的函数
fn generate_excel_file(data: &ExcelData, file_name: &str) -> Result<String, String> {
# FIXME: 处理边界情况
    let file_path = format!("./{}", file_name);
    let mut workbook = XlsxWriter::new(File::create(&file_path).map_err(|e| e.to_string())?);
    let mut worksheet = workbook.add_worksheet().map_err(|e| e.to_string())?;
    
    for (row_idx, row) in data.rows.iter().enumerate() {
        for (col_idx, cell) in row.iter().enumerate() {
# 增强安全性
            worksheet.write_string(row_idx, col_idx, cell).map_err(|e| e.to_string())?;
# 优化算法效率
        }
    }
    
    workbook.close().map_err(|e| e.to_string())?;
    
    Ok(file_path)
# TODO: 优化性能
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 启动Actix Web服务器
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .service(generate_excel)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
