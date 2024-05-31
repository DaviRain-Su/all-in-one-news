use aion_types::rebase::response::SimpleDisplay;
use askama_actix::Template;

#[derive(Template)] // this will generate the code...
#[template(path = "all_news.html")] // using the template in this path, relative
                                    // to the `templates` dir in the crate root
pub struct AllNewsTemplate {
    pub items: Vec<SimpleDisplay>,
}
