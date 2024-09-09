use std::path::PathBuf;

/// Vet input file path buffer metadata is a normal file.
///
/// Example:
///
/// ```
/// let input: PathBuf = PathBuf::from("example.md");
/// vet_input_file_path_buf_metadata(&input);
/// ```
///
#[allow(dead_code)]
pub fn vet_input_file_path_buf_metadata(input: &PathBuf) -> Result<(), impl std::error::Error> {
    match std::fs::metadata(input) {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::InputFileMetadataMustBeOk(input.to_owned(), e))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("InputFileMetadataMustBeOk input: {0:?}")]
    InputFileMetadataMustBeOk(PathBuf, std::io::Error),
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
        .join("vet_input_file_path_buf_metadata")
    );

    #[test]
    fn test_vet_input_file_path_buf_metadata_x_ok() {
        let x = vet_input_file_path_buf_metadata(&DIR.join("example.txt"));
        x.unwrap();
    }

    #[test]
    fn test_vet_input_file_path_buf_metadata_x_err() {
        let x = vet_input_file_path_buf_metadata(&DIR.join("missing.txt"));
        assert!(x.is_err());
    }

}
