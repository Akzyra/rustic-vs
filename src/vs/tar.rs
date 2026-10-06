use crate::vs::{Progress, UnpackError};
use std::path::Path;

pub fn extract_tar<F>(
    inno_path: &Path,
    out_path: &Path,
    mut on_progress: F,
) -> Result<(), UnpackError>
where
    F: FnMut(Progress),
{
    panic!("not implemented yet");
}
