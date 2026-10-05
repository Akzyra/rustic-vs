use json5::from_str;
use log::info;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;

const STABLE_URL: &str = "https://api.vintagestory.at/stable.json";
const UNSTABLE_URL: &str = "https://api.vintagestory.at/stable-unstable.json";

#[derive(Debug, Clone, Deserialize)]
pub struct DownloadInfoUrls {
    pub cdn: String,
    pub local: String,
}

#[derive(Debug, Clone, Deserialize)]
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
    // nt `linuxserver`, expected one of `linux`, `windows`, `windowsupdate`, `server`, `windowsserver`, `mac-x64`, `mac-arm64`"), position: Some(Position { line: 0, column: 972 }) }
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
    pub stable: HashMap<String, HashMap<DownloadPlatform, DownloadInfo>>,
    pub unstable: HashMap<String, HashMap<DownloadPlatform, DownloadInfo>>,
}

pub async fn load_versions() -> Result<Downloads, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let stable = handle_url(&client, STABLE_URL).await?;
    let unstable = handle_url(&client, UNSTABLE_URL).await?;

    info!(
        "fetched versions: {:?} stable, {:?} unstable",
        stable.len(),
        unstable.len()
    );
    Ok(Downloads { stable, unstable })
}

async fn handle_url(
    client: &reqwest::Client,
    url: &str,
) -> Result<HashMap<String, HashMap<DownloadPlatform, DownloadInfo>>, Box<dyn Error>> {
    let content = client
        .get(url)
        .header(reqwest::header::USER_AGENT, "rustic-vs launcher, by akzyra")
        .send()
        .await?
        .text()
        .await?;

    Ok(from_str::<
        HashMap<String, HashMap<DownloadPlatform, DownloadInfo>>,
    >(&content)?)
}
