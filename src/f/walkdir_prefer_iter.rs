use std::path::Path;
use walkdir::{DirEntry, WalkDir};
use crate::f::walkdir_dir_entry_is_visible::*;

/// Walk a directory using our preferred settings:
///
/// * When an unreadble entry happens, then call warn!(...).
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
    trace!("{} ➡ walkdir_prefer_iter ➡ dir: {:?}", file!(), dir.as_ref());
    WalkDir::new(dir)
    .into_iter()
    .filter_map(|result|
        match result {
            Ok(dir_entry) => {
                Some(dir_entry)
            },
            Err(error) => {
                error!("{} ➡ walkdir_prefer_iter -> Err ➡ {:?}", file!(), error);
                None
            }
        }
    )
    .filter(|dir_entry|
        walkdir_dir_entry_is_visible(dir_entry)
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
