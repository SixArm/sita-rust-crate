use std::path::Path;

/// Remove a file if it exists.
///
/// Example:
///
/// ```
/// let file = PathBuf::from("example");
/// remove_file_if_exists(&file);
/// ```
///
#[allow(dead_code)]
pub fn remove_file_if_exists<P: AsRef<Path>>(path: P) -> std::io::Result<()>
{
    if path.as_ref().exists() {
        ::std::fs::remove_file(path)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::std::path::PathBuf;
    use ::lazy_static::lazy_static;

    lazy_static! {
        pub static ref TMP_DIR: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tmp"].iter().collect::<PathBuf>();
    }

    #[test]
    fn test_remove_file_if_exists_x_present() {
        let file = TMP_DIR.join("test_remove_file_if_exists_x_present.tmp");
        ::std::fs::write(&file, "");
        assert!(file.exists());
        let x = remove_file_if_exists(&file);
        assert!(x.is_ok());
        assert!(!file.exists());
    }

    #[test]
    fn test_remove_file_if_exists_x_absent() {
        let file = TMP_DIR.join("test_remove_file_if_exists_x_absent.tmp");
        assert!(!file.exists());
        let x = remove_file_if_exists(&file);
        assert!(x.is_ok());
        assert!(!file.exists());
    }

}
