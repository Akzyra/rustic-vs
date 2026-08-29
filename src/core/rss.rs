use rss::Channel;
use std::error::Error;

pub async fn load_news_feed() -> Result<Channel, Box<dyn Error>> {
    let client = reqwest::Client::new();

    let content = client
        .get("https://www.vintagestory.at/blog.html/?rss=1")
        .header(reqwest::header::USER_AGENT, "rustic-vs launcher, by akzyra")
        .send()
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&content[..])?;
    log::info!("got channel: {:?}", channel.title);

    Ok(channel)
}
