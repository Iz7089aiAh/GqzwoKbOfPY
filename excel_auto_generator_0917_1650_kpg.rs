use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::http::StatusCode;
use actix_web::middleware::Logger;
use excel_writer::{Workbook, Worksheet, DataType};
use std::io::Cursor;
use bytes::Bytes;

// 定义一个结构体来处理生成Excel表格的请求
struct ExcelGenerator;

impl ExcelGenerator {
    // 生成Excel表格
    fn generate_excel() -> Result<Bytes, actix_web::error::Error> {
        // 创建一个新的工作簿
        let mut wb = Workbook::new();
        
        // 添加一个工作表
        let mut worksheet = wb.add_sheet("Sheet1");
        
        // 添加数据到工作表
        worksheet.write(0, 0, "Header 1", DataType::Str)?;
        worksheet.write(0, 1, "Header 2", DataType::Str)?;
        worksheet.write(1, 0, "Row 1, Cell 1", DataType::Str)?;
        worksheet.write(1, 1, "Row 1, Cell 2", DataType::Str)?;
        
        // 将工作簿写入到内存中的Cursor对象
        let mut cursor = Cursor::new(Vec::new());
        wb.write(&mut cursor)?;
        
        // 将内存中的Cursor对象转换为Bytes
        Ok(Bytes::from(cursor.into_inner()))
    }
}

// 定义一个异步的处理函数来处理生成Excel表格的请求
async fn generate_excel_handler() -> impl Responder {
    // 调用ExcelGenerator的generate_excel方法生成Excel表格
    match ExcelGenerator::generate_excel() {
        Ok(bytes) => {
            // 如果生成成功，返回200状态码和Excel表格的Bytes
            HttpResponse::Ok().content_type("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet").body(bytes)
        },
        Err(_) => {
            // 如果生成失败，返回500状态码和错误信息
            HttpResponse::InternalServerError().body("Failed to generate Excel file")
        },
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 启动HTTP服务器
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default()) // 添加日志中间件
            .route("/generate_excel", web::get().to(generate_excel_handler)) // 定义路由和处理函数
    })
    .bind("127.0.0.1:8080")? // 绑定IP和端口
    .run()
    .await
}
