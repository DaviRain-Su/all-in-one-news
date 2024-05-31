use aion_types::rebase::response::SimpleDisplay;
use askama_actix::Template;

#[derive(Template)]
#[template(path = "all_news.html")]
pub struct AllNewsTemplate {
    pub items: Vec<SimpleDisplay>,
}
