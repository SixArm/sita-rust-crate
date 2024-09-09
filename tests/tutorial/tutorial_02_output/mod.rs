use crate::testing::*;
use assertables::*;
use once_cell::sync::Lazy;
use std::path::PathBuf;

pub static DIR: Lazy<PathBuf> = Lazy::new(|| {
    crate::testing::TESTS_DIR
        .join("tutorial")
        .join("tutorial_02_output")
});

#[test]
pub fn test() {
    // Given
    let input = DIR.join("hello.md");
    let actual = DIR.join("world.html");
    let expect = DIR.join("world.html=expect.html");
    remove_file_if_exists(&actual).expect("remove");
    // When
    assert!(!actual.exists(), "actual path: {:?}", actual);
    let _output = std::process::Command::new(&*COMMAND_OS)
        .arg("--input")
        .arg(input.as_os_str())
        .arg("--output")
        .arg(actual.as_os_str())
        .output()
        .expect("failure");
    // Then
    assert!(actual.exists(), "actual path: {:?}", actual);
    assert_fs_read_to_string_eq!(&actual, &expect);
    // Done
    remove_file_if_exists(&actual).expect("remove");
}
