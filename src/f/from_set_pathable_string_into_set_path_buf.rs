use std::path::PathBuf;
use crate::types::*;

/// Convert from &Set<GlobString> into Set<PathBuf>.
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
pub fn from_set_pathable_string_into_set_path_buf(glob_string_set: &Set<GlobString>) -> Set<PathBuf> {
    let x: Set<PathBuf> = glob_string_set.iter().flat_map(|glob_string|
        ::glob::glob(&glob_string).unwrap().filter_map(|x| x.ok())
    ).collect::<_>();
    x
}

#[cfg(test)]
mod tests {
    use super::*;
    use once_cell::sync::Lazy;

    pub static DIR: Lazy<PathBuf> = Lazy::new(||
        crate::test::TESTS_DIR
        .join("src")
        .join("f")
        .join("from_set_pathable_string_into_set_path_buf")
    );

    #[test]
    fn test_from_set_pathable_string_into_set_path_buf() {
        let dir_as_string = DIR.to_string_lossy();
        let from: Set<PathableString> = set![
            format!("{}{}", dir_as_string, "/a/**/*"),
            format!("{}{}", dir_as_string, "/b/**/*")
        ];
        let actual: Set<PathBuf> = from_set_pathable_string_into_set_path_buf(&from);
        let expect: Set<PathBuf> = set![
            DIR.join("a/aa"),
            DIR.join("a/aa/aaa"),
            DIR.join("a/aa/aab"),
            DIR.join("a/ab"),
            DIR.join("a/ab/aba"),
            DIR.join("a/ab/abb"),
            DIR.join("b/ba"),
            DIR.join("b/ba/baa"),
            DIR.join("b/ba/bab"),
            DIR.join("b/bb"),
            DIR.join("b/bb/bba"),
            DIR.join("b/bb/bbb"),
            DIR.join("b/bb/bbb")
        ];
        assert_eq!(actual, expect);
    }

}
