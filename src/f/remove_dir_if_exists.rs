use std::path::Path;

/// Remove a dir if it exists.
///
/// Example:
///
/// ```
/// let dir = PathBuf::from("example");
/// remove_dir_if_exists(&file);
/// ```
///
#[allow(dead_code)]
pub fn remove_dir_if_exists<P: AsRef<Path>>(path: P) -> std::io::Result<()>
{
    if path.as_ref().exists() {
        std::fs::remove_dir(path)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assertables::*;
    use std::path::PathBuf;
    use once_cell::sync::Lazy;

    pub static TMP_DIR: Lazy<PathBuf> = Lazy::new(||
        [env!("CARGO_MANIFEST_DIR"), "tmp"].iter().collect::<PathBuf>()
    );

    #[test]
    fn test_remove_dir_if_exists_x_present() {
        let dir = TMP_DIR.join("test_remove_dir_if_exists_x_present");
        let result = std::fs::create_dir(&dir);
        assert_ok!(result);
        assert!(dir.exists());
        let result = remove_dir_if_exists(&dir);
        assert_ok!(result);
        assert!(!dir.exists());
    }

    #[test]
    fn test_remove_dir_if_exists_x_absent() {
        let dir = TMP_DIR.join("test_remove_dir_if_exists_x_absent");
        assert!(!dir.exists());
        let result = remove_dir_if_exists(&dir);
        assert_ok!(result);
        assert!(!dir.exists());
    }

}