use axum::{Router, routing};
use std::println;
use tower_http::services::ServeFile;

mod handlers;
mod model;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new()
        .route("/", routing::get(handlers::index_handler))
        .route("/actions", routing::get(handlers::actions_handler))
        .route_service("/output.css", ServeFile::new("assets/output.css"))
        .route_service("/htmx.js", ServeFile::new("assets/htmx.js"));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    println!("Server running into http://localhost:8080");
    axum::serve(listener, app).await?;
    Ok(())
}
