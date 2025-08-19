use actix_web::{get, HttpResponse, Responder, web};
    use serde::Deserialize;
    use excel_writer::{ExcelFile, Sheet, Cell, Style};

    /// 定义一个结构体来接收请求参数
    #[derive(Deserialize)]
    pub struct ExcelRequest {
        /// 定义表格标题
        title: String,
        /// 定义表格行数据
        rows: Vec<Vec<String>>,
    }

    /// Excel表格自动生成器
    /// 这个函数会根据请求参数生成一个Excel文件
    #[get("/generate_excel")]
    async fn generate_excel(req: web::Json<ExcelRequest>) -> impl Responder {
        let data = &req.rows;
        let title = &req.title;

        // 构建Excel文件
        let mut excel_file = ExcelFile::new();
        let mut sheet = Sheet::new();
        sheet.name = title.to_string();

        // 添加标题行
        let mut row = Vec::new();
        for header in data.first().unwrap_or(&Vec::new()) {
            row.push(Cell::String(header.clone()));
        }
        sheet.add_row(row);

        // 添加数据行
        for data_row in data.iter().skip(1) {
            let mut row = Vec::new();
            for cell in data_row {
                row.push(Cell::String(cell.clone()));
            }
            sheet.add_row(row);
        }

        // 添加样式
        let style = Style::new()
            .set_font_name("Arial")
            .set_font_size(12)
            .set_bold(true);
        sheet.apply_style(style, 0, 0, 0, data.first().unwrap_or(&Vec::new()).len());

        // 将sheet添加到文件中
        excel_file.add_sheet(sheet);

        // 将Excel文件转换为二进制数据并返回
        let mut buffer = Vec::new();
        excel_file.write(&mut buffer).unwrap();
        HttpResponse::Ok()
            .content_type("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet")
            .body(buffer)
    }

    /// Actix应用程序
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

    /// 定义模块结构
    pub mod excel_writer {
        // Excel文件操作相关的代码
    }

    /// 定义外部依赖项
    extern crate serde;
    extern crate actix_web;
    extern crate excel_writer;