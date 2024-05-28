use std::path::PathBuf;

/// Vet input file path buffer exists.
///
/// Example:
///
/// ```
/// let input: PathBuf = PathBuf::from("example.md");
/// vet_input_file_path_buf_exists(&input);
/// ```
///
#[allow(dead_code)]
pub fn vet_input_file_path_buf_exists(input: &PathBuf) -> Result<(), impl std::error::Error> {
    match input.exists() {
        true => Ok(()),
        false => Err(Error::MustExist(input.to_owned()))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("input file must exist; input: {0:?}")]
    MustExist(PathBuf),
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
        .join("vet_input_file_path_buf_exists")
    );

    #[test]
    fn test_vet_input_file_path_buf_exists_x_ok() {
        let x = vet_input_file_path_buf_exists(&DIR.join("example.txt"));
        x.unwrap();
    }

    #[test]
    fn test_vet_input_file_path_buf_exists_x_err() {
        let x = vet_input_file_path_buf_exists(&DIR.join("missing.txt"));
        assert!(x.is_err());
    }

}
