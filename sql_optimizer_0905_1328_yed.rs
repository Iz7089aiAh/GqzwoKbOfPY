use actix_web::{post, web, App, HttpResponse, HttpServer, Responder, Responder as _};
use sqlparser::dialect::PostgreSql;
use sqlparser::parser::Parser;
use sqlparser::tokenizer::Tokenizer;
use sqlparser::Dialect;

// SQL查询优化器服务
#[post("/optimize")]
async fn optimize_sql(sql_query: web::Json<String>) -> impl Responder {
    let query = sql_query.into_inner();
    match optimize_query(&query) {
        Ok(optimized) => HttpResponse::Ok().json(optimized),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

// 优化SQL查询
fn optimize_query(query: &str) -> Result<String, sqlparser::parser::ParserError> {
    // 使用Tokenizer将SQL查询字符串分解成令牌
    let tokenizer = Tokenizer::new(query, PostgreSql);
    let tokens = tokenizer.tokenize()?;

    // 使用Parser解析令牌，生成抽象语法树（AST）
    let ast = Parser::parse_sql(&tokens)?;

    // 应用优化规则（示例中省略具体优化逻辑）
    // 在这里，您可以添加实际的优化逻辑，例如重新排序JOIN操作、推导选择性等

    // 将优化后的AST转回SQL查询字符串
    let optimized_query = format!(