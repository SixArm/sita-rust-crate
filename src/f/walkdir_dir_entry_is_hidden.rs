use walkdir::DirEntry;

/// Is the directory entry a visible path i.e. doesn't start with a period?
///
/// Compare walkdir_dir_entry_is_hidden; these functions are not exact
/// opposites, because they skip file names that can't convert to UTF-8.
/// 
/// Compare the std::fs::DirEntry equivalents:
///   * dir_entry_is_hidden
///   * dir_entry_is_visible
/// 
pub fn walkdir_dir_entry_is_hidden(dir_entry: &DirEntry) -> bool {
    if let Some(file_name) = dir_entry.file_name().to_str() {
        return file_name.starts_with(".")
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use once_cell::sync::Lazy;
    use crate::f::walkdir_dir_entry_first_with_expect::*;

    pub static DIR: Lazy<PathBuf> = Lazy::new(||
        crate::testing::TESTS_DIR
        .join("src")
        .join("f")
        .join("walkdir_dir_entry_is_hidden")
    );

    #[test]
    fn test_hidden() {
        let dir_entry: DirEntry = walkdir_dir_entry_first_with_expect(DIR.join("contains-hidden"));
        assert_eq!(dir_entry.file_name().to_string_lossy(), ".hidden.txt");
        assert_eq!(walkdir_dir_entry_is_hidden(&dir_entry), true);
    }

    #[test]
    fn test_visible() {
        let dir_entry: DirEntry = walkdir_dir_entry_first_with_expect(DIR.join("contains-visible"));
        assert_eq!(dir_entry.file_name().to_string_lossy(), "visible.txt");
        assert_eq!(walkdir_dir_entry_is_hidden(&dir_entry), false);
    }

}
