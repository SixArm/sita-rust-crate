use std::ffi::OsString;
use walkdir::DirEntry;
use crate::types::Set;

pub fn walkdir_dir_entry_is_in_extension_set(dir_entry: &DirEntry, extension_set: &Set<OsString>) -> bool {
    if let Some(extension) =  dir_entry.path().extension() {
        return extension_set.contains(extension)
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use once_cell::sync::Lazy;
    use crate::types::*;
    use crate::f::walkdir_dir_entry_first_with_expect::*;

    pub static DIR: Lazy<PathBuf> = Lazy::new(||
        crate::testing::TESTS_DIR
        .join("src")
        .join("f")
        .join("walkdir_dir_entry_is_in_extension_set")
    );

    #[test]
    fn test_true() {
        let dir_entry = walkdir_dir_entry_first_with_expect(&*DIR);
        assert_eq!(dir_entry.file_name(), "example.txt");
        let set = set!(OsString::from("txt"));
        assert_eq!(walkdir_dir_entry_is_in_extension_set(&dir_entry, &set), true);
        let set = set!(OsString::from("jpg"));
        assert_eq!(walkdir_dir_entry_is_in_extension_set(&dir_entry, &set), false);
    }

}
