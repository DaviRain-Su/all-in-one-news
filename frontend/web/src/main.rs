#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    let title = "title";
    let by = "author";
    let score = 0;
    let time = chrono::Utc::now();
    let comments = "comments";

    cx.render(rsx! {
        div {
            "Hello, world!"
        }
        div {
            "Hello, Rust!"
        }
        div {
            "{title} by {by} ({score}) {time} {comments}"
        }
    })
}
