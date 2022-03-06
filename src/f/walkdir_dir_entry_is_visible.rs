#[allow(dead_code)] 
pub fn walkdir_dir_entry_is_visible(entry: &::walkdir::DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| !s.starts_with("."))
         .unwrap_or(false)
}
