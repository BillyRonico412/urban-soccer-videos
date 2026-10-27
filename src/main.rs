use axum::{
    Router,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing,
};
use maud::{DOCTYPE, Markup, html};
use std::println;

use crate::model::get_actions_from_match_url;

mod model;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new().route("/", routing::get(index_handler));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Server running into http://localhost:3000");
    axum::serve(listener, app).await?;
    Ok(())
}

async fn index_handler() -> Result<Markup, AppError> {
    Ok(layout(html!("Hello world")))
}

fn layout(children: Markup) -> Markup {
    html! {
        (DOCTYPE)
        head {
            meta charset="utf-8";
            title { "Urban soccer video" }
        }
        body {
            (&children)
        }
    }
}

struct AppError(anyhow::Error);

impl From<anyhow::Error> for AppError {
    fn from(value: anyhow::Error) -> Self {
        AppError(value)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.0.to_string()).into_response()
    }
}
