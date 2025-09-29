use actix_web::{web, App, HttpServer, Responder, HttpResponse, error, get};
use serde::Deserialize;
use serde_json::json;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::io::Error;

// 定义一个用于解析请求体的结构体
#[derive(Deserialize)]
pub struct GenerationRequest {
    content: String,
    file_name: String,
}

// 定义错误处理
#[derive(Debug)]
pub enum GenerationError {
    FileCreationError(Error),
    WriteError(Error),
}

// 实现错误转换为响应的功能
impl error::ResponseError for GenerationError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            GenerationError::FileCreationError(_) => HttpResponse::InternalServerError()
                .json(json!{{"error": "Failed to create file"}}),
            GenerationError::WriteError(_) => HttpResponse::InternalServerError()
                .json(json!{{"error": "Failed to write to file"}}),
        }
    }
}

// 创建一个处理生成文件请求的函数
#[get("/generate")]
async fn generate_file(req_body: web::Json<GenerationRequest>) -> Result<impl Responder, GenerationError> {
    let GenerationRequest { content, file_name } = req_body.into_inner();

    // 创建文件并写入内容
    let path = Path::new(&file_name);
    let display = path.display();
    let mut file = match File::create(&path) {
        Ok(fc) => fc,
        Err(e) => return Err(GenerationError::FileCreationError(e)),
    };

    match file.write_all(content.as_bytes()) {
        Ok(_) => Ok(HttpResponse::Ok().json(json!{{"message": "File created successfully"}})),
        Err(e) => Err(GenerationError::WriteError(e)),
    }
}

// 定义主函数，启动服务器
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(generate_file)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
