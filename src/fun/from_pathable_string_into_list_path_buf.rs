use std::path::PathBuf;
//use glob::GlobError;
use walkdir::WalkDir;

use crate::errors::*;
use crate::types::*;

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
pub fn from_pathable_string_into_list_path_buf(from: &PathableString) -> Result<List<PathBuf>> {
    trace!("from_pathable_string_into_list_path_buf from: {:?}", from); 
    let list_path_buf: List<PathBuf> = ::glob::glob(&from)
    .chain_err(|| format!("from_pathable_string_into_list_path_buf glob from: {:?}", from))?
    .inspect(|x|
        println!("f1: {:?}", x)
    )
    .inspect(|x|
        match x {
            Ok(x) => trace!("from_pathable_string_into_list_path_buf glob ok. ␟from: {:?} ␟path: {:?}", from, x),
            Err(err) => warn!("from_pathable_string_into_list_path_buf glob err. ␟from: {:?} ␟err: {:?}", from, err),
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
            crate::fun::walkdir_dir_entry_is_visible::walkdir_dir_entry_is_visible(&e)
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

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::*;

    lazy_static! {
        pub static ref TESTS_DIR: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests"].iter().collect::<PathBuf>();
    }    

    #[test]
    fn test_from_pathable_string_into_list_path_buf_x_dir() {
        let dir_as_buf = TESTS_DIR.join("fun").join("from_pathable_string_into_list_path_buf");
        let dir_as_string = dir_as_buf.to_string_lossy();
        let from: PathableString = format!("{}{}", dir_as_string, "/a");
        let result = from_pathable_string_into_list_path_buf(&from);
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
            dir_as_buf.join("a/ab/abb")
        ];
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_from_pathable_string_into_list_path_buf_x_glob() {
        let dir_as_buf = TESTS_DIR.join("fun").join("from_pathable_string_into_list_path_buf");
        let dir_as_string = dir_as_buf.to_string_lossy();
        let from: PathableString = format!("{}{}", dir_as_string, "/a*");
        let result = from_pathable_string_into_list_path_buf(&from);
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
            dir_as_buf.join("a/ab/abb")
        ];
        assert_eq!(actual, expect);
    }

}
