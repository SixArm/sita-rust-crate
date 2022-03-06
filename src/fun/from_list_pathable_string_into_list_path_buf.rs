use std::path::PathBuf;
use crate::errors::*;
use crate::types::*;
use crate::fun::from_pathable_string_into_list_path_buf::*;

/// Convert from &List<PathableString> into List<PathBuf>.
/// 
/// Example:
//
/// ```rust
/// let from: List<PathableString> = vec!["a/*", "b/*"];
/// let into: List<PathBuf> = from_list_pathable_string_into_list_path_buf(from);
/// //=> ["a", "a/a1.txt", "a/a2.txt", "b", "b/b1.txt", "b/b2.txt"]
/// ```
///
/// This function deliberately filters out errors.
/// 
/// For example, this function will silently skip directories that the 
/// owner of the running process does not have permission to access.
/// 
#[allow(dead_code)]
pub fn from_list_pathable_string_into_list_path_buf(from: &List<PathableString>) -> Result<List<PathBuf>> {
    trace!("from_list_pathable_string_into_list_path_buf");
    let list_path_buf: List<PathBuf> = from.iter().map(|from|
        from_pathable_string_into_list_path_buf(from)
    )
    .inspect(|x|
        match x {
            Ok(x) => trace!("from_list_pathable_string_into_list_path_buf ok. ␟from: {:?} ␟path: {:?}", from, x),
            Err(err) => warn!("from_list_pathable_string_into_list_path_buf err. ␟from: {:?} ␟err: {:?}", from, err),
        }
    )
    .filter_map(|x|
        x.ok()
    )
    .flatten()
    .collect::<_>();
    Ok(list_path_buf)
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::*;

    lazy_static! {
        pub static ref TESTS_DIR: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests"].iter().collect::<PathBuf>();
    }    

    #[test]
    fn test_from_list_pathable_string_into_list_path_buf_x_dir() {
        let dir_as_buf = TESTS_DIR.join("function").join("from_pathable_string_into_list_path_buf");
        let dir_as_string = dir_as_buf.to_string_lossy();
        let from: List<PathableString> = list![
            format!("{}{}", dir_as_string, "/a"),
            format!("{}{}", dir_as_string, "/b")
        ];
        let result = from_list_pathable_string_into_list_path_buf(&from);
        assert!(result.is_ok());
        let mut actual: List<PathBuf> = result.unwrap();
        actual.sort();
        let expect: List<PathBuf> = list![
            dir_as_buf.join("a"),
            dir_as_buf.join("a/aa"),
            dir_as_buf.join("a/aa/aaa"),
            dir_as_buf.join("a/aa/aab"),
            dir_as_buf.join("a/ab"),
            dir_as_buf.join("a/ab/aba"),
            dir_as_buf.join("a/ab/abb"),
            dir_as_buf.join("b"),
            dir_as_buf.join("b/ba"),
            dir_as_buf.join("b/ba/baa"),
            dir_as_buf.join("b/ba/bab"),
            dir_as_buf.join("b/bb"),
            dir_as_buf.join("b/bb/bba"),
            dir_as_buf.join("b/bb/bbb")
        ];
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_from_list_pathable_string_into_list_path_buf_x_glob() {
        let dir_as_buf = TESTS_DIR.join("function").join("from_pathable_string_into_list_path_buf");
        let dir_as_string = dir_as_buf.to_string_lossy();
        let from: List<PathableString> = list![
            format!("{}{}", dir_as_string, "/a*"),
            format!("{}{}", dir_as_string, "/b*")
        ];
        let result = from_list_pathable_string_into_list_path_buf(&from);
        assert!(result.is_ok());
        let mut actual: List<PathBuf> = result.unwrap();
        actual.sort();
        let expect: List<PathBuf> = list![
            dir_as_buf.join("a"),
            dir_as_buf.join("a/aa"),
            dir_as_buf.join("a/aa/aaa"),
            dir_as_buf.join("a/aa/aab"),
            dir_as_buf.join("a/ab"),
            dir_as_buf.join("a/ab/aba"),
            dir_as_buf.join("a/ab/abb"),
            dir_as_buf.join("b"),
            dir_as_buf.join("b/ba"),
            dir_as_buf.join("b/ba/baa"),
            dir_as_buf.join("b/ba/bab"),
            dir_as_buf.join("b/bb"),
            dir_as_buf.join("b/bb/bba"),
            dir_as_buf.join("b/bb/bbb")
        ];
        assert_eq!(actual, expect);
    }

}
