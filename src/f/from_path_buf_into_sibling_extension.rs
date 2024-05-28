use std::path::PathBuf;
use std::ffi::OsStr;

/// Create a sibling path buffer with a custom extension.
///
/// Example:
///
/// ```
/// let input: PathBuf = PathBuf::from("x.alfa");
/// let output: from_path_buf_into_sibling_extension(&input, "bravo").unwrap();
/// assert_eq!(output, PathBuf::from("x.bravo"));
/// ```
///
#[allow(dead_code)]
pub fn from_path_buf_into_sibling_extension<P: Into<PathBuf>, E: AsRef<OsStr> + Sized>(path_buf: P, extension: E) -> PathBuf {
    trace!("from_path_buf_into_sibling_extension path_buf");
    let mut sibling = PathBuf::from(path_buf.into());
    sibling.set_extension(extension);
    sibling
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_path_buf_into_sibling_extension() {
        let input = PathBuf::from("example.alfa");
        let extension = "bravo";
        let output = from_path_buf_into_sibling_extension(&input, &extension);
        assert_eq!(output, PathBuf::from("example.bravo"));
    }

}
