use actix_web::{web, App, HttpResponse, HttpServer, Responder, middleware, dev::ServiceRequest, dev::ServiceResponse, Error, fs};
use derive_more::{Display, Error};
use futures::future::{ok, Ready};
use regex::Regex;
use std::task::{Context, Poll};
use std::pin::Pin;

// Define errors for our application.
#[derive(Debug, Display, Error)]
pub enum AppError {
    #[display(fmt = "{}")]
    InvalidInput(String),
}

// Define a middleware to sanitize input and prevent XSS attacks.
pub struct XssProtection;

impl<S, B> middleware::Middleware<S, B> for XssProtection
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Error = Error;
    type Response = ServiceResponse<B>;
    type Future = Ready<Result<Self::Response, Self::Error>>;

    fn start(&self, req: ServiceRequest) -> Self::Future {
        let sanitized_body = sanitize_input(req.body_mut());
        let sanitized_request = ServiceRequest::new(req.head(), sanitized_body);
        let fut = req.into_service().call(sanitized_request);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}

// Function to sanitize the input by removing any malicious code.
fn sanitize_input(body: &mut actix_web::dev::Body) -> actix_web::dev::Body {
    let body_mut = body.take();
    let sanitized = ""; // Placeholder for actual sanitization logic.
    actix_web::dev::Body::Bytes(sanitized.into())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(XssProtection)
            .route("/", web::get().to(|| HttpResponse::Ok().body("Hello, world!")))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// Placeholder for actual sanitization logic, using regex to remove potential XSS attacks.
// This is a simple example and may not cover all cases.
fn actual_sanitize_input(input: &str) -> String {
    let xss_regex = Regex::new(r"(<|>|on[a-zA-Z]+)").unwrap();
    let sanitized = xss_regex.replace_all(input, "").to_string();
    sanitized
}

// The actual middleware implementation would use `actual_sanitize_input` to sanitize the input.
impl<S, B> middleware::Middleware<S, B> for XssProtection
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Error = Error;
    type Response = ServiceResponse<B>;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn start(&self, req: ServiceRequest) -> Self::Future {
        let fut = async move {
            let sanitized_body = actual_sanitize_input(&req.body().await.unwrap_or_default());
            let response_body = actix_web::body::to_bytes(sanitized_body);
            let mut res = ServiceResponse::new(req.into_parts().0, response_body);
            res.map_body(|b| b.map_err(|_| actix_web::error::Error::new(actix_web::error::ErrorInternalServerError, "Internal Server Error")));
            Ok(res)
        };
        Box::pin(fut)
    }
}
