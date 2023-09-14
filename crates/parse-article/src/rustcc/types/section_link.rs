use super::article::{Article, ArticleList};
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SectionLink {
    pub id: usize,
}

impl Display for SectionLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let url = format!(
            "https://rustcc.cn/section?id=f4703117-7e6b-4caf-aa22-a3ad3db6898f&current_page={}",
            self.id
        );
        write!(f, "{}", url)
    }
}

impl SectionLink {
    pub async fn get_articles(&self) -> anyhow::Result<ArticleList> {
        let section_url = self.to_string();
        let response = reqwest::get(section_url).await?;

        let html = response.text().await?;

        let mut aticle_links = ArticleList {
            article_list: Vec::new(),
        };

        // 使用scraper解析HTML
        let document = Html::parse_document(&html);

        // 创建一个选择器来选择所有的<a>标签
        let selector = Selector::parse("a").unwrap();

        // 找到所有的<a>标签并打印链接和文本内容
        for link in document.select(&selector) {
            let href = link.value().attr("href").unwrap_or("");
            let text = link.text().collect::<String>();
            if href.starts_with("/article") {
                let link = href.trim_start_matches("/article?id=").to_string();
                let section_link = Article { link, title: text };
                aticle_links.push(section_link);
            }
        }

        Ok(aticle_links)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SectionLinkList {
    pub section_id: String,
    pub section_link_list: Vec<SectionLink>,
}

impl Display for SectionLinkList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.section_id)?;
        let mut section_link_list = String::new();
        for section_link in &self.section_link_list {
            section_link_list.push_str(&format!("{}\n", section_link));
        }
        write!(f, "{}", section_link_list)
    }
}

impl SectionLinkList {
    pub fn push(&mut self, section_link: SectionLink) {
        self.section_link_list.push(section_link);
    }

    pub fn total(&self) -> usize {
        self.section_link_list.len()
    }
}

#[tokio::test]
async fn test_get_first_article() {
    let section_link = SectionLink { id: 1 };
    let article_list = section_link.get_articles().await.unwrap();
    println!("{:#?}", article_list)
}
