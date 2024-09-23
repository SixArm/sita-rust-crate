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
    let output: PathBuf = DIR.join("custom-file-name.html");
    let expect: PathBuf = DIR.join("custom-file-name.html=expect.html");
    assert!(input.exists(), "input path: {:?}", input);
    assert!(expect.exists(), "expect path: {:?}", expect);
    assert_ok!(remove_file_if_exists(&output));
    // When
    assert!(!output.exists(), "!output.exists() path: {:?}", output);
    let command_result = std::process::Command::new(&*COMMAND_OS)
        .arg("--input")
        .arg(input.as_os_str())
        .arg("--output")
        .arg(output.as_os_str())
        .output();
    // Then
    assert_ok!(command_result);
    assert!(output.exists(), "output.exists() path: {:?}", output);
    assert_fs_read_to_string_eq!(&output, &expect);
    // Done
    assert_ok!(remove_file_if_exists(&output));
}
