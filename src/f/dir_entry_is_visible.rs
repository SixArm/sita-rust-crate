use std::fs::DirEntry;

/// Is the directory entry a visible path i.e. doesn't start with a period?
///
/// Compare dir_entry_is_hidden; these functions are not exact opposites,
/// because of they skip file names that can't convert to UTF-8.
///
/// Compare the walkdir equivalents:
///   * walkdir_dir_entry_is_hidden
///   * walkdir_dir_entry_is_visible
///
pub fn dir_entry_is_visible(dir_entry: &DirEntry) -> bool {
    if let Some(file_name) = dir_entry.file_name().to_str() {
        return !file_name.starts_with(".")
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use once_cell::sync::Lazy;
    use crate::f::dir_entry_first_with_expect::*;

    pub static DIR: Lazy<PathBuf> = Lazy::new(||
        crate::testing::TESTS_DIR
        .join("src")
        .join("f")
        .join("dir_entry_is_visible")
    );

    #[test]
    fn test_visible() {
        let dir_entry: DirEntry = dir_entry_first_with_expect(DIR.join("contains-visible"));
        assert_eq!(dir_entry.file_name().to_string_lossy(), "visible.txt");
        assert_eq!(dir_entry_is_visible(&dir_entry), true);
    }

    #[test]
    fn test_hidden() {
        let dir_entry: DirEntry = dir_entry_first_with_expect(DIR.join("contains-hidden"));
        assert_eq!(dir_entry.file_name().to_string_lossy(), ".hidden.txt");
        assert_eq!(dir_entry_is_visible(&dir_entry), false);
    }

}
