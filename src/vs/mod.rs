use crate::vs::api::DownloadInfo;
use humansize::{BINARY, FormatSizeOptions, Kilo, format_size};
use std::fs::File;
use std::io::{BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::LazyLock;
use std::time::Instant;
use std::{env, fs, io};
use thiserror::Error;
use ureq::http::header::USER_AGENT;

pub mod api;

pub mod inno;
pub mod tar;

// hopefully aligned with what the API uses
static VS_SIZE_FMT: LazyLock<FormatSizeOptions> = LazyLock::new(|| {
    FormatSizeOptions::from(BINARY)
        .units(Kilo::Decimal)
        .decimal_places(1)
});

#[derive(Error, Debug)]
pub enum UnpackError {
    #[error("file operation failed")]
    Io(#[from] io::Error),

    #[error("inno unpack failed")]
    Inno(#[from] ::inno::error::InnoError),

    #[error("cannot handle {0}")]
    UnsupportedFile(String),
}

#[derive(Error, Debug)]
pub enum DownloadError {
    #[error("file operation failed")]
    Io(#[from] io::Error),

    #[error("request failed")]
    Request(#[from] ureq::Error),
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

pub fn download_vs<F>(
    download_info: &DownloadInfo,
    mut on_progress: F,
) -> Result<PathBuf, DownloadError>
where
    F: FnMut(Progress),
{
    let start_time = Instant::now();
    let temp_dir = env::temp_dir().join(crate::NAME);
    if !temp_dir.exists() {
        fs::create_dir(&temp_dir)?;
    }

    // check if file exists and matches in size and hash
    let temp_file = temp_dir.join(&download_info.filename);
    if temp_file.exists() {
        let size = temp_file.metadata().map(|m| m.len()).unwrap_or_default();
        let size_str = format_size(size, *VS_SIZE_FMT);

        if size_str == download_info.filesize {
            on_progress(Progress {
                current_file: format!("checking existing {}", download_info.filename),
                files_total: 1,
                files_written: 0,
                bytes_total: size / 2, // fake progress for hashing
                bytes_written: size,
                ..Default::default()
            });

            let digest = md5::compute(fs::read(&temp_file)?);
            let md5sum = format!("{:x}", digest);

            if md5sum == download_info.md5 {
                log::info!("using cache for {}", download_info.filename);

                on_progress(Progress {
                    current_file: format!("DONE {}", download_info.filename),
                    files_total: 1,
                    files_written: 1,
                    bytes_total: size,
                    bytes_written: size,
                    ..Default::default()
                });

                return Ok(temp_file);
            }
        }
    }

    log::info!(
        "downloading {} to {}",
        download_info.filename,
        temp_dir.display()
    );

    let mut resp = ureq::get(&download_info.urls.cdn)
        .header(USER_AGENT, crate::USER_AGENT)
        .call()?;

    let bytes_total = resp.body().content_length().unwrap_or_default(); // Option<u64>

    on_progress(Progress {
        current_file: download_info.filename.clone(),
        files_total: 1,
        files_written: 0,
        bytes_total,
        bytes_written: 0,
        ..Default::default()
    });

    let mut reader = resp.body_mut().as_reader();
    let mut writer = BufWriter::new(File::create(&temp_file)?);
    let mut buf = [0u8; 128 * 1024];
    let mut bytes_written = 0u64;

    loop {
        let n = reader.read(&mut buf)?;
        if n == 0 {
            break;
        }
        writer.write_all(&buf[..n])?;

        bytes_written += n as u64;
        on_progress(Progress {
            current_file: download_info.filename.clone(),
            files_total: 1,
            files_written: 0,
            bytes_total,
            bytes_written,
            ..Default::default()
        });
    }

    on_progress(Progress {
        current_file: format!("DONE {}", download_info.filename),
        files_total: 1,
        files_written: 1,
        bytes_total,
        bytes_written,
        ..Default::default()
    });

    writer.flush()?;

    let elapsed_secs = start_time.elapsed().as_secs();
    log::info!("downloaded {} bytes, {} sec", bytes_written, elapsed_secs);
    Ok(temp_file)
}
