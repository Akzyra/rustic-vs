use crate::vs::{Progress, UnpackError};
use inno::Inno;
use log;
use std::fs::File;
use std::path::Path;
use std::time::Instant;
use std::{assert_matches, fs};

pub fn unpack_inno<F>(
    inno_path: &Path,
    out_path: &Path,
    mut on_progress: F,
) -> Result<(), UnpackError>
where
    F: FnMut(Progress),
{
    let start_time = Instant::now();
    let inno_file = File::open(inno_path)?;
    let mut inno = Inno::new(inno_file)?;
    let header = inno.header();
    log::info!(
        "unpacking file {}",
        inno_path.file_name().unwrap().display()
    );
    assert_matches!(header.app_name(), Some("Vintage Story"));
    assert_matches!(header.app_publisher(), Some("Anego Systems"));

    on_progress(Progress {
        current_file: "parsing archive...".to_string(),
        ..Default::default()
    });

    let files_total = inno.file_locations().len() as u64;
    let bytes_total = inno
        .file_locations()
        .iter()
        .map(|e| e.uncompressed_size())
        .sum();

    let mut files_written: u64 = 0;
    let mut bytes_written: u64 = 0;

    inno.filtered_files(|ee| ee.file().destination().unwrap().starts_with("{app}"))
        .flatten()
        .for_each(|(ee, bytes)| {
            let f = ee.file();
            let fl = ee.file_location();
            let rel_path_str = f.normalized_destination().expect("normalize failed");

            let path = out_path.join(&rel_path_str);
            fs::create_dir_all(path.parent().expect("missing parent")).expect("create dir failed");
            fs::write(&path, &bytes).expect("write failed");

            let elapsed_secs = start_time.elapsed().as_secs();
            let speed_bps = bytes_written.checked_div(elapsed_secs).unwrap_or(0);

            on_progress(Progress {
                current_file: rel_path_str,
                files_total,
                files_written,
                bytes_total,
                bytes_written,
                elapsed_secs,
                speed_bps,
            });

            bytes_written += fl.file().size();
            files_written += 1;
        });

    let elapsed_secs = start_time.elapsed().as_secs();
    let speed_bps = bytes_written.checked_div(elapsed_secs).unwrap_or(0);

    // we only unpack {app} files, but we do not know how many are skipped
    // to get 100% when done we "fix" it with "files_total: files_written"
    on_progress(Progress {
        current_file: "DONE".to_string(),
        files_total: files_written,
        files_written,
        bytes_total,
        bytes_written,
        elapsed_secs,
        speed_bps,
    });
    log::info!(
        "unpacked {} files, {} bytes, {} sec",
        files_written,
        bytes_written,
        elapsed_secs
    );
    Ok(())
}
