use std::path::PathBuf;
use crate::types::map::*;
use crate::f::from_input_dir_and_output_dir_into_map::*;

/// Convert from an input path buffer and output path buffer into
/// an ordered map of input file path to output file path.
///
/// Example:
//
/// ```rust
/// use crate::types::Map;
/// let input = PathBuf::from("input"); // any input directory
/// let output = PathBuf::from("output"); // any output directory
/// let map: Map<PathBuf, PathBuf> = from_input_path_buf_and_output_path_buf_into_map(input, output);
/// //=> {
///     "input/a.md" => "output/a.md",
///     "input/b.md" => "output/b.html",
///     "input/c.md" => "output/c.html",
/// }
/// ```
///
/// This function deliberately filters out errors.
///
/// For example, this function will silently skip directories that the
/// owner of the running process does not have permission to access.
///
#[allow(dead_code)]
pub fn from_input_path_buf_and_output_path_buf_into_map(input_path_buf: &PathBuf, output_path_buf: &PathBuf) -> Result<Map<PathBuf, PathBuf>, Error> {
    trace!("{} ➡ from_input_path_buf_and_output_path_buf_into_map ➡ input_path_buf: {:?}, output_path_buf: {:?} ", file!(), input_path_buf, output_path_buf);
    let mut map: std::collections::BTreeMap<PathBuf, PathBuf> = Map::new();
    if input_path_buf.is_file() && output_path_buf.is_file() {
        //TODO optimize by doing borrow
        map.insert(input_path_buf.clone(), output_path_buf.clone());
        Ok(map)
    } else
    if input_path_buf.is_dir() && output_path_buf.is_dir() {
        from_input_dir_and_output_dir_into_map(input_path_buf, output_path_buf)
        .map_or_else(
            |err| Err(Error::Wrap(err)),
            |val| Ok(val)
        )
    } else {
        Err(Error::MustBeFilesOrDirectories { input_path_buf: input_path_buf.to_owned(), output_path_buf: output_path_buf.to_owned() })
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {

    #[error("MustBeFilesOrDirectories ➡ input_path_buf: {input_path_buf:?}, output_path_buf: {output_path_buf:?}")]
    MustBeFilesOrDirectories{
        input_path_buf: PathBuf,
        output_path_buf: PathBuf,
    },

    #[error("Wrap ➡ {0}")]
    Wrap(crate::f::from_input_dir_and_output_dir_into_map::Error)

}

#[cfg(test)]
mod tests {
    use super::*;
    use once_cell::sync::Lazy;

    pub static DIR: Lazy<PathBuf> = Lazy::new(||
        crate::testing::TESTS_DIR
        .join("src")
        .join("f")
        .join("from_input_path_buf_and_output_path_buf_into_map")
    );

    #[test]
    fn test_with_files() {
        let input_path_buf = DIR.join("input/a.md");
        let output_path_buf = DIR.join("output/a.html");
        let result = from_input_path_buf_and_output_path_buf_into_map(&input_path_buf, &output_path_buf);
        assert!(result.is_ok());
        let actual: Map<PathBuf, PathBuf> = result.unwrap();
        let mut expect: Map<PathBuf, PathBuf> = Map::new();
        expect.insert(PathBuf::from("input/a.md"), PathBuf::from("output/a.html"));
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_with_directories() {
        let input_path_buf = DIR.join("input");
        let output_path_buf = DIR.join("output");
        let result = from_input_path_buf_and_output_path_buf_into_map(&input_path_buf, &output_path_buf);
        assert!(result.is_ok());
        let actual: Map<PathBuf, PathBuf> = result.unwrap();
        let mut iter = actual.iter();
        assert_eq!(iter.next().unwrap(), (&input_path_buf, &output_path_buf));
        assert_eq!(iter.next().unwrap(), (&input_path_buf.join("a.md"), &output_path_buf.join("a.md")));
        assert_eq!(iter.next().unwrap(), (&input_path_buf.join("b.md"), &output_path_buf.join("b.md")));
        assert_eq!(iter.next().unwrap(), (&input_path_buf.join("c.md"), &output_path_buf.join("c.md")));
    }

}
