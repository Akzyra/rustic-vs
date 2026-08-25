use std::{
    ffi::{OsStr, OsString},
    path::Path,
};

use filenamify::filenamify;
use thiserror::Error;

const DIR_NAME: &str = "installs";
const GAME_DLL: &str = "Vintagestory.dll";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Install {
    pub id: String,
    pub version: Option<String>,
}

#[derive(Error, Debug)]
pub enum InstallError {
    #[error("Instance path not a directory")]
    NotDir(),

    #[error("Instance path has bad name: {0:?}")]
    BadDirName(OsString),

    #[error("Missing file: {0}")]
    MissingFile(String),
}
impl Install {
    pub fn new(id: String) -> Self {
        Self {
            id: filenamify(&id),
            version: None,
        }
    }

    pub fn load_from_dir(cwd: &Path, dir_name: &OsStr) -> Result<Install, InstallError> {
        let dir_path = cwd.join(DIR_NAME).join(dir_name);

        // must be a directory...
        if !dir_path.is_dir() {
            return Err(InstallError::NotDir());
        }
        // with an UTF name
        let Some(dir_name_str) = dir_name.to_str() else {
            return Err(InstallError::BadDirName(dir_name.to_os_string()));
        };

        if !dir_path.join(GAME_DLL).exists() {
            return Err(InstallError::MissingFile(GAME_DLL.into()));
        }

        let id = dir_name_str.to_string();
        //TODO: detect version from version-<VER>.txt

        Ok(Install::new(id))
    }
}

pub fn load_installs(cwd: &Path) -> Vec<Install> {
    let install_dir = cwd.join(DIR_NAME);
    if let Err(e) = std::fs::create_dir_all(&install_dir) {
        log::warn!("failed to ensure dir: {}", e);
        return Vec::new();
    }

    match install_dir.read_dir() {
        Ok(read_dir) => {
            let installs: Vec<Install> = read_dir
                .flatten()
                .filter(|e| e.file_type().map(|ft| ft.is_dir()).unwrap_or(false))
                .map(|e| e.file_name())
                .filter_map(|dir_name| match Install::load_from_dir(cwd, &dir_name) {
                    Ok(install) => {
                        log::debug!("loaded `{}`", install.id);
                        Some(install)
                    }
                    Err(e) => {
                        log::error!("loading `{}` failed: {}", dir_name.display(), e);
                        None
                    }
                })
                .collect();
            log::info!("loaded {} installs", installs.len());
            installs
        }
        Err(e) => {
            log::error!("failed reading dir: {}", e);
            Vec::new()
        }
    }
}
