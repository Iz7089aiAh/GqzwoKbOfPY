use actix_web::{get, HttpResponse, Responder, web};
use serde::Serialize;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, Error};
use std::path::Path;

// 定义一个结构体来表示测试报告的数据
#[derive(Serialize)]
struct TestReport {
    id: u32,
    name: String,
    passed: bool,
    results: Vec<String>,
}

// 实现TestReport的Display特性，用于格式化输出
impl fmt::Display for TestReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Test Report: {{ id: {}, name: {}, passed: {}", self.id, self.name, self.passed)?;
        for result in &self.results {
            write!(f, ", result: {} 
", result)?;
        }
        write!(f, "}}")
    }
}

// 定义一个结构体来表示我们的应用程序的状态
struct AppState {
    test_reports: Vec<TestReport>,
}

// 实现ActixWeb的服务配置
impl actix_web::App<AppState> {
    // 添加一个新的测试报告生成器路由
    #[get("/report/{id}")]
    async fn generate_report(&self, path: web::Path<u32>) -> impl Responder {
        let report_id = path.into_inner();
        // 查找对应的测试报告
        let report = self.state().test_reports.iter()
            .find(|report| report.id == report_id)
            .cloned();
        
        // 处理找到或未找到测试报告的情况
        match report {
            Some(report) => HttpResponse::Ok().json(report),
            None => HttpResponse::NotFound().finish(),
        }
    }
}

fn main() -> std::io::Result<()> {
    // 初始化测试报告数据
    let reports = vec![
        TestReport {
            id: 1,
            name: "Unit Test".to_string(),
            passed: true,
            results: vec!["Test Case 1 passed".to_string(), "Test Case 2 passed".to_string()],
        },
        TestReport {
            id: 2,
            name: "Integration Test".to_string(),
            passed: false,
            results: vec!["Test Case 3 failed".to_string(), "Test Case 4 skipped".to_string()],
        },
    ];

    // 设置ActixWeb应用
    let app = actix_web::HttpServer::new(move || {
        // 将测试报告数据传递给ActixWeb应用
        actix_web::App::with_state(AppState { test_reports: reports.clone() })
            .configure(|cfg| {
                cfg.service(actix_web::App::generate_report);
            })
    }).bind("127.0.0.1:8080")?.run();

    // 启动ActixWeb服务器
    println!("Server running at http://127.0.0.1:8080/");
    app?
}
