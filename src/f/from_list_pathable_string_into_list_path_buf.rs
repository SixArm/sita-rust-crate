use std::path::PathBuf;
use crate::errors::*;
use crate::types::*;
use crate::f::from_pathable_string_into_list_path_buf::*;

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
    use once_cell::sync::Lazy;

    pub static DIR: Lazy<PathBuf> = Lazy::new(||
        crate::test::TESTS_DIR
        .join("src")
        .join("f")
        .join("from_pathable_string_into_list_path_buf")
    );

    #[test]
    fn test_from_list_pathable_string_into_list_path_buf_x_dir() {
        let dir_as_string = DIR.to_string_lossy();
        let from: List<PathableString> = list![
            format!("{}{}", dir_as_string, "/a"),
            format!("{}{}", dir_as_string, "/b")
        ];
        let result = from_list_pathable_string_into_list_path_buf(&from);
        assert!(result.is_ok());
        let mut actual: List<PathBuf> = result.unwrap();
        actual.sort();
        let expect: List<PathBuf> = list![
            DIR.join("a"),
            DIR.join("a/aa"),
            DIR.join("a/aa/aaa"),
            DIR.join("a/aa/aab"),
            DIR.join("a/ab"),
            DIR.join("a/ab/aba"),
            DIR.join("a/ab/abb"),
            DIR.join("b"),
            DIR.join("b/ba"),
            DIR.join("b/ba/baa"),
            DIR.join("b/ba/bab"),
            DIR.join("b/bb"),
            DIR.join("b/bb/bba"),
            DIR.join("b/bb/bbb")
        ];
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_from_list_pathable_string_into_list_path_buf_x_glob() {
        let dir_as_string = DIR.to_string_lossy();
        let from: List<PathableString> = list![
            format!("{}{}", dir_as_string, "/a*"),
            format!("{}{}", dir_as_string, "/b*")
        ];
        let result = from_list_pathable_string_into_list_path_buf(&from);
        assert!(result.is_ok());
        let mut actual: List<PathBuf> = result.unwrap();
        actual.sort();
        let expect: List<PathBuf> = list![
            DIR.join("a"),
            DIR.join("a/aa"),
            DIR.join("a/aa/aaa"),
            DIR.join("a/aa/aab"),
            DIR.join("a/ab"),
            DIR.join("a/ab/aba"),
            DIR.join("a/ab/abb"),
            DIR.join("b"),
            DIR.join("b/ba"),
            DIR.join("b/ba/baa"),
            DIR.join("b/ba/bab"),
            DIR.join("b/bb"),
            DIR.join("b/bb/bba"),
            DIR.join("b/bb/bbb")
        ];
        assert_eq!(actual, expect);
    }

}
