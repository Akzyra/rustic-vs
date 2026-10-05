use chrono::{DateTime, Local};
use rss::Channel;
use std::error::Error;

pub struct FeedEntry {
    pub title: String,
    pub description: String,
    pub date: DateTime<Local>,
    pub link: String,
}

impl FeedEntry {
    fn from_rss(item: rss::Item) -> Option<FeedEntry> {
        let title = item.title?;
        let description = item.description?;
        let link = item.link?;
        let date = DateTime::parse_from_rfc2822(&item.pub_date?)
            .map(|dt| dt.with_timezone(&Local))
            .ok()?;

        Some(FeedEntry {
            title,
            description,
            date,
            link,
        })
    }
}

pub async fn load_news_feed() -> Result<Vec<FeedEntry>, Box<dyn Error>> {
    let client = reqwest::Client::builder()
        .user_agent("rustic-vs launcher, by akzyra")
        .build()?;

    let content = client
        .get("https://www.vintagestory.at/blog.html/?rss=1")
        .send()
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&content[..])?;
    log::info!("got channel: {:?}", channel.title);

    Ok(channel
        .items
        .into_iter()
        .flat_map(FeedEntry::from_rss)
        .collect())
}
