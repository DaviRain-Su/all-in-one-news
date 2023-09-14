use scraper::{Html, Selector};
pub mod article;
pub mod message;
pub mod section_link;

use section_link::{SectionLink, SectionLinkList};

pub async fn get_section_links() -> anyhow::Result<SectionLinkList> {
    let response =
        reqwest::get("https://rustcc.cn/section?id=f4703117-7e6b-4caf-aa22-a3ad3db6898f").await?;

    let html = response.text().await?;

    let mut secion_links = SectionLinkList {
        section_id: "f4703117-7e6b-4caf-aa22-a3ad3db6898f".to_string(),
        section_link_list: Vec::new(),
    };

    // 使用scraper解析HTML
    let document = Html::parse_document(&html);

    // 创建一个选择器来选择所有的<a>标签
    let selector = Selector::parse("a").unwrap();

    // 找到所有的<a>标签并打印链接和文本内容
    for link in document.select(&selector) {
        let href = link.value().attr("href").unwrap_or("");
        let _text = link.text().collect::<String>();
        if href.starts_with("/section") {
            let id = href
                .trim_start_matches(
                    "/section?id=f4703117-7e6b-4caf-aa22-a3ad3db6898f&current_page=",
                )
                .to_string()
                .parse::<usize>()?;
            let section_link = SectionLink { id };
            secion_links.push(section_link);
        }
    }

    Ok(secion_links)
}

#[tokio::test]
async fn test_get_section_links() {
    let r = get_section_links().await.unwrap();
    println!("{}", r);
}
