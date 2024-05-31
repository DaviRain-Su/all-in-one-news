use aion_types::rebase::response::SimpleDisplay;
//use askama::Template;
use askama_actix::Template;

#[derive(Template)] // this will generate the code...
#[template(path = "latest_news.html")] // using the template in this path, relative
                                       // to the `templates` dir in the crate root
pub struct LatestNewsTemplate {
    pub items: Vec<SimpleDisplay>,
}
