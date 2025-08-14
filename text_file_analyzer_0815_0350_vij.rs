use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::str;

/// 分析文本文件内容的异步函数
#[get("/analyze/{file_path}")]
async fn analyze_text_file(file_path: web::Path<String>) -> impl Responder {
    let path = Path::new(&file_path.into_inner());
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    let mut contents = String::new();
    if let Err(_) = file.read_to_string(&mut contents) {
        return HttpResponse::InternalServerError().finish();
    }
    // 这里可以添加实际的文本内容分析逻辑
    // 例如：统计单词数量，检测敏感词等
    let analysis_result = analyze_contents(&contents);
    HttpResponse::Ok().json(analysis_result)
}

/// 模拟文本内容分析的函数
/// 此函数应根据实际需求实现具体的分析逻辑
fn analyze_contents(contents: &str) -> AnalysisResult {
    let word_count = contents.split_whitespace().count();
    AnalysisResult {
        word_count,
        sensitive_words_detected: false,
    }
}

/// 文本分析结果的结构体
#[derive(serde::Serialize)]
struct AnalysisResult {
    word_count: usize,
    sensitive_words_detected: bool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(analyze_text_file)
    }).
       .bind("127.0.0.1:8080")?.run()
       .await
}
