use std::path::{PathBuf};
use crate::errors::*;

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
pub fn vet_input_file_path_buf_exists(input: &PathBuf) -> Result<()> {
    if !input.exists() {
        bail!("input file must exist. path: {:?}", input)
    }
    Ok(())
}

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
pub fn vet_input_file_path_buf_metadata(input: &PathBuf) -> Result<()> {
    let metadata = ::std::fs::metadata(input)
    .chain_err(|| format!("input file must have metadata. path: {:?}", input))?;
    if !metadata.is_file() {
        bail!("input file must be a normal file. path: {:?}", input);
    }
    Ok(())
}



#[cfg(test)]
mod tests {
    use super::*;
    use ::std::path::PathBuf;

    #[test]
    fn test_vet_input_file_path_buf_exists_x_ok() {
        let input_file_path_buf = crate::test::TESTS_DIR.join("function").join("vet_input_file_path_buf_exists").join("example.txt");
        let x = vet_input_file_path_buf_exists(&input_file_path_buf);
        assert!(x.is_ok());
    }

    #[test]
    fn test_vet_input_file_path_buf_exists_x_err() {
        let input_file_path_buf = crate::test::TESTS_DIR.join("function").join("vet_input_file_path_buf_exists").join("missing");
        let x = vet_input_file_path_buf_exists(&input_file_path_buf);
        assert!(x.is_err());
    }

    #[test]
    fn test_vet_input_file_path_buf_metadata_x_ok() {
        let input_file_path_buf = crate::test::TESTS_DIR.join("function").join("vet_input_file_path_buf_metadata").join("example.txt");
        let x = vet_input_file_path_buf_metadata(&input_file_path_buf);
        assert!(x.is_ok());
    }

    #[test]
    fn test_vet_input_file_path_buf_metadata_x_err() {
        let input_file_path_buf = crate::test::TESTS_DIR.join("function").join("vet_input_file_path_buf_metadata").join("missing");
        let x = vet_input_file_path_buf_metadata(&input_file_path_buf);
        assert!(x.is_err());
    }

}
