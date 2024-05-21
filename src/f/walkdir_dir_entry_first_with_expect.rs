use std::path::Path;
use walkdir::{DirEntry, WalkDir};

/// Read a directory and always return the first directory entry.
/// This function is intended for prototyping and testing, not production.
/// 
/// Compare dir_entry_first_with_expect which is for std::fs::DirEntry.
/// 
pub fn walkdir_dir_entry_first_with_expect(path: impl AsRef<Path>) -> DirEntry {
    WalkDir::new(path)
    .into_iter()
    .skip(1)
    .next()
    .expect("next")
    .expect("dir_entry")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use once_cell::sync::Lazy;

    pub static DIR: Lazy<PathBuf> = Lazy::new(||
        crate::testing::TESTS_DIR
        .join("src")
        .join("f")
        .join("walkdir_dir_entry_first_with_expect")
    );

    #[test]
    fn test() {
        let dir_entry: DirEntry = walkdir_dir_entry_first_with_expect(&*DIR);
        assert_eq!(dir_entry.file_name().to_string_lossy(), "example.txt");
    }

}
