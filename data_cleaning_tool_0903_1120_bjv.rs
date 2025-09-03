use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

// 数据清洗工具配置结构体
#[derive(Deserialize)]
pub struct DataCleaningConfig {
    regex_patterns: HashMap<String, String>,
}

// 数据清洗和预处理工具
pub struct DataCleaningTool;

impl DataCleaningTool {
    // 创建新的数据清洗工具实例
    pub fn new(config: DataCleaningConfig) -> Self {
        DataCleaningTool
    }

    // 清洗数据
    pub fn clean_data(&self, data: &str) -> Result<String, String> {
        let mut cleaned_data = data.to_string();
        for (key, pattern) in config.regex_patterns.iter() {
            let re = Regex::new(pattern).map_err(|e| e.to_string())?;
            cleaned_data = re.replace_all(&cleaned_data, key).to_string();
        }
        Ok(cleaned_data)
    }
}

// HTTP服务，用于提供数据清洗工具接口
#[actix_web::main]
async fn main() -> io::Result<()> {
    let config = DataCleaningConfig {
        regex_patterns: HashMap::from([
            ("\s+".to_string(), " ".to_string()), // 将多个空格替换为一个空格
            ("[^a-zA-Z0-9 ]".to_string(), "".to_string()), // 移除非字母数字和空格字符
        ]),
    };

    let tool = DataCleaningTool::new(config);

    HttpServer::new(move || {
        App::new()
            .service(clean_data)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// HTTP接口，用于接收数据并返回清洗后的数据
#[get("/clean_data/{data}")]
async fn clean_data(data: web::Path<String>) -> impl Responder {
    let tool = DataCleaningTool::new(DataCleaningConfig::default());
    match tool.clean_data(&data) {
        Ok(cleaned_data) => HttpResponse::Ok().json(json!({
            "cleaned_data": cleaned_data,
        })),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}
