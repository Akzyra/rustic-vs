use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

const CONFIG_NAME: &str = "rustic.json5";

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
}

pub const THEMES: &[Theme] = &[Theme::Light, Theme::Dark];

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Settings {
    pub theme: Theme,

    #[serde(skip)]
    pub cwd: PathBuf,
}
impl Settings {
    pub fn save(&self) {
        let config_path = self.cwd.join(CONFIG_NAME);
        json5::to_string(&self)
            .and_then(|json| {
                std::fs::write(&config_path, json).expect("write json works");
                Ok(())
            })
            .expect("serde works");
    }
}
pub fn load_settings(cwd: &Path) -> Settings {
    let config_path = cwd.join(CONFIG_NAME);
    if let Ok(json) = std::fs::read_to_string(&config_path) {
        if let Ok(mut settings) = json5::from_str::<Settings>(&json) {
            settings.cwd = cwd.to_owned();
            return settings;
        } else {
            log::warn!("failed parsing -> re-init");
        }
    } else {
        log::warn!("missing config -> re-init");
    }

    Settings {
        theme: Theme::Light,
        cwd: cwd.to_owned(),
    }
}
