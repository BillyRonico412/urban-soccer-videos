use maud::{Markup, html};

use crate::utils::{AppError, Htmx, page};

pub async fn handler(htmx: Htmx) -> Result<Markup, AppError> {
    Ok(page(
        htmx,
        html! {
            form
                class="w-full h-full flex justify-center-safe items-center-safe px-4 py-8"
                hx-get="/actions?index=0"
                hx-push-url="true"
                hx-target="#main"
                hx-disable="find button"
            {
                div class="flex flex-col items-center gap-6 max-w-xl" {
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
