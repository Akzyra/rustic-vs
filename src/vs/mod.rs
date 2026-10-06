use std::io;
use thiserror::Error;

pub mod api;

#[cfg(target_os = "windows")]
pub mod inno;
pub mod tar;

#[derive(Error, Debug)]
pub enum UnpackError {
    #[error("file operation failed")]
    Io(#[from] io::Error),

    #[cfg(target_os = "windows")]
    #[error("inno unpack failed")]
    Inno(#[from] ::inno::error::InnoError),
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

#[cfg(target_os = "windows")]
pub(crate) use self::inno::extract_inno as extract_vs;

#[cfg(not(target_os = "windows"))]
use self::tar::extract_tar as extract_vs;
