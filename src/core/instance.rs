use filenamify::filenamify;

const INSTANCE_FOLDER: &str = "instances";
const INSTANCE_TOML: &str = "instance.toml";
const MODS_FOLDER: &str = "Mods";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Instance {
    pub id: String,
    pub name: String,
    pub install_id: Option<String>,
}

impl Instance {
    pub fn new(name: String) -> Self {
        let id = filenamify(&name);
        Self {
            id,
            name,
            install_id: None,
        }
    }
}
