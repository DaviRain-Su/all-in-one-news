#![allow(non_snake_case)]
use dioxus::prelude::*;

pub mod rebase;
use futures::future::join_all;

use rebase::types::ListAllItemsResponse;
pub static REBASE_BASE__API_URL: &str = "http://127.0.0.1:8000";

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("sup");
    // launch the web app
    dioxus_web::launch(App);
}

pub fn App(cx: Scope) -> Element {
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

fn Stories(cx: Scope) -> Element {
    let story = use_future(cx, (), |_| get_rebase_dailys(10));

    match story.value() {
        Some(Ok(list)) => render! {
            div {
                for story in list {
                    StoryListing { story: story.clone() }
                }
            }
        },
        Some(Err(err)) => render! {"An error occurred while fetching stories {err}"},
        None => render! {"Loading items"},
    }
}

async fn resolve_story(
    full_story: UseRef<Option<ListAllItemsResponse>>,
    preview_state: UseSharedState<PreviewState>,
    story_id: i32,
) {
    if let Some(cached) = &*full_story.read() {
        *preview_state.write() = PreviewState::Loaded(cached.clone());
        return;
    }

    *preview_state.write() = PreviewState::Loading;
    if let Ok(story) = get_rebase_daily_preview(story_id).await {
        *preview_state.write() = PreviewState::Loaded(story.clone());
        *full_story.write() = Some(story);
    }
}

#[inline_props]
fn StoryListing(cx: Scope, story: ListAllItemsResponse) -> Element {
    let preview_state = use_shared_state::<PreviewState>(cx).unwrap();
    let ListAllItemsResponse {
        title,
        url,
        author: by,
        time,
        id,
        ..
    } = story;
    let full_story = use_ref(cx, || None);

    let url = url.as_str();
    let hostname = url
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .trim_start_matches("www.");

    let time = time.format("%D %l:%M %p");

    cx.render(rsx! {
        div {
            padding: "0.5rem",
            position: "relative",
            onmouseenter: move |_event| {
                resolve_story(full_story.clone(), preview_state.clone(), *id)
            },
            div {
                font_size: "1.5rem",
                a {
                    href: url,
                    onfocus: move |_event| {
                        resolve_story(full_story.clone(), preview_state.clone(), *id)
                    },
                    "{title}"
                }
                a {
                    color: "gray",
                    href: "https://news.ycombinator.com/from?site={hostname}",
                    text_decoration: "none",
                    " ({hostname})"
                }
            }
            div {
                display: "flex",
                flex_direction: "row",
                color: "gray",
                div {
                    padding_left: "0.5rem",
                    "by {by}"
                }
                div {
                    padding_left: "0.5rem",
                    "{time}"
                }

            }
        }
    })
}

#[derive(Clone, Debug)]
enum PreviewState {
    Unset,
    Loading,
    Loaded(ListAllItemsResponse),
}

fn Preview(cx: Scope) -> Element {
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

pub async fn get_rebase_daily_preview(id: i32) -> Result<ListAllItemsResponse, reqwest::Error> {
    let url = format!("{}/by_id?id={}", REBASE_BASE__API_URL, id);
    let result = reqwest::get(&url)
        .await?
        .json::<Vec<ListAllItemsResponse>>()
        .await?;

    assert!(result.len() == 1);

    Ok(result.first().unwrap().clone())
}

pub async fn get_rebase_dailys(count: usize) -> Result<Vec<ListAllItemsResponse>, reqwest::Error> {
    let url = format!("{}/ids", REBASE_BASE__API_URL);
    let stories_ids = &reqwest::get(&url).await?.json::<Vec<i32>>().await?[..count];

    let story_futures = stories_ids[..usize::min(stories_ids.len(), count)]
        .iter()
        .map(|&story_id| get_rebase_daily_preview(story_id));
    Ok(join_all(story_futures)
        .await
        .into_iter()
        .filter_map(|story| story.ok())
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_rebase_dailys() {
        let result = get_rebase_daily_preview(4198).await.unwrap();
        println!("result = {:?}", result);
        let result = get_rebase_dailys(10).await.unwrap();
        println!("result = {:?}", result);
    }
}
