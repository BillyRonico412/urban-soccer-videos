use axum::{
    Router,
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing,
};
use maud::{DOCTYPE, Markup, html};
use serde::Deserialize;
use std::println;
use tower_http::services::ServeFile;

use crate::model::get_actions_from_match_url;

mod model;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new()
        .route("/", routing::get(index_handler))
        .route("/actions", routing::get(actions_handler))
        .route_service("/output.css", ServeFile::new("assets/output.css"));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Server running into http://localhost:3000");
    axum::serve(listener, app).await?;
    Ok(())
}

async fn index_handler() -> Result<Markup, AppError> {
    Ok(layout(html! {
        form
            class="w-screen h-screen flex justify-center-safe items-center-safe"
            action="/actions"
            method="GET"
        {
            div class="flex flex-col items-center gap-6" {
                h1 class="font-bold text-2xl" {
                    "Urban soccer videos"
                }
                p {
                    "Un site permettant de faciliter la lecture des actions sur le site urban soccer"
                }
                label class="floating-label" {
                    input type="text" name="url" placeholder="Entrer le lien du match" class="input";
                    span { "Entrer le lien du match" }
                }
                button class="btn btn-primary" { "Voir les videos" }
            }
        }
    }))
}

#[derive(Deserialize)]
struct ActionsQuery {
    url: String,
}

async fn actions_handler(Query(query): Query<ActionsQuery>) -> Result<Markup, AppError> {
    let Ok(actions) = get_actions_from_match_url(&query.url).await else {
        return Ok(layout(html! {
            main class="h-screen w-screen flex items-center-safe justify-center-safe" {
                div role="alert" class="alert alert-error alert-soft" {
                    "Erreur lors du chargement de l'url. Soit l'url est incorect, soit le site est en panne !!!"
                }
            }
        }));
    };
    if actions.is_empty() {
        return Ok(layout(html! {
            main class="h-screen w-screen flex items-center-safe justify-center-safe" {
                div role="alert" class="alert alert-warning alert-soft" {
                    "Aucune action trouvé sur ce lien !"
                }
            }
        }));
    }
    Ok(layout(html! {
        main class="flex flex-col gap-8 items-center px-4 py-8 w-screen h-screen overflow-hidden" {
            h1 class="font-bold text-2xl" {
                "Urban soccer videos"
            }
            div class="flex-1 overflow-y-auto container flex flex-col gap-8" {
                @for (i, action) in actions.iter().enumerate() {
                    @if i > 0 { div class="divider" {} }
                    div class="flex items-center gap-8" {
                        div class="w-full aspect-video rounded-lg overflow-hidden" {
                            video src=(action.video_url_1) controls="true" class="w-full h-full";
                        }
                        @if let Some(video_url_2) = &action.video_url_2 {
                            div class="w-full aspect-video rounded-lg overflow-hidden" {
                                video src=(video_url_2) controls="true" class="w-full h-full";
                            }
                        }
                    }
                }
            }
        }
    }))
}

fn layout(children: Markup) -> Markup {
    html! {
        (DOCTYPE)
        head {
            meta charset="utf-8";
            title { "Urban soccer video" }
            link href="/output.css" rel="stylesheet";
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
