use std::path::Path;

pub fn get_folder_size(path: &Path) -> Option<String> {
    fs_extra::dir::get_size(path)
        .map(|size| humansize::format_size(size, humansize::BINARY))
        .ok()
}
