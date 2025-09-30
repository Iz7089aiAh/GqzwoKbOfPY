use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Mutex;

// 定义贷款申请的结构体
#[derive(Serialize, Deserialize, Debug)]
struct LoanApplication {
    applicant_name: String,
    amount: f64,
    duration: u32,  // 贷款期限（月）
}

// 定义贷款审批结果的结构体
#[derive(Serialize, Deserialize, Debug)]
struct LoanApprovalResult {
    status: String,
    message: String,
}

// 模拟数据库存储贷款申请结果
lazy_static::lazy_static! {
    static ref LOAN_APPROVALS: Mutex<Vec<LoanApprovalResult>> = Mutex::new(Vec::new());
}

// 贷款审批函数
fn approve_loan(app: LoanApplication) -> Result<LoanApprovalResult, &'static str> {
    if app.amount <= 0.0 || app.duration == 0 {
        // 简单的验证逻辑
        return Err("Invalid loan amount or duration");
    }

    // 模拟审批逻辑，实际应用中这里应该是更复杂的业务逻辑
    let result = LoanApprovalResult {
        status: "Approved".to_string(),
        message: format!("Loan for {} has been approved", app.applicant_name),
    };

    // 将审批结果添加到模拟数据库
    let mut approvals = LOAN_APPROVALS.lock().unwrap();
    approvals.push(result.clone());

    Ok(result)
}

#[post("/apply")]
async fn apply_loan(data: web::Json<LoanApplication>) -> impl Responder {
    match approve_loan(data.into_inner()) {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::BadRequest().body(e),
    }
}

#[get("/approvals")]
async fn get_approvals() -> impl Responder {
    let approvals = LOAN_APPROVALS.lock().unwrap();
    HttpResponse::Ok().json(&*approvals)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(apply_loan)
            .service(get_approvals)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// 添加必要的注释和文档
#[cfg(test)]
mod tests {
    use super::*;
    #[actix_web::test]
    async fn test_apply_loan() {
        let loan_app = LoanApplication {
            applicant_name: "John Doe".to_string(),
            amount: 5000.0,
            duration: 12,
        };
        let res = apply_loan(actix_web::web::Json(loan_app)).await;
        assert!(res.status().is_success());
    }
}
