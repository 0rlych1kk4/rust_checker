use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use crate::report::{ValidationSummary, FileValidationResult};

#[get("/summary")]
async fn summary_endpoint() -> impl Responder {
    HttpResponse::Ok().body("Rust Checker Summary API is active.")
}

pub async fn run_dashboard(validation_summary: ValidationSummary) -> std::io::Result<()> {
    println!(
        "Launching web dashboard with {} total files...",
        validation_summary.total_files
    );

    HttpServer::new(|| {
        App::new()
            .service(summary_endpoint)
            .route("/", actix_web::web::get().to(|| async {
                HttpResponse::Ok().body("Rust Checker Web Dashboard Running...")
            }))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
