use chrono::{DateTime, Local};
use json5::from_str;
use log;
use rss::Channel;
use serde::Deserialize;
use std::collections::{BTreeMap, HashMap};
use std::error::Error;
use std::sync::LazyLock;

const RSS_URL: &str = "https://www.vintagestory.at/blog.html/?rss=1";
const STABLE_URL: &str = "https://api.vintagestory.at/stable.json";
const UNSTABLE_URL: &str = "https://api.vintagestory.at/stable-unstable.json";

static CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| {
    reqwest::ClientBuilder::new()
        .user_agent(crate::USER_AGENT)
        .build()
        .expect("failed to build client")
});

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct DownloadInfoUrls {
    pub cdn: String,
    pub local: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct DownloadInfo {
    pub filename: String,
    pub filesize: String,
    pub md5: String,
    pub urls: DownloadInfoUrls,
    #[serde(default)]
    pub latest: i32,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Deserialize)]
pub enum DownloadPlatform {
    #[serde(rename = "linux")]
    Linux,
    #[serde(rename = "windows")]
    Windows,
    #[serde(rename = "windowsupdate")]
    WindowsUpdate,
    #[serde(rename = "linuxserver", alias = "server")]
    LinuxServer,
    #[serde(rename = "windowsserver")]
    WindowsServer,
    #[serde(rename = "mac-x64")]
    MacX64,
    #[serde(rename = "mac-arm64")]
    MacArm64,
}
#[derive(Debug, Clone, Default)]
pub struct Downloads {
    pub stable: BTreeMap<String, HashMap<DownloadPlatform, DownloadInfo>>,
    pub unstable: BTreeMap<String, HashMap<DownloadPlatform, DownloadInfo>>,
}

pub async fn load_versions() -> Result<Downloads, Box<dyn Error>> {
    let stable = get_downloads(STABLE_URL).await?;
    let unstable = get_downloads(UNSTABLE_URL).await?;

    log::info!(
        "fetched versions: {:?} stable, {:?} unstable",
        stable.len(),
        unstable.len()
    );
    Ok(Downloads { stable, unstable })
}

async fn get_downloads(
    url: &str,
) -> Result<BTreeMap<String, HashMap<DownloadPlatform, DownloadInfo>>, Box<dyn Error>> {
    let resp = CLIENT.get(url).send().await?;
    let content = resp.text().await?;

    Ok(from_str(&content)?)
}

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

pub async fn load_news_feed() -> Result<Vec<FeedEntry>, Box<dyn Error>> {
    let resp = CLIENT.get(RSS_URL).send().await?;
    let bytes = resp.bytes().await?;

    let channel = Channel::read_from(&bytes[..])?;
    log::info!("got channel: {:?}", channel.title);

    Ok(channel
        .items
        .into_iter()
        .flat_map(FeedEntry::from_rss)
        .collect())
}
