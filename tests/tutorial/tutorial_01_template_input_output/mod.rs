use crate::testing::*;
use assertables::*;
use once_cell::sync::Lazy;
use std::path::PathBuf;

pub static DIR: Lazy<PathBuf> = Lazy::new(|| {
    crate::testing::TESTS_DIR
        .join("tutorial")
        .join("tutorial_01_template_input_output")
});

#[test]
pub fn test() {
    // Given
    let template = DIR.join("template.html");
    let input = DIR.join("hello.md");
    let output = DIR.join("hello.html");
    let expect = DIR.join("hello.html=expect.html");
    assert_ok!(remove_file_if_exists(&output));
    assert!(!output.exists(), "output path: {:?}", output);
    // When
    let command_result = std::process::Command::new(&*COMMAND_OS)
        .arg("--template")
        .arg(template.as_os_str())
        .arg("--input")
        .arg(input.as_os_str())
        .arg("--output")
        .arg(output.as_os_str())
        .output();
    // Then
    assert_ok!(&command_result);
    let command_output = command_result.unwrap();
    let stdout_string = String::from_utf8(command_output.stdout).unwrap();
    let stderr_string = String::from_utf8(command_output.stderr).unwrap();
    assert_eq!(stdout_string, "");
    assert_eq!(stderr_string, "");
    assert!(output.exists(), "output path: {:?}", output);
    assert_fs_read_to_string_eq!(&output, &expect);
    // Done
    assert_ok!(remove_file_if_exists(&output));
}
