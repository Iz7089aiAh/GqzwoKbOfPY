use actix_web::{get, HttpResponse, Responder, web};
# 优化算法效率
use actix_web::HttpServer;
use std::path::PathBuf;
use chrono::Local;

// 使用 `xlsxwriter` 库来生成 Excel 文件
use xlsxwriter::Workbook;

// 定义 `ExcelGenerator` 结构体，用于生成 Excel 文件
struct ExcelGenerator;

#[get("/generate_excel")]
async fn generate_excel() -> impl Responder {
# TODO: 优化性能
    // 设置 Excel 文件的路径
    let path = PathBuf::from("./output.xlsx");

    // 创建一个新的 Workbook 对象
    let mut workbook = Workbook::new(&path).expect("Failed to create workbook");

    // 添加一个工作表
    let worksheet = workbook.add_worksheet(None).expect("Failed to create worksheet");
# 改进用户体验

    // 设置 Excel 文件的标题和内容
    worksheet.write(0, 0, "Date").expect("Failed to write title");
    worksheet.write(0, 1, format!("Generated on {}", Local::now().format(\%Y-\%m-\%d \%H:\%M:\%S))).expect("Failed to write date");

    // 添加更多的 Excel 内容（例如表格数据）
    // 示例： worksheet.write(1, 0, "Data1").expect("Failed to write data");

    // 关闭 Workbook，保存文件
    workbook.close().expect("Failed to close workbook");
# 扩展功能模块

    // 返回生成的 Excel 文件路径
    HttpResponse::Ok().content_type("text/plain").body(format!("Excel file generated at: {}", path.display()))
}
# TODO: 优化性能

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 设置服务器地址和端口
    HttpServer::new(|| {
        // 定义路由
        actix_web::App::new()
            .service(generate_excel)
# 扩展功能模块
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// 请注意，为了使上述代码正常工作，你需要在 `Cargo.toml` 中添加以下依赖项：
// xlsxwriter = "0.1.0"
// chrono = "0.4.19"
// actix-web = { version = "4", features = ["macros"] }