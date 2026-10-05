use chrono::{DateTime, Local};
use rss::Channel;
use std::error::Error;
use std::io::BufReader;
use ureq::http::header::USER_AGENT;

#[derive(PartialEq, Clone)]
pub struct FeedEntry {
    pub title: String,
    pub description: String,
    pub date: DateTime<Local>,
    pub link: String,
}

impl FeedEntry {
    fn from_rss(item: rss::Item) -> Option<FeedEntry> {
        let title = item.title?;
        let description = item.description?.replace("=\"//", "=\"https://");
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

pub fn load_news_feed() -> Result<Vec<FeedEntry>, Box<dyn Error>> {
    let mut resp = ureq::get("https://www.vintagestory.at/blog.html/?rss=1")
        .header(USER_AGENT, crate::USER_AGENT)
        .call()?;
    let reader = resp.body_mut().as_reader();

    let channel = Channel::read_from(BufReader::new(reader))?;
    log::info!("got channel: {:?}", channel.title);

    Ok(channel
        .items
        .into_iter()
        .flat_map(FeedEntry::from_rss)
        .collect())
}
