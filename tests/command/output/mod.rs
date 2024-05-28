use crate::testing::*;
use assertables::*;
use once_cell::sync::Lazy;
use std::path::PathBuf;

pub static DIR: Lazy<PathBuf> =
    Lazy::new(|| crate::testing::TESTS_DIR.join("command").join("output"));

#[test]
fn test() {
    // Given
    let input: PathBuf = DIR.join("example.md");
    let actual: PathBuf = DIR.join("custom-output-file-name.html");
    let expect: PathBuf = DIR.join("custom-output-file-name.html=expect.html");
    assert!(input.exists(), "input path: {:?}", input);
    assert!(expect.exists(), "expect path: {:?}", expect);
    remove_file_if_exists(&actual).expect("remove");
    // When
    assert!(!actual.exists(), "!actual.exists() path: {:?}", actual);
    let _output = std::process::Command::new(&*COMMAND_OS)
        .arg("--input")
        .arg(input.as_os_str())
        .arg("--output")
        .arg(actual.as_os_str())
        .output()
        .expect("failure");
    // Then
    assert!(actual.exists(), "actual.exists() path: {:?}", actual);
    assert_fn_ok_eq_other!(std::fs::read_to_string, &actual, &expect,);
    // Done
    remove_file_if_exists(&actual).expect("remove");
}
