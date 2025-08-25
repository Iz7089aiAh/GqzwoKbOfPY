use actix_web::{get, HttpResponse, Responder, web, App, HttpServer};
use sysinfo::{System, SystemExt};

/// 系统性能监控工具
///
/// 这个工具提供了获取系统性能信息的接口
#[get("/monitor")]
async fn monitor() -> impl Responder {
    let mut sys = System::new_all();
    sys.refresh_all();
    let cpu_usage = sys.global_cpu_info().cpu_usage();
    let ram_usage = sys.used_memory();
    let swap_usage = sys.used_swap();

    HttpResponse::Ok().json(
        serde_json::json!({
            "cpu_usage": cpu_usage,
            "ram_usage": ram_usage,
            "swap_usage": swap_usage,
        })
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(monitor)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
