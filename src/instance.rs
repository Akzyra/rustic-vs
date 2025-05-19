use glob::glob;
use serde::{Deserialize, Serialize};
use std::ffi::OsString;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Instance {
    #[serde(skip)]
    pub folder_name: OsString,

    pub name: String,
    pub icon: Option<String>,
    pub game_exe_path: PathBuf,
}

// #[derive(PartialEq, Eq, Clone, Debug)]
// pub enum Error {
//     ConfigParseError,
// }

pub fn load_instances() -> Vec<Instance> {
    let mut instances = Vec::new();
    for entry in glob("./instances/*/instance.toml").expect("glob failed") {
        match entry {
            Ok(path) => {
                let instance_path = path.parent().expect("instance path empty");
                let folder_name = instance_path.file_name().expect("folder name not absolute");

                let config_data = std::fs::read_to_string(path.clone()).expect("toml read failed");
                let mut instance: Instance =
                    toml::from_str(&config_data).expect("toml parse failed");
                instance.folder_name = folder_name.to_os_string();

                instances.push(instance);
                println!("loaded {}", path.display());
            }
            Err(e) => println!("err {:?}", e),
        }
    }
    instances
}
