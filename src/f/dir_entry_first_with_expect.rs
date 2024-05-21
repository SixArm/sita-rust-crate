use std::path::Path;
use std::fs::DirEntry;

/// Read a directory and always return the first directory entry.
/// This function is intended for prototyping and testing, not production.
/// 
/// Compare walkdir_dir_entry_first_with_expect which is for walkdir::DirEntry.
///
pub fn dir_entry_first_with_expect(path: impl AsRef<Path>) -> DirEntry {
    std::fs::read_dir(path)
    .expect("read_dir")
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
        .join("dir_entry_first_with_expect")
    );

    #[test]
    fn test() {
        let dir_entry: DirEntry = dir_entry_first_with_expect(&*DIR);
        assert_eq!(dir_entry.file_name().to_string_lossy(), "example.txt");
    }

}
