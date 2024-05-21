use std::path::PathBuf;
use crate::errors::*;
use crate::types::*;
use crate::f::from_input_dir_and_output_dir_into_map::*;

/// Convert from an input pathable string and output pathable string into
/// an ordered map of input file path to output file path.
///
/// Example:
//
/// ```rust
/// use crate::types::Map;
/// let input: PathableString = "input/";
/// let output: PathableString = "output/";
/// let map: Map<PathBuf, PathBuf> = from_input_pathable_string_and_output_pathable_string_into_map(input, output);
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
pub fn from_input_pathable_string_and_output_pathable_string_into_map(input: &PathableString, output: &PathableString) -> Result<Map<PathBuf, PathBuf>> {
    trace!("from_input_pathable_string_and_output_pathable_string_into_map input: {}, output: {} ", input, output);
    let input_pathbuf = PathBuf::from(input);
    let output_pathbuf = PathBuf::from(output);
    let mut map = Map::new();
    if input_pathbuf.is_file() && output_pathbuf.is_file() {
        map.insert(input_pathbuf, output_pathbuf);
        return Ok(map);
    }
    if input_pathbuf.is_dir() && output_pathbuf.is_dir() {
        return from_input_dir_and_output_dir_into_map(input_pathbuf, output_pathbuf)
    }
    return Ok(map)
}

#[cfg(test)]
mod tests {
    use super::*;
    use once_cell::sync::Lazy;

    pub static DIR: Lazy<PathBuf> = Lazy::new(||
        crate::testing::TESTS_DIR
        .join("src")
        .join("f")
        .join("from_input_pathable_string_and_output_pathable_string_into_map")
    );

    #[test]
    fn test_with_files() {
        let input_pathable_string = DIR.join("input/a.md").to_string_lossy().into_owned();
        let output_pathable_string = DIR.join("output/a.html").to_string_lossy().into_owned();
        let result = from_input_pathable_string_and_output_pathable_string_into_map(&input_pathable_string, &output_pathable_string);
        assert!(result.is_ok());
        let actual: Map<PathBuf, PathBuf> = result.unwrap();
        let mut expect: Map<PathBuf, PathBuf> = Map::new();
        expect.insert(PathBuf::from("input/a.md"), PathBuf::from("output/a.html"));
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_with_directories() {
        let input_pathable_string = DIR.join("input").to_string_lossy().into_owned();
        let output_pathable_string = DIR.join("output").to_string_lossy().into_owned();
        let result = from_input_pathable_string_and_output_pathable_string_into_map(&input_pathable_string, &output_pathable_string);
        assert!(result.is_ok());
        let actual: Map<PathBuf, PathBuf> = result.unwrap();
        let mut expect: Map<PathBuf, PathBuf> = Map::new();
        expect.insert(PathBuf::from("input/a.md"), PathBuf::from("output/a.html"));
        expect.insert(PathBuf::from("input/b.md"), PathBuf::from("output/b.html"));
        expect.insert(PathBuf::from("input/c.md"), PathBuf::from("output/c.html"));
        assert_eq!(actual, expect);
    }

}
