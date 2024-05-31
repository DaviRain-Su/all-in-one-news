use aion_types::rebase::response::SimpleDisplay;
use askama_actix::Template;

#[derive(Template)]
#[template(path = "latest_news.html")]
pub struct LatestNewsTemplate {
    pub items: Vec<SimpleDisplay>,
}
