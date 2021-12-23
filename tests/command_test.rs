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
fn test_command_x_input() {
    // Given
    let dir: PathBuf = DIR.join("command_x_input");
    let input: PathBuf = dir.join("example.md");
    let actual: PathBuf = dir.join("example.html");
    let expect: PathBuf = dir.join("example.html=expect.html");
    assert!(input.exists(), "input path: {:?}", input);
    assert!(expect.exists(), "expect path: {:?}", expect);
    remove_file_if_exists(&actual).expect("remove");
    // When
    assert!(!actual.exists(), "!actual.exists() path: {:?}", actual);
    let _output = Command::new(COMMAND)
        .arg("--input")
        .arg(input.as_os_str())
        .output()
        .expect("failure");
    // Then
    assert!(actual.exists(), "actual.exists() path: {:?}", actual);
    assert_fn_ok_eq!(
        ::std::fs::read_to_string,
        &actual,
        &expect,
    );
    // Done
    remove_file_if_exists(&actual).expect("remove");
}

//TODO
// #[test]
// fn test_command_x_output() {
//     // Given
//     let dir: PathBuf = DIR.join("command_x_output");
//     let input: PathBuf = dir.join("example.md");
//     let actual: PathBuf = dir.join("output.html");
//     let expect: PathBuf = dir.join("output.html=expect.html");
//     assert!(input.exists(), "input path: {:?}", input);
//     assert!(expect.exists(), "expect path: {:?}", expect);
//     remove_file_if_exists(&actual).expect("remove");
//     // When
//     assert!(!actual.exists(), "!actual.exists() path: {:?}", actual);
//     let _output = Command::new(COMMAND)
//         .arg("--input")
//         .arg(input.as_os_str())
//         .arg("--output")
//         .arg(actual.as_os_str())
//         .output()
//         .expect("failure");
//     // Then
//     assert!(actual.exists(), "actual.exists() path: {:?}", actual);
//     assert_fn_ok_eq!(
//         ::std::fs::read_to_string,
//         &actual,
//         &expect,
//     );
//     // Done
//     remove_file_if_exists(&actual).expect("remove");
// }

#[test]
fn test_command_x_template() {
    // Given
    let dir: PathBuf = DIR.join("command_x_template");
    let template: PathBuf = dir.join("custom-template.html");
    let input: PathBuf = dir.join("example.md");
    let actual: PathBuf = dir.join("example.html");
    let expect: PathBuf = dir.join("example.html=expect.html");
    assert!(input.exists(), "input path: {:?}", input);
    assert!(template.exists(), "template path: {:?}", template);
    assert!(expect.exists(), "expect path: {:?}", expect);
    remove_file_if_exists(&actual).expect("remove");
    // When
    assert!(!actual.exists(), "!actual.exists() path: {:?}", actual);
    let _output = Command::new(COMMAND)
        .arg("--input")
        .arg(input.as_os_str())
        .arg("--template")
        .arg(template.as_os_str())
        .output()
        .expect("failure");
    // Then
    assert!(actual.exists(), "actual.exists() path: {:?}", actual);
    assert_fn_ok_eq!(
        ::std::fs::read_to_string,
        &actual,
        &expect,
    );
    // Done
    remove_file_if_exists(&actual).expect("remove");
}
