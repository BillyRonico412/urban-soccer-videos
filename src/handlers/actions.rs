use axum::extract::Query;
use maud::{Markup, html};
use serde::Deserialize;
use url::form_urlencoded;

use crate::{
    model::{Action, get_actions_from_match_url},
    utils::{AppError, Htmx, page},
};

#[derive(Deserialize)]
pub struct ActionsQuery {
    pub url: String,
    pub index: usize,
}

pub async fn handler(htmx: Htmx, Query(query): Query<ActionsQuery>) -> Result<Markup, AppError> {
    let actions = get_actions_from_match_url(&query.url).await?;
    if actions.is_empty() {
        return Ok(page(
            htmx,
            html! {
                main class="h-full w-full flex items-center-safe justify-center-safe" {
                    div role="alert" class="alert alert-warning alert-soft" {
                        "Aucune action trouvé sur ce lien !"
                    }
                }
            },
        ));
    }

    let data = format!("{{ index: {}, video_state: '12' }}", query.index);

    Ok(page(
        htmx,
        html! {
            main
                class="flex flex-col gap-8 items-center px-4 w-full h-full"
                x-data=(data)
            {
                (actions_html(&actions))
                (video_state_html(&actions))
                (prev_next_html(actions.len()))
                (action_items_html(&query, &actions))
            }
        },
    ))
}

fn actions_html(actions: &Vec<Action>) -> Markup {
    html! {
        @for (index, action) in actions.iter().enumerate() {
            (action_html(action, index))
        }
    }
}

fn action_html(action: &Action, index: usize) -> Markup {
    html! {
        div
            class="flex flex-col gap-4"
            x-show=(format!("index === {}", index))
        {
            div class="text-center font-semibold" {
                (&action.datetime.format("%d-%m-%Y %H:%M:%S"))
            }
            div class="flex items-center gap-8" {
                div
                    class="w-full aspect-video rounded-lg overflow-hidden"
                    x-show="video_state === '1' || video_state === '12'"
                {
                    video
                        src=(action.video_url_1)
                        controls="true"
                        controlslist="nofullscreen"
                        class="w-full h-full";
                }
                @if let Some(video_url_2) = &action.video_url_2 {
                    div
                        class="w-full aspect-video rounded-lg overflow-hidden"
                        x-show="video_state === '2' || video_state === '12'"
                    {
                        video src=(video_url_2) controls="true" class="w-full h-full";
                    }
                }
            }
        }
    }
}

fn video_state_html(actions: &Vec<Action>) -> Markup {
    let Some(first) = actions.get(0) else {
        return html! {};
    };
    if first.video_url_2.is_none() {
        return html! {};
    }
    html! {
        div class="flex gap-4 justify-center items-center" {
            button
                class="btn"
                x-bind:class="video_state === '1' && 'btn-primary'"
                x-on:click="video_state = '1'"
                {
                    "Video 1"
                }
                button
                class="btn"
                x-bind:class="video_state === '12' && 'btn-primary'"
                x-on:click="video_state = '12'"
                {
                    "Video 1 & Video 2"
                }
                button
                class="btn"
                x-bind:class="video_state === '2' && 'btn-primary'"
                x-on:click="video_state = '2'"
            {
                "Video 2"
            }
        }
    }
}

fn prev_next_html(action_len: usize) -> Markup {
    html! {
        div class="flex gap-4 justify-center items-center" {
            button
                class="btn btn-primary"
                x-bind:disabled="index === 0"
                x-on:click="index -= 1"
                {
                    "Précedent"
                }
            button
                class="btn btn-primary"
                x-bind:disabled=(format!("index === {}", action_len - 1))
                x-on:click="index += 1"
            {
                "Suivant"
            }
        }
    }
}

fn action_items_html(query: &ActionsQuery, actions: &Vec<Action>) -> Markup {
    html! {
        div class="flex flex-wrap gap-2" {
            @for (index, action) in actions.iter().enumerate() {
                (action_item_html(index, query, action))
            }
        }
    }
}

fn action_item_html(index: usize, query: &ActionsQuery, action: &Action) -> Markup {
    let url = format!(
        "/actions?index={}&url={}",
        index,
        form_urlencoded::byte_serialize(&query.url.clone().into_bytes()).collect::<String>(),
    );
    let on_click = format!(
        "window.history.pushState({{path: '{}'}}, '', '{}'); index={}",
        url, url, index
    );
    html! {
        btn
            x-on:click=(on_click)
            class="btn"
            x-bind:class=(format!("index == {} && 'btn-primary'", index))
        {
            (action.datetime.format("%H:%M:%S"))
        }
    }
}
