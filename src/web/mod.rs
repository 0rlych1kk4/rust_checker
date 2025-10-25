use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

use crate::report::ValidationSummary;

/// App state wrapper for sharing the summary with handlers.
#[derive(Clone)]
struct AppState {
    summary: ValidationSummary,
}

#[get("/summary")]
async fn summary_route(state: web::Data<AppState>) -> impl Responder {
    // Return JSON; change to HTML if you prefer.
    HttpResponse::Ok().json(&state.summary)
}

/// Start a tiny dashboard showing the latest summary.
///
/// NOTE:
/// - This is a runner function; **do not** attach `#[get]` to it. The previous
///   error (`expected ValidationSummary, found summary`) came from putting a route
///   attribute on a non-handler function, which turned `summary` into a unit struct.
pub async fn run_dashboard(summary: ValidationSummary) -> std::io::Result<()> {
    let state = AppState { summary };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(summary_route)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
