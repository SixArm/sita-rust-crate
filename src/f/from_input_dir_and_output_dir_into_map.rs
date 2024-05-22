use std::path::{Path, PathBuf};
use std::ffi::OsString;
use once_cell::sync::Lazy;
use crate::errors::*;
use crate::types::{map::*, set::*};
use crate::f::walkdir_dir_entry_is_in_extension_set::*;
use crate::f::walkdir_dir_entry_is_visible::*;

#[allow(dead_code)]
pub static EXTENSION_SET: Lazy<Set<OsString>> = Lazy::new(||
    set!(
        OsString::from("md"),
        OsString::from("markdown")
    )
);

/// Convert from an input directory path and output directory path
/// into an ordered map of input file path to output file path.
///
/// Example:
//
/// ```rust
/// use crate::types::Map;
/// let input = PathBuf::from("input/");
/// let output = PathBuf::from("output/");
/// let map: Map<PathBuf, PathBuf> = from_input_dir_and_output_dir_into_map(input, output);
/// //=> {
///     "input/1/a.md" => "output/1/a.md",
///     "input/1/b.md" => "output/1/b.html",
///     "input/2/c.md" => "output/2/c.html",
///     "input/2/d.md" => "output/2/d.html",
/// }
/// ```
#[allow(dead_code)]
pub fn from_input_dir_and_output_dir_into_map(input_dir: impl AsRef<Path>, output_dir: impl AsRef<Path>) -> Result<Map<PathBuf, PathBuf>> {
    let input_dir = input_dir.as_ref();
    let output_dir = output_dir.as_ref();
    trace!("from_input_dir_and_output_dir_into_map input_dir: {:?}, output_dir: {:?} ", input_dir, output_dir);
    if !input_dir.is_dir() { return Err("input_dir.is_dir()".into()); }
    if !output_dir.is_dir() { return Err("output_dir.is_dir()".into()); }
    let mut map: Map<PathBuf, PathBuf> = Map::new();
    use walkdir::WalkDir;
    WalkDir::new(&input_dir)
    .follow_links(true)
    .into_iter()
    // Reject result error
    .filter_map(|result|
        match result {
            Ok(dir_entry) => {
                Some(dir_entry)
            },
            Err(error) => {
                error!("from_input_dir_and_output_dir_into_map -> match result -> error: {}", &error);
                None
            }
        }
    )
    .filter(|dir_entry| 
        walkdir_dir_entry_is_visible(&dir_entry)
    )
    .for_each(|dir_entry| {
        let file_name = dir_entry.file_name().to_string_lossy();
        let file_type = dir_entry.file_type();
        let depth = dir_entry.depth();
        let sub: PathBuf = dir_entry.path().iter().skip(depth).collect::<PathBuf>();
        trace!("from_input_dir_and_output_dir_into_map -> depth: {}, sub: {:?}, file_name: {:?}, file_type: {:?}", depth, sub, file_name, file_type);
        if file_type.is_dir() {
            let i = input_dir.join(&sub);
            let o = output_dir.join(&sub);
            trace!("from_input_dir_and_output_dir_into_map -> is_dir -> i: {:?}, o: {:?} ", i, o);
            // TODO mkdir
        } else 
        if file_type.is_file() {
            if walkdir_dir_entry_is_in_extension_set(&dir_entry, &EXTENSION_SET) {
                let i = input_dir.join(&sub);
                let o = output_dir.join(&sub);
                trace!("from_input_dir_and_output_dir_into_map -> is_file -> i: {:?}, o: {:?} ", i, o);
                map.insert(i, o);
            }
        } else {
            warn!("from_input_dir_and_output_dir_into_map -> file type is unexpected => file_name: {:?}, file_type: {:?}", file_name, file_type);
        };
    });
    Ok(map)
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
    fn test() {
        let input_dir = DIR.join("input");
        let output_dir  = DIR.join("output");
        let actual = from_input_dir_and_output_dir_into_map(&input_dir, &output_dir).expect("result");
        let mut expect: Map<PathBuf, PathBuf> = Map::new();
        expect.insert(PathBuf::from("input/1/a.md"), PathBuf::from("output/1/a.html"));
        expect.insert(PathBuf::from("input/1/b.md"), PathBuf::from("output/1/b.html"));
        expect.insert(PathBuf::from("input/2/c.md"), PathBuf::from("output/2/c.html"));
        expect.insert(PathBuf::from("input/2/d.md"), PathBuf::from("output/2/d.html"));
        assert_eq!(actual, expect);
    }

}
