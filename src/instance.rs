use serde::{Deserialize, Serialize};
use std::ffi::{OsStr, OsString};
use std::fs::create_dir_all;
use std::path::PathBuf;
use std::{env, fs};

use crate::mods::{ModInfo, load_mods};

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

#[derive(Debug)]
pub enum InstanceError {
    TomlMissing,
    TomlParseError,
}

impl Instance {
    pub fn new(name: &str) -> Instance {
        Self {
            folder_name: name.into(),
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
    let mut instances = Vec::new();

    let root = env::current_dir().expect("Failed to get CWD");
    let instances_folder = root.join(INSTANCE_FOLDER);
    create_dir_all(&instances_folder).expect("failed to ensure instances folder");

    println!("loading from {}", instances_folder.display());

    for entry in fs::read_dir(instances_folder).expect("Failed to read INSTANCE_FOLDER") {
        match entry {
            Ok(entry) if entry.path().is_dir() => {
                let folder_name = entry.file_name();
                match Instance::load(&folder_name) {
                    Ok(instance) => {
                        instances.push(instance);
                        println!("loaded instance {}", folder_name.display());
                    }
                    Err(e) => {
                        println!("failed instance {}: {:?}", folder_name.display(), e)
                    }
                }
            }
            _ => {}
        }
    }

    instances
}
