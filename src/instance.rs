use crate::mods::{ModInfo, load_mods};
use filenamify::filenamify;
use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use std::ffi::{OsStr, OsString};
use std::fs::create_dir_all;
use std::path::PathBuf;
use std::{env, fs};

const INSTANCE_FOLDER: &str = "instances";
const INSTANCE_TOML: &str = "instance.toml";
const MODS_FOLDER: &str = "Mods";

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Instance {
    #[serde(skip)]
    pub folder_name: OsString,

    #[serde(skip)]
    pub mods: Vec<ModInfo>,

    pub name: String,
    pub icon: Option<String>,
    pub game_exe_path: Option<PathBuf>,
}

impl Instance {
    pub fn folder_name_string(&self) -> String {
        self.folder_name.to_string_lossy().to_string()
    }

    pub fn mods_count(&self) -> usize {
        self.mods.len()
    }
}

#[derive(Debug)]
pub enum InstanceError {
    TomlMissing,
    TomlParseError,
}

impl Instance {
    pub fn new(name: &str) -> Instance {
        Self {
            folder_name: filenamify(name).into(),
            name: name.to_string(),
            icon: None,
            game_exe_path: None,
            mods: Vec::new(),
        }
    }

    pub fn load(folder_name: &OsStr) -> Result<Instance, InstanceError> {
        let root = env::current_dir().expect("Failed to get CWD");
        let instance_path = root.join(INSTANCE_FOLDER).join(folder_name);
        let toml_path = instance_path.join(INSTANCE_TOML);
        let mods_path = instance_path.join(MODS_FOLDER);

        toml_path
            .try_exists()
            .map_err(|_| InstanceError::TomlMissing)?;
        let toml_data = fs::read_to_string(toml_path).map_err(|_| InstanceError::TomlParseError)?;

        let mut instance: Instance =
            toml::from_str(&toml_data).map_err(|_| InstanceError::TomlParseError)?;

        instance.folder_name = folder_name.to_os_string();
        instance.mods = load_mods(&mods_path);

        Ok(instance)
    }

    pub fn save(&self) {
        // TODO: show UI message if errors
        create_dir_all(self.path()).expect("Failed to create instance dir");

        let toml_data = toml::to_string_pretty(self).unwrap();
        fs::write(self.toml_path(), toml_data).unwrap();
    }

    pub fn path(&self) -> PathBuf {
        let root = env::current_dir().expect("Failed to get CWD");

        root.join(INSTANCE_FOLDER).join(&self.folder_name)
    }

    pub fn toml_path(&self) -> PathBuf {
        self.path().join(INSTANCE_TOML)
    }
}

pub fn load_instances() -> Vec<Instance> {
    let root = env::current_dir().expect("Failed to get CWD");
    let instances_folder = root.join(INSTANCE_FOLDER);

    if let Err(e) = create_dir_all(&instances_folder) {
        debug!("failed to ensure instances folder: {}", e);
        return Vec::new();
    }

    let instances: Result<Vec<_>, _> = instances_folder.read_dir().map(|read_dir| {
        read_dir
            .flatten()
            .filter(|entry| entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false))
            .map(|folder_entry| folder_entry.file_name())
            .flat_map(|folder_name| match Instance::load(&folder_name) {
                Ok(instance) => {
                    debug!("loaded {}", folder_name.display());
                    Some(instance)
                }
                Err(e) => {
                    error!("failed loading {}: {:?}", folder_name.display(), e);
                    None
                }
            })
            .collect()
    });

    match instances {
        Ok(instances) => {
            info!("loaded {} instances", instances.len());
            instances
        }
        Err(e) => {
            error!("failed reading instances folder: {}", e);
            Vec::new()
        }
    }
}
