use assertables::*;
use lazy_static::*;
use std::path::PathBuf;
use std::process::Command;

#[path = "util.rs"]
mod util;
use util::*;

lazy_static! {
    pub static ref DIR: PathBuf = TESTS_DIR.join("command");
}

#[test]
fn test_command_x_default() {
    // Given
    let dir: PathBuf = DIR.join("command_x_default");
    let input: PathBuf = dir.join("example.md");
    let actual: PathBuf = dir.join("example.html");
    let expect: PathBuf = dir.join("example.html=expect.html");
    assert!(input.exists(), "input path: {:?}", input);
    assert!(expect.exists(), "expect path: {:?}", expect);
    remove_file_if_exists(&actual).expect("remove");
    // When
    assert!(!actual.exists(), "actual path: {:?}", actual);
    let _output = Command::new(COMMAND)
        .arg("--input")
        .arg(input.as_os_str())
        .output()
        .expect("failure");
    // Then
    assert!(actual.exists(), "actual path: {:?}", actual);
    assert_fn_ok_eq!(
        ::std::fs::read_to_string,
        &actual,
        &expect,
    );
    // Done
    remove_file_if_exists(&actual).expect("remove");
}

#[test]
fn test_command_x_output_file() {
    // Given
    let dir: PathBuf = DIR.join("command_x_output_file");
    let input: PathBuf = dir.join("example.md");
    let actual: PathBuf = dir.join("custom-output-file.html");
    let expect: PathBuf = dir.join("custom-output-file.html=expect.html");
    assert!(input.exists(), "input path: {:?}", input);
    assert!(expect.exists(), "expect path: {:?}", expect);
    remove_file_if_exists(&actual).expect("remove");
    // When
    assert!(!actual.exists(), "actual path: {:?}", actual);
    let _output = Command::new(COMMAND)
        .arg("--input")
        .arg(input.as_os_str())
        .arg("--output-file")
        .arg(actual.as_os_str())
        .output()
        .expect("failure");
    // Then
    assert!(actual.exists(), "actual path: {:?}", actual);
    assert_fn_ok_eq!(
        ::std::fs::read_to_string,
        &actual,
        &expect,
    );
    // Done
    remove_file_if_exists(&actual).expect("remove");
}

#[test]
fn test_command_x_template_file() {
    // Given
    let dir: PathBuf = DIR.join("command_x_template_file");
    let template: PathBuf = dir.join("custom-template-file.html");
    let input: PathBuf = dir.join("example.md");
    let actual: PathBuf = dir.join("example.html");
    let expect: PathBuf = dir.join("example.html=expect.html");
    assert!(input.exists(), "input path: {:?}", input);
    assert!(template.exists(), "template path: {:?}", template);
    assert!(expect.exists(), "expect path: {:?}", expect);
    remove_file_if_exists(&actual).expect("remove");
    // When
    assert!(!actual.exists(), "actual path: {:?}", actual);
    let _output = Command::new(COMMAND)
        .arg("--input")
        .arg(input.as_os_str())
        .arg("--template")
        .arg(template.as_os_str())
        .output()
        .expect("failure");
    // Then
    assert!(actual.exists(), "actual path: {:?}", actual);
    assert_fn_ok_eq!(
        ::std::fs::read_to_string,
        &actual,
        &expect,
    );
    // Done
    remove_file_if_exists(&actual).expect("remove");
}
