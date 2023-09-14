use crate::tag::Tag;
use aion_parse::rebase::types::RebaseDaliyEpisode;

#[derive(Debug)]
pub struct RebaseDaliy {
    pub id: usize,
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
        let time = raw_time.parse::<chrono::DateTime<chrono::Utc>>().unwrap();
        // let time = time.format("%Y-%m-%d").to_string();

        Ok(Self {
            id: episode.id,
            author: episode.attributes.author,
            episode: episode.attributes.episode,
            introduce: episode.attributes.introduce,
            time,
            title: episode.attributes.title,
            url: episode.attributes.url,
            tag: vec![Tag::Rebase, Tag::Daily],
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
