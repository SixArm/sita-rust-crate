use ::assertables::*;
use ::std::path::PathBuf;
use ::std::process::Command;

#[path = "util.rs"]
mod util;
use util::*;

#[test]
fn test_command_x_default() {
    // Prep
    let dir: PathBuf = TESTS_DIR.join("command_x_default");
    let input: PathBuf = dir.join("example.md");
    let actual: PathBuf = dir.join("example.html");
    let expect: PathBuf = dir.join("expect.html");
    assert!(input.exists(), "input path: {:?}", input);
    assert!(expect.exists(), "expect path: {:?}", expect);
    // Run
    assert!(!actual.exists(), "actual path: {:?}", actual);
    let _output = Command::new(COMMAND)
        .arg("--input")
        .arg(&input)
        .output()
        .expect("failure");
    assert!(actual.exists(), "actual path: {:?}", actual);
    assert_fn_ok_eq!(
        ::std::fs::read_to_string,
        &actual,
        &expect,
    );
    // Done
    ::std::fs::remove_file(&actual).expect("remove");
}

#[test]
fn test_command_x_output_file() {
    // Prep
    let dir: PathBuf = TESTS_DIR.join("command_x_output_file");
    let input: PathBuf = dir.join("example.md");
    let actual: PathBuf = dir.join("custom-output-file.html");
    let expect: PathBuf = dir.join("expect.html");
    assert!(input.exists(), "input path: {:?}", input);
    assert!(expect.exists(), "expect path: {:?}", expect);
    // Run
    assert!(!actual.exists(), "actual path: {:?}", actual);
    let _output = Command::new(COMMAND)
        .arg("--input")
        .arg(&input)
        .arg("--output-file")
        .arg(actual.as_os_str())
        .output()
        .expect("failure");
    assert!(actual.exists(), "actual path: {:?}", actual);
    assert_fn_ok_eq!(
        ::std::fs::read_to_string,
        &actual,
        &expect,
    );
    // Done
    ::std::fs::remove_file(&actual).unwrap();
}

#[test]
fn test_command_x_template_file() {
    // Prep
    let dir: PathBuf = TESTS_DIR.join("command_x_template_file");
    let input: PathBuf = dir.join("example.md");
    let template: PathBuf = dir.join("custom-template-file.html");
    let actual: PathBuf = dir.join("example.html");
    let expect: PathBuf = dir.join("expect.html");
    assert!(input.exists(), "input path: {:?}", input);
    assert!(template.exists(), "template path: {:?}", template);
    assert!(expect.exists(), "expect path: {:?}", expect);
    // Run
    assert!(!actual.exists(), "actual path: {:?}", actual);
    let _output = Command::new(COMMAND)
        .arg("--input")
        .arg(&input)
        .arg("--template")
        .arg(template.as_os_str())
        .output()
        .expect("failure");
    assert!(actual.exists(), "actual path: {:?}", actual);
    assert_fn_ok_eq!(
        ::std::fs::read_to_string,
        &actual,
        &expect,
    );
    // Done
    ::std::fs::remove_file(&actual).unwrap();
}
