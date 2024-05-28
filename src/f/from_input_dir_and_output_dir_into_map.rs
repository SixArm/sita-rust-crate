use std::path::PathBuf;
// use std::fs::DirEntry;
// use std::path::StripPrefixError;
use crate::types::map::*;
use walkdir::WalkDir;

/// Convert from an input dir and output dir into
/// an ordered map of input file path to output file path.
///
/// Example:
//
/// ```rust
/// use crate::types::Map;
/// let input = PathBuf::from("input"); // any input directory
/// let output = PathBuf::from("output"); // any output directory
/// let map: Map<PathBuf, PathBuf> = from_input_dir_and_output_dir_into_map(input, output);
/// //=> {
///     "input/a.md" => "output/a.md",
///     "input/b.md" => "output/b.md",
///     "input/c.md" => "output/c.md",
/// }
/// ```
///
/// This function deliberately filters out errors.
///
/// For example, this function will silently skip directories that the
/// owner of the running process does not have permission to access.
///
#[allow(dead_code)]
pub fn from_input_dir_and_output_dir_into_map(input_dir: &PathBuf, output_dir: &PathBuf) -> Result<Map<PathBuf, PathBuf>, Error> {
    trace!("{} ➡ from_input_dir_and_output_dir_into_map ➡ input_dir: {:?}, output_dir: {:?} ", file!(), input_dir, output_dir);
    if !input_dir.is_dir() {
        return Err(Error::InputDirMustBeDir { input_dir: input_dir.to_owned() })
    }
    if !output_dir.is_dir() {
        return Err(Error::OutputDirMustBeDir { output_dir: output_dir.to_owned() })
    }
    let mut map: std::collections::BTreeMap<PathBuf, PathBuf> = Map::new();
    for dir_entry in WalkDir::new(&input_dir) {
        match dir_entry {
            Ok(dir_entry) => {
                match dir_entry.path().strip_prefix(&input_dir) {
                    Ok(path) => {
                        map.insert(
                            input_dir.join(path), 
                            output_dir.join(path)
                        );
                    },
                    Err(error) => {
                        return Err(Error::StripPrefixError {
                            input_dir: input_dir.to_owned(),
                            dir_entry: dir_entry.to_owned(),
                            inner: error.to_owned(),
                        });
                    }
                }
            },
            Err(err) => {
                return Err(Error::Walk { 
                    input_dir: input_dir.to_owned(), 
                    output_dir: output_dir.to_owned(), 
                    walkdir_error: err
                });
            }
        }
    }
    Ok(map)
}

#[derive(thiserror::Error, Debug)]
pub enum Error {

    #[error("InputDirMustBeDir ➡ input_dir: {input_dir:?}")]
    InputDirMustBeDir {
        input_dir: PathBuf,
    },

    #[error("OutputDirMustBeDir ➡ output_dir: {output_dir:?}")]
    OutputDirMustBeDir {
        output_dir: PathBuf,
    },

    #[error("Walk ➡ input_dir: {input_dir:?}, output_dir: {output_dir:?}, walkdir_error: {walkdir_error:?}")]
    Walk {
        input_dir: PathBuf,
        output_dir: PathBuf,
        walkdir_error: walkdir::Error,
    },

    #[error("StripPrefixError ➡ input_dir: {input_dir:?}, dir_entry: {dir_entry:?}, inner: {inner:?}")]
    StripPrefixError {
        input_dir: PathBuf,
        dir_entry: walkdir::DirEntry,
        inner: std::path::StripPrefixError,  
    },

}

#[cfg(test)]
mod tests {
    use super::*;
    use once_cell::sync::Lazy;

    pub static DIR: Lazy<PathBuf> = Lazy::new(||
        crate::testing::TESTS_DIR
        .join("src")
        .join("f")
        .join("from_input_dir_and_output_dir_into_map")
    );

    #[test]
    fn test_with_input_file() {
        let input_file = DIR.join("input/a.md");
        let output_dir = DIR.join("output/");
        let result = from_input_dir_and_output_dir_into_map(&input_file, &output_dir);
        assert!(result.is_err());
    }

    #[test]
    fn test_with_output_file() {
        let input_dir = DIR.join("input/");
        let output_file = DIR.join("output/a.md");
        let result = from_input_dir_and_output_dir_into_map(&input_dir, &output_file);
        assert!(result.is_err());
    }

    #[test]
    fn test_with_directories() {
        let input_dir = DIR.join("input");
        let output_dir = DIR.join("output");
        let result = from_input_dir_and_output_dir_into_map(&input_dir, &output_dir);
        let actual: Map<PathBuf, PathBuf> = result.unwrap();
        let mut iter = actual.iter();
        assert_eq!(iter.next().unwrap(), (&input_dir, &output_dir));
        assert_eq!(iter.next().unwrap(), (&input_dir.join("a.md"), &output_dir.join("a.md")));
        assert_eq!(iter.next().unwrap(), (&input_dir.join("b.md"), &output_dir.join("b.md")));
        assert_eq!(iter.next().unwrap(), (&input_dir.join("c.md"), &output_dir.join("c.md")));
    }

}

// cSpell:ignore walkdir