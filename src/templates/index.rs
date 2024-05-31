use aion_types::rebase::response::SimpleDisplay;
use askama_actix::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub items: Vec<SimpleDisplay>,
}
