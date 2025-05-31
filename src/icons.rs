use iced::widget::image;
use log::{debug, error, info};
use std::env;
use std::fs::create_dir_all;

pub const DEFAULT_ICON: &[u8] = include_bytes!("../assets/default.png");
pub const ICONS_FOLDER: &str = "icons";

pub fn load_icon(name: &Option<String>) -> image::Handle {
    if let Some(name) = name {
        let root = env::current_dir().expect("Failed to get CWD");
        let path = root.join("icons").join(name);
        if path.is_file() {
            return path.strip_prefix(root).expect("failed rel path").into();
        }
    }
    image::Handle::from_bytes(DEFAULT_ICON)
}

pub fn load_icons() -> Vec<String> {
    let root = env::current_dir().expect("Failed to get CWD");
    let icons_folder = root.join(ICONS_FOLDER);

    if let Err(e) = create_dir_all(&icons_folder) {
        debug!("failed to ensure icons folder: {}", e);
        return Vec::new();
    }

    let file_names: Result<Vec<_>, _> = icons_folder.read_dir().map(|read_dir| {
        read_dir
            .flatten()
            .filter(|entry| entry.file_type().map(|e| e.is_file()).unwrap_or(false))
            .map(|file_entry| file_entry.file_name())
            .flat_map(|file_name| file_name.to_str().map(String::from))
            .collect()
    });

    match file_names {
        Ok(files) => {
            info!("loaded {} icons", files.len());
            files
        }
        Err(e) => {
            error!("failed reading icons folder: {}", e);
            Vec::new()
        }
    }
}
