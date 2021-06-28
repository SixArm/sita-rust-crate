use std::path::PathBuf;
use crate::types::*;

/// Process a glob string set to a path buf set.
/// 
/// Example:
//
/// ```rust
/// let globs = vec!["a/*", "b/*"];
/// let paths = globs_to_paths(globs);
/// //=> ["a", "a/a1.txt", "a/a2.txt", "b", "b/b1.txt", "b/b2.txt"]
/// ```
///
/// This function deliberately ignores errors.
///
#[allow(dead_code)]
pub fn glob_string_set_to_path_buf_set(glob_string_set: &Set<GlobString>) -> Set<PathBuf> {
    let x: Set<PathBuf> = glob_string_set.iter().flat_map(|glob_string| 
        ::glob::glob(&glob_string).unwrap().filter_map(|x| x.ok())
    ).collect::<_>();
    x
}

/// Process a pathable string list to a path buf list.
/// 
/// Example:
//
/// ```rust
/// let pathable_string_list = vec!["a/*", "b/*"];
/// let path_buf_list = pathable_string_list_to_path_buf_list(pathable_string_list);
/// //=> ["a", "a/a1.txt", "a/a2.txt", "b", "b/b1.txt", "b/b2.txt"]
/// ```
///
/// This function deliberately ignores errors.
///
#[allow(dead_code)]
pub fn pathable_string_list_to_path_buf_list(pathable_string_list: &List<PathableString>) -> List<PathBuf> {
    let x: List<PathBuf> = pathable_string_list.iter().flat_map(|pathable_string| 
        ::glob::glob(&pathable_string).unwrap().filter_map(|x| x.ok())
    ).collect::<_>();
    x
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::*;

    lazy_static! {
        pub static ref TESTS_DIR: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests"].iter().collect::<PathBuf>();
    }    

    #[test]
    fn test_glob_string_set_to_path_buf_set() {
        let dir_as_buf = TESTS_DIR.join("function").join("glob_string_set_to_path_buf_set");
        let dir_as_string = dir_as_buf.to_string_lossy();
        let globs: Set<String> = set![
            format!("{}{}", dir_as_string, "/a/**/*"),
            format!("{}{}", dir_as_string, "/b/**/*")
        ];
        let actual: Set<PathBuf> = glob_string_set_to_path_buf_set(&globs);
        let expect: Set<PathBuf> = set![
            dir_as_buf.join("a/aa"),
            dir_as_buf.join("a/aa/aaa"),
            dir_as_buf.join("a/aa/aab"),
            dir_as_buf.join("a/ab"),
            dir_as_buf.join("a/ab/aba"),
            dir_as_buf.join("a/ab/abb"),
            dir_as_buf.join("b/ba"),
            dir_as_buf.join("b/ba/baa"),
            dir_as_buf.join("b/ba/bab"),
            dir_as_buf.join("b/bb"),
            dir_as_buf.join("b/bb/bba"),
            dir_as_buf.join("b/bb/bbb"),
            dir_as_buf.join("b/bb/bbb")
        ];
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_pathable_string_list_to_path_buf_list() {
        let dir_as_buf = TESTS_DIR.join("function").join("pathable_string_list_to_path_buf_list");
        let dir_as_string = dir_as_buf.to_string_lossy();
        let globs: Set<String> = set![
            format!("{}{}", dir_as_string, "/a/**/*"),
            format!("{}{}", dir_as_string, "/b/**/*")
        ];
        let actual: Set<PathBuf> = glob_string_set_to_path_buf_set(&globs);
        let expect: Set<PathBuf> = set![
            dir_as_buf.join("a/aa"),
            dir_as_buf.join("a/aa/aaa"),
            dir_as_buf.join("a/aa/aab"),
            dir_as_buf.join("a/ab"),
            dir_as_buf.join("a/ab/aba"),
            dir_as_buf.join("a/ab/abb"),
            dir_as_buf.join("b/ba"),
            dir_as_buf.join("b/ba/baa"),
            dir_as_buf.join("b/ba/bab"),
            dir_as_buf.join("b/bb"),
            dir_as_buf.join("b/bb/bba"),
            dir_as_buf.join("b/bb/bbb"),
            dir_as_buf.join("b/bb/bbb")
        ];
        assert_eq!(actual, expect);
    }

}
