use log::debug;
use serde::Deserialize;
use std::ffi::OsString;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::result::Result;
use zip::ZipArchive;

const MODINFO_JSON: &str = "modinfo.json";

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone, Default)]
pub struct ModInfo {
    #[serde(skip)]
    pub zip_name: OsString,

    #[serde(alias = "modid", alias = "ModID", alias = "modId", default)]
    pub mod_id: String,

    #[serde(alias = "Name")]
    pub name: String,

    #[serde(alias = "Description", default)]
    pub description: String,

    #[serde(alias = "Version")]
    pub version: String,
}

#[derive(Debug)]
pub enum ModError {
    ZipMissing,
    ZipParseError,
    InfoParseError(#[expect(unused)] json5::Error),
}
impl ModInfo {
    fn from_zip(zip_path: &PathBuf) -> Result<ModInfo, ModError> {
        let Some(filename) = zip_path.file_name() else {
            return Err(ModError::ZipMissing);
        };
        let file = File::open(zip_path).map_err(|_| ModError::ZipParseError)?;
        let mut zip = ZipArchive::new(file).unwrap();

        match zip.by_name(MODINFO_JSON) {
            Ok(mut json_file) => {
                let mut json = String::new();
                json_file
                    .read_to_string(&mut json)
                    .map_err(|_| ModError::ZipParseError)?;
                let mut mod_info: ModInfo =
                    json5::from_str(&json).map_err(ModError::InfoParseError)?;

                mod_info.zip_name = filename.to_os_string();

                Ok(mod_info)
            }
            Err(..) => Ok(ModInfo {
                zip_name: filename.to_os_string(),
                ..ModInfo::default()
            }),
        }
    }
}

pub fn load_mods(folder_path: &PathBuf) -> Vec<ModInfo> {
    if !folder_path.exists() {
        return Vec::new();
    }

    let mut mods = Vec::new();

    for entry in fs::read_dir(folder_path)
        .expect("failed to read mods folder")
        .flatten()
    {
        let zip_name = entry.file_name();
        match ModInfo::from_zip(&entry.path()) {
            Ok(mod_info) => {
                debug!("parsed {} ", zip_name.display());
                mods.push(mod_info);
            }
            Err(e) => {
                debug!("failed parsing {}: {:?}", zip_name.display(), e)
            }
        }
    }

    mods
}
