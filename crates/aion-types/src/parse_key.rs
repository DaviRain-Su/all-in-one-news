use crate::tag::Tag;
use async_openai::{
    types::{ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs, Role},
    Client,
};
use std::collections::HashMap;

async fn core_parse_tag(content: &str) -> anyhow::Result<Vec<Tag>> {
    let mut result = vec![];
    let client = Client::new();

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u16)
        .model("gpt-3.5-turbo")
        .messages([
            ChatCompletionRequestMessageArgs::default()
                .role(Role::System)
                .content("You are a powerful analytical expert capable of summarizing content and extracting individual keywords. The keywords are separated by commas, and the generated keywords should not exceed five words.")
                .build()?,
            ChatCompletionRequestMessageArgs::default()
                .role(Role::User)
                .content(content)
                .build()?,
        ])
        .build()?;

    let response = client.chat().create(request).await?;

    for choice in response.choices {
        let msgs = choice.message.content.unwrap_or_default();
        if msgs.is_empty() {
            result.push(Tag::Other("".into()));
        }
        let mut msgs = msgs
            .split(',')
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|v| {
                let v = v.replace('.', "");
                Tag::try_from(v.trim())
            })
            .collect::<Result<Vec<_>, _>>()?;
        result.append(&mut msgs);
    }

    Ok(result)
}

pub async fn parse_tag(content: &str, loop_index: usize) -> anyhow::Result<Vec<Tag>> {
    let mut map = HashMap::new();
    for _ in 0..loop_index {
        let result = core_parse_tag(content).await?;
        for item in result {
            if let Some(value) = map.get_mut(&item) {
                *value += 1;
            } else {
                map.insert(item, 0usize);
            }
        }
    }

    let mut result = vec![];
    for (value, count) in map.iter() {
        if count >= &loop_index {
            result.push(value.clone());
        }
    }

    if result.is_empty() {
        for (value, count) in map.iter() {
            if count >= &(loop_index - 1) {
                result.push(value.clone());
            }
        }
    }
    Ok(result)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[tokio::test]
    #[ignore = "need provider openai key"]
    async fn test_parse_tag() {
        let content =
            "PeerDAS，一个简单的数据可用采样方法。通过复用已有的P2P网络来实现数据可用采样。";
        let _tags = parse_tag(content, 3).await.unwrap();
        println!("{:?}", _tags);
    }

    #[tokio::test]
    #[ignore = "need provider openai key"]
    async fn test_parse_tag2() {
        let content = "ERC-6551 是一个为每个 ERC-721 代币（NFT）提供智能合约账户的系统。它可被认为是一种能够将 NFT 变成钱包的技术 —— 本质上就是「NFT as wallet」。Loot Adventure 撰文盘点链游领域与 ERC-6551 结合的案例";
        let _tags = parse_tag(content, 3).await.unwrap();
        println!("{:?}", _tags);
    }
}
