use std::io;
use std::path::Path;
use thiserror::Error;

pub mod api;

pub mod inno;
pub mod tar;

#[derive(Error, Debug)]
pub enum UnpackError {
    #[error("file operation failed")]
    Io(#[from] io::Error),

    #[error("inno unpack failed")]
    Inno(#[from] ::inno::error::InnoError),

    #[error("cannot handle {0}")]
    UnsupportedFile(String),
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Progress {
    pub current_file: String,
    pub files_total: u64,
    pub files_written: u64,
    pub bytes_total: u64,
    pub bytes_written: u64,
    pub elapsed_secs: u64,
    pub speed_bps: u64,
}

pub fn unpack_vs<F>(archive_path: &Path, out_path: &Path, on_progress: F) -> Result<(), UnpackError>
where
    F: FnMut(Progress),
{
    let Some(file_name) = archive_path.file_name().and_then(|s| s.to_str()) else {
        return Err(UnpackError::UnsupportedFile(format!(
            "filename {:?}",
            archive_path.file_name()
        )));
    };

    if file_name.ends_with(".exe") {
        inno::unpack_inno(archive_path, out_path, on_progress)
    } else if file_name.ends_with(".tar.gz") {
        tar::unpack_tar(archive_path, out_path, on_progress)
    } else {
        Err(UnpackError::UnsupportedFile(format!(
            "extension {:?}",
            file_name
        )))
    }
}
