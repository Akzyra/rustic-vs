use filenamify::filenamify;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{
    ffi::{OsStr, OsString},
    path::Path,
};
use thiserror::Error;

const DIR_NAME: &str = "instances";
const CONFIG_NAME: &str = "instance.json5";
//const MODS_DIR_NAME: &str = "Mods";

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Instance {
    #[serde(skip)]
    pub id: String,

    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_id: Option<String>,

    #[serde(skip)]
    pub path: PathBuf,
}

#[derive(Error, Debug)]
pub enum InstanceError {
    #[error("Instance path not a directory")]
    NotDir(),

    #[error("Instance path has bad name: {0:?}")]
    BadDirName(OsString),

    #[error("Failed to create instance dir: {0}")]
    CreateDir(#[source] std::io::Error),

    #[error("Failed to serialize instance config: {0}")]
    Serialize(#[source] json5::Error),

    #[error("Failed to write instance config: {0}")]
    WriteConfig(#[source] std::io::Error),
}

impl Instance {
    pub fn create(cwd: &Path, name: String, install_id: Option<String>) -> Self {
        let id = filenamify(&name);
        Self {
            path: cwd.join(DIR_NAME).join(&id),
            id,
            name,
            install_id,
        }
    }

    pub fn save(&self) -> Result<(), InstanceError> {
        std::fs::create_dir_all(&self.path).map_err(InstanceError::CreateDir)?;

        let config_path = self.path.join(CONFIG_NAME);
        let json = json5::to_string(self).map_err(InstanceError::Serialize)?;
        std::fs::write(&config_path, json).map_err(InstanceError::WriteConfig)?;
        Ok(())
    }

    pub fn load_from_dir(cwd: &Path, dir_name: &OsStr) -> Result<Instance, InstanceError> {
        let dir_path = cwd.join(DIR_NAME).join(dir_name);

        // must be a directory...
        if !dir_path.is_dir() {
            return Err(InstanceError::NotDir());
        }
        // with an UTF name
        let Some(dir_name_str) = dir_name.to_str() else {
            return Err(InstanceError::BadDirName(dir_name.to_os_string()));
        };

        let id = dir_name_str.to_string();

        // parse config
        let config_path = dir_path.join(CONFIG_NAME);
        if let Ok(json) = std::fs::read_to_string(&config_path) {
            if let Ok(mut inst) = json5::from_str::<Instance>(&json) {
                // attach non serialized data
                inst.id = id;
                inst.path = dir_path;
                return Ok(inst);
            } else {
                log::warn!("failed parsing `{}` -> re-init", id);
            }
        } else {
            log::warn!("missing config `{}` -> re-init", id);
        }

        // fallback: re-create information as best as possible
        //TODO: keep backup of bad file? what if backup already exists?

        let instance = Instance {
            id: id.clone(),
            name: id,
            install_id: None,
            path: dir_path,
        };
        instance.save()?;
        Ok(instance)
    }
}

pub fn load_instances(cwd: &Path) -> Vec<Instance> {
    let instance_dir = cwd.join(DIR_NAME);
    if let Err(e) = std::fs::create_dir_all(&instance_dir) {
        log::warn!("failed to ensure dir: {}", e);
        return Vec::new();
    }

    match instance_dir.read_dir() {
        Ok(read_dir) => {
            let instances: Vec<Instance> = read_dir
                .flatten()
                .filter(|e| e.file_type().map(|ft| ft.is_dir()).unwrap_or(false))
                .map(|e| e.file_name())
                .flat_map(|dir_name| match Instance::load_from_dir(cwd, &dir_name) {
                    Ok(instance) => {
                        log::debug!("loaded `{}`: {}", instance.id, instance.name);
                        Some(instance)
                    }
                    Err(e) => {
                        log::error!("loading `{}` failed: {}", dir_name.display(), e);
                        None
                    }
                })
                .collect();
            log::info!("loaded {} instances", instances.len());
            instances
        }
        Err(e) => {
            log::error!("failed reading dir: {}", e);
            Vec::new()
        }
    }
}
