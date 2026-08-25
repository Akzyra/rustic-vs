use filenamify::filenamify;

const INSTALL_FOLDER: &str = "installs";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Install {
    pub id: String,
    pub name: String,
    pub version: Option<String>,
}

impl Install {
    pub fn new(name: String) -> Self {
        let id = filenamify(&name);
        Self {
            id,
            name,
            version: None,
        }
    }
}
