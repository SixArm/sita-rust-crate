use std::path::PathBuf;
use glob::PatternError;
use walkdir::WalkDir;

use crate::types::{list::*, pathable::*};
use crate::f::walkdir_dir_entry_is_visible::*;

/// Convert from &PathableString into List<PathBuf>.
///
/// Example:
//
/// ```rust
/// let from: "a/*";
/// let into: List<PathBuf> = from_pathable_string_into_list_path_buf(from);
/// //=> ["a", "a/a1.txt", "a/a2.txt"]
/// ```
///
/// This function deliberately filters out errors.
///
/// For example, this function will silently skip directories that the
/// owner of the running process does not have permission to access.
///
#[allow(dead_code)]
pub fn from_pathable_string_into_list_path_buf(from: &PathableString) -> Result<List<PathBuf>, FromPathableStringIntoListPathBufError> {
    trace!("from_pathable_string_into_list_path_buf from: {:?}", from);
    let list_path_buf: List<PathBuf> = ::glob::glob(&from)
    .map_or_else(
        |err: PatternError| Err(FromPathableStringIntoListPathBufError::PatternError(err)),
        |paths: glob::Paths| Ok(paths)
    )?
    .inspect(|x|
        println!("f1: {:?}", x)
    )
    .inspect(|x|
        match x {
            Ok(x) => trace!("from_pathable_string_into_list_path_buf glob ➡ Ok ➡ from: {:?}, path: {:?}", from, x),
            Err(err) => warn!("from_pathable_string_into_list_path_buf glob ➡ Err ➡ from: {:?}, err: {:?}", from, err),
        }
    )
    .filter_map(|x|
        x.ok()
        //TODO
        // match x {
        //     Ok(path_buf) => path_buf,
        //     Err(err) => bail!(err),
        // }
    )
    .inspect(|x|
        println!("f2: {:?}", x)
    )
    .flat_map(|path_buf|
        WalkDir::new(&path_buf)
        .into_iter()
        .filter_entry(|e|
            walkdir_dir_entry_is_visible(&e)
        )
        .inspect(|x|
            println!("f3: {:?}", x)
        )
        .inspect(|x|
            match x {
                Ok(x) => trace!("from_pathable_string_into_list_path_buf dir entry ok. ␟from: {:?} ␟dir entry: {:?}", from, x),
                Err(err) => warn!("from_pathable_string_into_list_path_buf dir entry err. ␟from: {:?} ␟err: {:?}", from, err),
            }
        )
        .filter_map(|x|
            x.ok()
            //TODO
            // match x {
            //     Ok(dir_entry) => dir_entry,
            //     Err(err) => bail!(err),
            // }
        )
        .map(|x|
            PathBuf::from(x.path())
        )
    )
    .collect::<_>();
    Ok(list_path_buf)
}

#[derive(thiserror::Error, Debug)]
pub enum FromPathableStringIntoListPathBufError {
    #[error("PatternError")]
    PatternError(#[from] PatternError),
}

#[cfg(test)]
mod tests {
    use super::*;
    use once_cell::sync::Lazy;

    pub static DIR: Lazy<PathBuf> = Lazy::new(||
        crate::testing::TESTS_DIR
        .join("src")
        .join("f")
        .join("from_pathable_string_into_list_path_buf")
    );

    #[test]
    fn test_from_pathable_string_into_list_path_buf_x_dir() {
        let dir_as_string = DIR.to_string_lossy();
        let from: PathableString = format!("{}{}", dir_as_string, "/a");
        let result = from_pathable_string_into_list_path_buf(&from);
        let mut actual: List<PathBuf> = result.unwrap();
        actual.sort();
        let expect: List<PathBuf> = list![
            DIR.join("a"),
            DIR.join("a/aa"),
            DIR.join("a/aa/aaa"),
            DIR.join("a/aa/aab"),
            DIR.join("a/ab"),
            DIR.join("a/ab/aba"),
            DIR.join("a/ab/abb")
        ];
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_from_pathable_string_into_list_path_buf_x_glob() {
        let dir_as_string = DIR.to_string_lossy();
        let from: PathableString = format!("{}{}", dir_as_string, "/a*");
        let result = from_pathable_string_into_list_path_buf(&from);
        let mut actual: List<PathBuf> = result.unwrap();
        actual.sort();
        let expect: List<PathBuf> = list![
            DIR.join("a"),
            DIR.join("a/aa"),
            DIR.join("a/aa/aaa"),
            DIR.join("a/aa/aab"),
            DIR.join("a/ab"),
            DIR.join("a/ab/aba"),
            DIR.join("a/ab/abb")
        ];
        assert_eq!(actual, expect);
    }

}

// cSpell:ignore walkdir
