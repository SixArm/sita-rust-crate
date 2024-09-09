use std::path::Path;
use walkdir::{DirEntry, WalkDir};

/// Walk a directory using our preferred settings:
///
/// * When an unreadable entry happens, then call warn!(...).
///
/// * Select files; reject directories.
/// 
/// * Skip hidden entries i.e. each entry that starts with a period.
///
/// Example:
//
/// ```rust
/// use crate::types::Map;
/// let dir = PathBuf::from("dir");
/// let map: Map<PathBuf, PathBuf> = walkdir_prefer_iter(dir);
/// //=> {
///     "dir/1.txt",
///     "dir/2.txt",
///     "dir/3.txt",
/// }
/// ```
#[allow(dead_code)]
pub fn walkdir_prefer_iter(dir: impl AsRef<Path>) -> impl Iterator<Item = DirEntry> {
    trace!("walkdir_prefer_iter ➡ dir: {:?}", dir.as_ref());
    WalkDir::new(dir)
    .into_iter()
    .filter_map(|result|
        match result {
            Ok(dir_entry) => {
                Some(dir_entry)
            },
            Err(error) => {
                error!("walkdir_prefer_iter -> Err ➡ {:?}", error);
                None
            }
        }
    )
    .filter(|dir_entry|
        dir_entry.file_type().is_file() &&
        !dir_entry.file_name().to_string_lossy().starts_with(".")
    )
    .into_iter()
}

#[cfg(test)]
mod tests {
    use super::*;
    use once_cell::sync::Lazy;
    use std::path::PathBuf;

    pub static DIR: Lazy<PathBuf> = Lazy::new(||
        crate::testing::TESTS_DIR
        .join("src")
        .join("f")
        .join("walkdir_prefer_iter")
    );

    #[test]
    fn test() {
        let mut i = walkdir_prefer_iter(&*DIR);
        assert_eq!(i.next().expect("next").file_name().to_string_lossy(), "b.txt");
        assert_eq!(i.next().expect("next").file_name().to_string_lossy(), "d.txt");
        assert!(i.next().is_none());
    }

}
