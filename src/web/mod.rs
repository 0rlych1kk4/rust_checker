// src/web/mod.rs
use crate::report::ValidationSummary;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Rust Checker Web Dashboard is running.")
}

#[get("/summary")]
async fn get_summary(data: web::Data<Mutex<ValidationSummary>>) -> impl Responder {
    // Serialize by reference; no need to clone the summary.
    let summary = data.lock().expect("summary lock poisoned");
    HttpResponse::Ok().json(&*summary)
}

/// Starts the dashboard server and exposes the provided `ValidationSummary`.
pub async fn run_dashboard(current: ValidationSummary) -> std::io::Result<()> {
    let state = web::Data::new(Mutex::new(current));

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(index)
            .service(get_summary)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
