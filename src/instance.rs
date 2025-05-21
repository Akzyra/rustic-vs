use serde::{Deserialize, Serialize};
use std::ffi::{OsStr, OsString};
use std::fs::create_dir_all;
use std::path::PathBuf;
use std::{env, fs};

const INSTANCE_FOLDER: &str = "instances";
const INSTANCE_TOML: &str = "instance.toml";

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Instance {
    #[serde(skip)]
    pub folder_name: OsString,

    pub name: String,
    pub icon: Option<String>,
    pub game_exe_path: Option<PathBuf>,
}

#[derive(Debug)]
pub enum Error {
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
        }
    }

    pub fn load(folder_name: &OsStr) -> Result<Instance, Error> {
        let root = env::current_dir().expect("Failed to get CWD");
        let toml_path = root
            .join(INSTANCE_FOLDER)
            .join(folder_name)
            .join(INSTANCE_TOML);

        toml_path.try_exists().map_err(|_| Error::TomlMissing)?;
        let toml_data = std::fs::read_to_string(toml_path).map_err(|_| Error::TomlParseError)?;

        let mut instance: Instance =
            toml::from_str(&toml_data).map_err(|_| Error::TomlParseError)?;
        instance.folder_name = folder_name.to_os_string();

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

    println!("loading from {}", instances_folder.display());

    for entry in fs::read_dir(instances_folder).expect("Failed to read INSTANCE_FOLDER") {
        match entry {
            Ok(entry) if entry.path().is_dir() => {
                let folder_name = entry.file_name();
                match Instance::load(&folder_name) {
                    Ok(instance) => {
                        instances.push(instance);
                        println!("loaded {}", folder_name.display());
                    }
                    Err(e) => {
                        println!("failed loading {}: {:?}", folder_name.display(), e)
                    }
                }
            }
            _ => {}
        }
    }

    instances
}
