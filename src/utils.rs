use axum::{extract::FromRequestParts, http::StatusCode, response::{IntoResponse, Response}};
use maud::{DOCTYPE, Markup, html};

pub struct AppError(anyhow::Error);

impl From<anyhow::Error> for AppError {
    fn from(value: anyhow::Error) -> Self {
        AppError(value)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR, 
            layout(html! {
                main class="h-full w-full flex items-center-safe justify-center-safe" {
                    div role="alert" class="alert alert-error alert-soft alert-vertical" {
                        h3 class="font-bold" {
                            "Erreur lors du chargement de l'url. Soit l'url est incorect, soit le site est en panne !"
                        }
                        p class="text-xs" {
                            (&self.0.to_string())
                        }
                    }
                }
            })
        ).into_response()
    }
}

pub fn layout(children: Markup) -> Markup {
    html! {
        (DOCTYPE)
        head {
            meta charset="utf-8";
            title { "Urban Soccer Video" }
            link href="/output.css" rel="stylesheet";
            script src="/htmx.js" {};
            script src="/alpine.js" {};
        }
        body class="w-screen h-screen overflow-hidden flex flex-col" {
            header class="navbar shadow-sm bg-base-200" {
                div class="container mx-auto" {
                    a 
                        hx-get="/" 
                        hx-push-url="true"
                        hx-target="body"
                        class="btn btn-ghost text-xl font-bold" 
                    { 
                        "Urban Soccer Videos"
                    }
                }
            }
            main id="main" class="flex-1 w-full overflow-hidden py-8" {
                div class="w-full h-full overflow-y-auto" {
                    div class="container mx-auto" {
                        (&children)
                    }
                }
            }
        }
    }
}

pub fn page(
    htmx: Htmx,    
    children: Markup
) -> Markup {
    if htmx.0 {
        children
    } else {
        layout(children)
    }
}

pub struct Htmx(bool);

impl<S> FromRequestParts<S> for Htmx {
    type Rejection = ();

    fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        async {
            Ok(Self(parts.headers.get("HX-Request").is_some()))
        }
    }
}
