use crate::{
    model::get_actions_from_match_url,
    utils::{AppError, Htmx, page},
};
use axum::extract::Query;
use maud::{Markup, html};
use serde::Deserialize;

pub async fn index_handler(htmx: Htmx) -> Result<Markup, AppError> {
    Ok(page(
        htmx,
        html! {
            form
                class="w-screen h-screen flex justify-center-safe items-center-safe px-4 py-8"
                hx-get="/actions"
                hx-push-url="true"
                hx-target="body"
                hx-disable="find button"
            {
                div class="flex flex-col items-center gap-6 w-full max-w-xl" {
                    h1 class="font-bold text-2xl" {
                        "Urban soccer videos"
                    }
                    p {
                        "Un site permettant de faciliter la lecture des actions sur le site urban soccer"
                    }
                    label class="floating-label w-full" {
                        input type="text" name="url" placeholder="Entrer le lien du match" class="input w-full";
                        span {
                            "Entrer le lien du match"
                        }
                    }
                    button class="btn btn-primary w-full" {
                        "Voir les videos"
                    }
                }
            }
        },
    ))
}

#[derive(Deserialize)]
pub struct ActionsQuery {
    pub url: String,
}

pub async fn actions_handler(
    htmx: Htmx,
    Query(query): Query<ActionsQuery>,
) -> Result<Markup, AppError> {
    let actions = get_actions_from_match_url(&query.url).await?;
    if actions.is_empty() {
        return Ok(page(
            htmx,
            html! {
                main class="h-screen w-screen flex items-center-safe justify-center-safe" {
                    div role="alert" class="alert alert-warning alert-soft" {
                        "Aucune action trouvé sur ce lien !"
                    }
                }
            },
        ));
    }
    Ok(page(
        htmx,
        html! {
            main class="flex flex-col gap-8 items-center px-4 py-8 w-screen h-screen overflow-hidden" {
                h1 class="font-bold text-2xl" {
                    "Urban soccer videos"
                }
                div class="flex-1 overflow-y-auto container flex flex-col gap-8" {
                    @for (i, action) in actions.iter().enumerate() {
                        @if i > 0 { div class="divider" {} }
                        div class="text-center font-medium" {
                            (&action.datetime.format("%d-%m-%Y %H:%M:%S"))
                        }
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
        },
    ))
}
