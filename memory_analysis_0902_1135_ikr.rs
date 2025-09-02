use actix_web::{get, HttpResponse, Responder, web};
use sys_info::*;

/// 获取系统内存使用情况的函数
/// 返回当前的内存使用情况，包括总内存和已使用的内存。
#[get("/memory")]
async fn memory_usage() -> impl Responder {
    match get_memory_info() {
        Ok(memory) => HttpResponse::Ok().json(memory),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

/// 系统内存信息结构体
/// 包含总内存和已使用的内存
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MemoryInfo {
    pub total: usize,
    pub used: usize,
}

/// 获取内存信息的函数
/// 使用 sys-info 库来获取内存信息。
fn get_memory_info() -> Result<MemoryInfo, std::io::Error> {
    let memory_info = sys_info::mem_info()?;
    Ok(MemoryInfo {
        total: memory_info.total,
        used: memory_info.free(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 设置日志
    env_logger::init_from_env(env_logger::Env::new().default_filter_or(