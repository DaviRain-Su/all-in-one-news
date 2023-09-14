use super::message::{Message, Messages};
// use retry::delay::Fixed;
// use retry::OperationResult;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Article {
    pub link: String,
    pub title: String,
}

impl Article {
    pub fn title(&self) -> &str {
        &self.title
    }

    pub async fn content(&self) -> anyhow::Result<Messages> {
        let article_url = self.to_string();
        let response = reqwest::get(article_url).await?;
        let body = response.text().await?;
        // dbg!(body.clone());
        // println!("body : {}", body);

        let mut messages = Messages::new();

        let document0 = Html::parse_document(&body);
        let selector0 = Selector::parse("p.vice-title").unwrap();
        let content0 = document0
            .select(&selector0)
            .next()
            .unwrap()
            .text()
            .collect::<String>();

        let content1 = content0
            .split('\n')
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|item| item.trim())
            .filter(|item| !item.is_empty())
            .collect::<Vec<&str>>();

        let author = content1[0].to_string();

        let time = content1[1]
            .to_string()
            .trim_start_matches("发表于")
            .trim()
            .to_string();

        let document = Html::parse_document(&body);
        let selector = Selector::parse("div.detail-body").unwrap();

        if let Some(element) = document.select(&selector).next() {
            let title_and_contents_selector = Selector::parse("h1,h2,h3,p,ul,a").unwrap();

            let title_and_contents = element.select(&title_and_contents_selector);

            let mut message = Message {
                author: author.clone(),
                time: time.clone(),
                ..Message::default()
            };
            for title_and_content in title_and_contents {
                let value = title_and_content.value().name();
                if value.contains("h3") || value.contains("h2") || value.contains("h1") {
                    let title = title_and_content.text().collect::<String>();
                    messages.push(message);
                    message = Message {
                        author: author.clone(),
                        time: time.clone(),
                        ..Message::default()
                    };
                    message.title = title;
                } else if value.contains('p') {
                    let content = title_and_content.text().collect::<String>();
                    let content = content.replace('\n', "");
                    if content.contains("From 日报小组")
                        || content.contains("社区学习交流")
                        || content.contains("Telgram Channel")
                        || content.contains("阿里云语雀订阅")
                        || content.contains("Steemit")
                        || content.contains("日报订阅")
                        || content.contains("独立日报订阅")
                    {
                    } else {
                        message.contents.push(content.clone());
                    }

                    let raw_html = title_and_content.inner_html();
                    let document = Html::parse_document(&raw_html);
                    let selector = Selector::parse("a").unwrap();

                    if let Some(element) = document.select(&selector).next() {
                        let link_content = element.value().attr("href").unwrap();
                        let link_content = link_content.replace('\n', "");
                        if !content.contains(&link_content) {
                            message.contents.push(link_content);
                        } else {
                            message.link = link_content.to_string();
                        }
                    }
                } else if value.contains("ul") {
                    let content = title_and_content.text().collect::<String>();

                    if content.contains("Rust.cc 论坛") || content.contains("微信公众号") {
                    } else {
                        message.contents.push(content);
                    }
                }
            }
            messages.push(message);
        }

        let msg = Messages {
            messages: messages
                .messages
                .into_iter()
                .filter(|item| !item.is_empty())
                .collect(),
        };

        Ok(msg)
    }
}

impl Display for Article {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let url = format!("https://rustcc.cn/article?id={}", self.link);
        write!(f, "{}", url)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArticleList {
    pub article_list: Vec<Article>,
}

impl ArticleList {
    pub fn push(&mut self, article: Article) {
        self.article_list.push(article);
    }
}

#[tokio::test]
async fn test_article_content() {
    let article = Article {
        link: "1ad7d23c-2392-4cce-9dc7-4bebcb3d51a5".to_string(),
        title: "【Rust日报】2023-09-09 Arroyo v0.5，高效地将流式数据传输到 S3".to_string(),
    };
    let messages = article.content().await.unwrap();
    for msg in messages.messages {
        println!("Title: {}", msg.title);
        println!("{}", msg);
    }
}

#[tokio::test]
async fn test_article_content1() {
    let article = Article {
        link: "11c0c645-a5bf-4a73-9e1c-314450e16ee7".to_string(),
        title: "【Rust日报】2023-08-11 Bevy 三周年🎂！".to_string(),
    };
    let messages = article.content().await.unwrap();
    for msg in messages.messages {
        println!("Title: {}", msg.title);
        println!("{}", msg);
    }
}

#[tokio::test]
async fn test_article_content2() {
    let article = Article {
        link: "d3109d9a-496f-4051-9f44-16e095d1f74f".to_string(),
        title: "【Rust日报】2023-09-05 cargo-audit 0.18 版本 - 性能、兼容性和安全性改进
"
        .to_string(),
    };
    let messages = article.content().await.unwrap();
    for msg in messages.messages {
        println!("Author: {}", msg.author);
        println!("time: {}", msg.time);
        println!("Title: {}", msg.title);
        println!("{}", msg);
    }
}

#[tokio::test]
async fn test_multi_article_content() {
    use super::SectionLink;

    let section_link = SectionLink { id: 1 };
    let article_list = section_link.get_articles().await.unwrap();
    // println!("{:#?}", article_list)

    for article in article_list.article_list {
        let messages = article.content().await.unwrap();
        for msg in messages.messages {
            println!("Title: {}", msg.title);
            println!("{}", msg);
        }
    }
}

#[tokio::test]
async fn test_multi_article_content_all() {
    use super::SectionLink;

    for idx in 1..=66 {
        let section_link = SectionLink { id: idx };
        let article_list = section_link.get_articles().await.unwrap();

        for article in article_list.article_list {
            let messages = article.content().await.unwrap();
            for msg in messages.messages {
                println!("Title: {}", msg.title);
                println!("{}", msg);
            }
        }
    }
}
