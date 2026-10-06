use crate::vs::{Progress, UnpackError};
use flate2::read::GzDecoder;
use log;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::time::Instant;
use tar::Archive;

pub fn unpack_tar<F>(
    tar_gz_path: &Path,
    out_path: &Path,
    mut on_progress: F,
) -> Result<(), UnpackError>
where
    F: FnMut(Progress),
{
    // TAR can only be streamed, so we read twice...
    // first pass: collect stats
    let tar_gz_file = File::open(tar_gz_path)?;
    let tar = GzDecoder::new(tar_gz_file);
    let mut archive = Archive::new(tar);
    log::info!(
        "unpacking file {}",
        tar_gz_path.file_name().unwrap().display()
    );

    on_progress(Progress {
        current_file: "parsing archive...".to_string(),
        ..Default::default()
    });

    let start_time = Instant::now();
    let mut files_total: u64 = 0;
    let mut files_written: u64 = 0;
    let mut bytes_total: u64 = 0;
    let mut bytes_written: u64 = 0;

    for entry in archive.entries()? {
        let entry = entry?;
        if entry.header().entry_type().is_file() {
            files_total += 1;
            bytes_total += entry.header().size().unwrap_or(0);
        }
    }

    let elapsed_secs = start_time.elapsed().as_secs_f64();
    log::debug!("tar peek time: {}", elapsed_secs);

    // seconds pass: actually unpack
    let tar_gz_file = File::open(tar_gz_path)?;
    let tar = GzDecoder::new(tar_gz_file);
    let mut archive = Archive::new(tar);

    fs::create_dir_all(out_path)?;
    for entry in archive.entries()? {
        let mut entry = entry?;

        // remove "vintagestory" folder from unpack path
        let rel_path = entry.path()?.components().skip(1).collect::<PathBuf>();
        if rel_path.is_empty() {
            continue;
        }

        let path = out_path.join(&rel_path);
        fs::create_dir_all(path.parent().expect("missing parent"))?;
        entry.unpack(path)?;

        let elapsed_secs = start_time.elapsed().as_secs();
        let speed_bps = bytes_written.checked_div(elapsed_secs).unwrap_or(0);

        on_progress(Progress {
            current_file: rel_path.display().to_string(),
            files_total,
            files_written,
            bytes_total,
            bytes_written,
            elapsed_secs,
            speed_bps,
        });

        bytes_written += entry.header().size().unwrap_or(0);
        files_written += 1;
    }

    let elapsed_secs = start_time.elapsed().as_secs();
    let speed_bps = bytes_written.checked_div(elapsed_secs).unwrap_or(0);

    on_progress(Progress {
        current_file: "DONE".to_string(),
        files_total,
        files_written,
        bytes_total,
        bytes_written,
        elapsed_secs,
        speed_bps,
    });
    log::info!("unpacked {} files, {} bytes", files_written, bytes_written,);
    Ok(())
}
