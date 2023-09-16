use crate::fetch_data::get_new_by_id;
use crate::rebase::types::ListAllItemsResponse;
use crate::PreviewState;
use dioxus::prelude::*;

async fn resolve_story(
    full_rebase: UseRef<Option<ListAllItemsResponse>>,
    preview_state: UseSharedState<PreviewState>,
    id: i32,
) {
    if let Some(cached) = &*full_rebase.read() {
        *preview_state.write() = PreviewState::Loaded(cached.clone());
        return;
    }

    *preview_state.write() = PreviewState::Loading;
    if let Ok(story) = get_new_by_id(id).await {
        *preview_state.write() = PreviewState::Loaded(story.clone());
        *full_rebase.write() = Some(story);
    }
}

#[inline_props]
pub fn StoryListing(cx: Scope, rebase_list: ListAllItemsResponse) -> Element {
    let preview_state = use_shared_state::<PreviewState>(cx).unwrap();
    let ListAllItemsResponse {
        id,
        author: by,
        episode: _,
        introduce: _,
        time,
        title,
        url,
        tag: _,
    } = rebase_list;

    let full_story = use_ref(cx, || None);
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
                    href: url.as_str(),
                    onfocus: move |_event| {
                        resolve_story(full_story.clone(), preview_state.clone(),*id)
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
