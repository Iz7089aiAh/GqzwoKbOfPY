use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::path::Path;
use std::fs::File;
use std::io::Write;
use xlsxwriter::XlsxWriter;

/// 定义用于接收请求数据的结构体
#[derive(Deserialize)]
struct GenerateExcelRequest {
    name: String,
    rows: usize,
}

/// 处理生成Excel表格的路由
#[get("/generate_excel")]
async fn generate_excel(req_data: web::Json<GenerateExcelRequest>) -> Result<HttpResponse, actix_web::Error> {
    let mut workbook = XlsxWriter::new(File::create("output.xlsx")?);
    let worksheet = workbook.add_worksheet(None)?;

    // 添加标题行
    worksheet.write_string(0, 0, "Name", None)?;
    worksheet.write_string(0, 1, "Value", None)?;

    // 添加数据行
    for i in 1..=req_data.rows {
        worksheet.write_string(i, 0, &req_data.name, None)?;
        worksheet.write_number(i, 1, i as f64, None)?;
    }

    // 保存工作簿
    workbook.close()?;

    Ok(HttpResponse::Ok().content_type("text/plain").body("Excel file generated successfully"))
}

/// 程序入口点
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(generate_excel)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

/// 错误处理
/// 这部分代码应该包含错误处理逻辑，但由于代码简洁性的考虑，这里省略了具体的错误处理代码。
/// 在实际应用中，应该为每个可能发生错误的操作提供详细的错误处理。

/// 注释和文档
/// 这个程序使用Actix Web框架来处理HTTP请求，并使用XlsxWriter库来生成Excel文件。
/// 用户可以通过发送包含`name`和`rows`的JSON请求到`/generate_excel`端点来生成一个Excel文件。
/// 生成的Excel文件将保存在服务器的当前目录下，文件名为`output.xlsx`。
