use std::path::PathBuf;
use crate::types::*;

/// Convert from &List<PathableString> into List<PathBuf>
/// 
/// Example:
//
/// ```rust
/// let from: List<PathableString> = vec!["a/*", "b/*"];
/// let into: List<PathBuf> = from_list_pathable_string_into_list_path_buf(pathable_string_list);
/// //=> ["a", "a/a1.txt", "a/a2.txt", "b", "b/b1.txt", "b/b2.txt"]
/// ```
///
/// This function deliberately ignores errors.
///
#[allow(dead_code)]
pub fn from_list_pathable_string_into_list_path_buf(pathable_string_list: &List<PathableString>) -> List<PathBuf> {
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
    fn test_from_list_pathable_string_into_list_path_buf() {
        let from: List<PathableString> = list!["alpha".into(), "bravo".into()];
        let actual: List<PathBuf> = from_list_pathable_string_into_list_path_buf(&from);
        let expect: List<PathBuf> = list![PathBuf::from("alpha"), PathBuf::from("bravo")];
        assert_eq!(actual, expect);
    }

}
