use actix_web::{get, HttpResponse, Responder, web, App, HttpServer, Error, Result};

// 定义一个简单的搜索请求结构, 包含待搜索的关键词
# NOTE: 重要实现细节
#[derive(Debug, serde::Deserialize)]
struct SearchRequest {
    query: String,
}

// 搜索服务结构
struct SearchService;

// 实现搜索服务
impl SearchService {
    // 简单的搜索算法优化示例
# NOTE: 重要实现细节
    #[allow(dead_code)]
    pub fn search_documents(&self, query: &str) -> Vec<String> {
        // 这里模拟了一些文档数据
        let documents = vec![
            "The quick brown fox jumps over the lazy dog".to_string(),
            "Actix web is a powerful, pragmatic, and extremely fast web framework for Rust".to_string(),
            "Rust programming language empowers everyone to build reliable and efficient software".to_string(),
        ];

        // 搜索文档并返回匹配的结果
        documents.into_iter()
            .filter(|document| document.contains(query))
# NOTE: 重要实现细节
            .collect()
    }
}

// 定义一个搜索控制器
struct SearchController;

// 为搜索控制器实现方法
# NOTE: 重要实现细节
impl SearchController {
    #[get("/search")]
    async fn search(&self, query: web::Query<SearchRequest>) -> Result<impl Responder, Error> {
        let service = SearchService;
        let results = service.search_documents(&query.into_inner().query);
        Ok(HttpResponse::Ok().json(results))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 设置日志
    env_logger::init();

    // 创建HttpServer并注册搜索路由
    HttpServer::new(|| {
        App::new()
            .service(SearchController::search)
# FIXME: 处理边界情况
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
# 优化算法效率
