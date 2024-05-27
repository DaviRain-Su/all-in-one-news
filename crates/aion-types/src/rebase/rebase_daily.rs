use crate::tag::Tag;
use aion_parse::common::Message;
use aion_parse::rebase::types::RebaseDaliyEpisode;
use sha256::digest;

#[derive(Debug)]
pub struct RebaseDaliy {
    pub id: usize,
    pub hash: String,
    pub author: String,
    pub episode: String,
    pub introduce: String,
    pub time: chrono::DateTime<chrono::Utc>,
    pub title: String,
    pub url: String,
    pub tag: Vec<Tag>,
}

impl TryFrom<RebaseDaliyEpisode> for RebaseDaliy {
    type Error = anyhow::Error;

    fn try_from(episode: RebaseDaliyEpisode) -> anyhow::Result<Self> {
        let raw_time = format!("{}T00:00:00.000Z", episode.attributes.time);
        let now_time = chrono::Utc::now();
        let time = raw_time
            .parse::<chrono::DateTime<chrono::Utc>>()
            .unwrap_or(now_time);

        //sha256 digest String
        let mut input = String::new();
        input.push_str(&episode.attributes.title);
        input.push_str(&episode.attributes.introduce);
        input.push_str(&episode.attributes.author);
        let hash = digest(input);

        Ok(Self {
            id: episode.id,
            hash,
            author: episode.attributes.author,
            episode: episode.attributes.episode,
            introduce: episode.attributes.introduce,
            time,
            title: episode.attributes.title,
            url: episode.attributes.url,
            tag: vec![Tag::Rebase],
        })
    }
}

impl TryFrom<Message> for RebaseDaliy {
    type Error = anyhow::Error;

    fn try_from(value: Message) -> Result<Self, Self::Error> {
        let raw_time = format!("{}T00:00:00.000Z", value.time);
        let now_time = chrono::Utc::now();
        let time = raw_time
            .parse::<chrono::DateTime<chrono::Utc>>()
            .unwrap_or(now_time);

        //sha256 digest String
        let mut input = String::new();
        input.push_str(&value.title);
        input.push_str(&value.time);
        input.push_str(&value.author);
        let hash = digest(input);

        let content = value.contents.into_iter().collect::<String>();
        let content = content.replace('\n', " ");

        Ok(Self {
            id: 0,
            hash,
            author: value.author,
            episode: String::from("rustcc"),
            introduce: content,
            time,
            title: value.title,
            url: value.link,
            tag: vec![Tag::Rust],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aion_parse::rebase::types::RebaseDaliyAttribute;

    pub fn fix_rebase_daily_episode() -> RebaseDaliyEpisode {
        RebaseDaliyEpisode {
            attributes: RebaseDaliyAttribute {
                author: "Qizhou".to_string(),
                episode: "#1315".to_string(),
                introduce: "PeerDAS，一个简单的数据可用采样方法。通过复用已有的P2P网络来实现数据可用采样。".to_string(),
                time: "2023-09-12".to_string(),
                title: "PeerDAS – a simpler DAS approach using battle-tested p2p components"
                    .to_string(),
                url: "https://ethresear.ch/t/peerdas-a-simpler-das-approach-using-battle-tested-p2p-components/16541".to_string()
            },
            id: 4366,
        }
    }

    #[test]
    fn test_time_parse() {
        let time = "2023-09-12T00:00:00.000Z";
        let time = time.parse::<chrono::DateTime<chrono::Utc>>().unwrap();
        println!("{:?}", time);
    }

    #[test]
    fn test_rebase_daily_episode_to_rebase_daily() {
        let episode = fix_rebase_daily_episode();
        let daily = RebaseDaliy::try_from(episode).unwrap();
        println!("{:#?}", daily);
    }
}
