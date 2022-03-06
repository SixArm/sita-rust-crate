use std::path::PathBuf;
use std::ffi::OsStr;

/// Create a sibling path buffer with a custom extension.
///
/// Example:
///
/// ```
/// let input: PathBuf = PathBuf::from("x.alpha");
/// let output: from_path_buf_into_sibling(&input, "bravo").unwrap();
/// assert_eq!(output, PathBuf::from("x.bravo"));
/// ```
///
pub fn from_path_buf_into_sibling<P: Into<PathBuf>, E: AsRef<OsStr> + Sized>(path_buf: P, extension: E) -> PathBuf {
    trace!("from_path_buf_into_sibling path_buf");
    let mut sibling = PathBuf::from(path_buf.into());
    sibling.set_extension(extension);
    sibling
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_path_buf_into_sibling() {
        let input = PathBuf::from("example.alpha");
        let extension = "bravo";
        let output = from_path_buf_into_sibling(&input, &extension);
        assert_eq!(output, PathBuf::from("example.bravo"));
    }

}
