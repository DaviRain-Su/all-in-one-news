#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;
use fetch_data::get_latest_new;

pub mod fetch_data;
pub mod rebase;
pub mod story;

use crate::rebase::types::ListAllItemsResponse;
use story::*;

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    use_shared_state_provider(cx, || PreviewState::Unset);
    cx.render(rsx! {
        div {
            display: "flex",
            flex_direction: "row",
            width: "100%",
            div {
                width: "50%",
                Stories {}
            }
            div {
                width: "50%",
                Preview {}
            }
        }
    })
}

// New
fn Stories(cx: Scope) -> Element {
    let story = use_future(cx, (), |_| get_latest_new());

    match story.value() {
        Some(Ok(list)) => render! {
            div {
                for story in list {
                    StoryListing { rebase_list: story.clone() }
                }
            }
        },
        Some(Err(err)) => render! {"An error occurred while fetching stories {err}"},
        None => render! {"Loading items"},
    }
}

// New
#[derive(Clone, Debug)]
enum PreviewState {
    Unset,
    Loading,
    Loaded(ListAllItemsResponse),
}

// New
fn Preview(cx: Scope) -> Element {
    // New
    let preview_state = use_shared_state::<PreviewState>(cx)?;

    match &*preview_state.read() {
        PreviewState::Unset => render! {
            "Hover over a story to preview it here"
        },
        PreviewState::Loading => render! {
            "Loading..."
        },
        PreviewState::Loaded(story) => {
            let title = &story.title;
            let url = &story.url;
            let text = &story.introduce;
            render! {
                div {
                    padding: "0.5rem",
                    div {
                        font_size: "1.5rem",
                        a {
                            href: "{url}",
                            "{title}"
                        }
                    }
                    div {
                        dangerous_inner_html: "{text}",
                    }
                }
            }
        }
    }
}
