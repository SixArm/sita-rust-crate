use ::assertables::*;
use ::std::ffi::OsStr;
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
    assert!(input.exists());
    assert!(expect.exists());
    // Run
    assert!(!actual.exists());
    let _output = Command::new(COMMAND)
        .arg(&input)
        .output()
        .expect("failure");
    assert!(actual.exists());
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
    assert!(input.exists());
    assert!(expect.exists());
    // Run
    assert!(!actual.exists());
    let _output = Command::new(COMMAND)
        .arg(&input)
        .arg("--output-file")
        .arg(actual.as_os_str())
        .output()
        .expect("failure");
    assert!(actual.exists());
    assert_fn_ok_eq!(
        ::std::fs::read_to_string,
        &actual,
        &expect,
    );
    // Done
    ::std::fs::remove_file(&actual).unwrap();
}

#[test]
fn test_command_x_template_name() {
    // Prep
    let dir: PathBuf = TESTS_DIR.join("command_x_template_name");
    let input: PathBuf = dir.join("example.md");
    let template: PathBuf = dir.join("custom-template-name.html");
    let actual: PathBuf = dir.join("example.html");
    let expect: PathBuf = dir.join("expect.html");
    assert!(input.exists());
    assert!(template.exists());
    assert!(expect.exists());
    // Run
    assert!(!actual.exists());
    let _output = Command::new(COMMAND)
        .arg(&input)
        .arg("--template-name")
        .arg(template.file_name().unwrap()) // because Tera favors using the file name as the template name
        .arg("--template-glob")
        .arg(template.as_os_str()) // because Tera favors initialization via globs not one file
        .output()
        .expect("failure");
    assert!(actual.exists());
    assert_fn_ok_eq!(
        ::std::fs::read_to_string,
        &actual,
        &expect,
    );
    // Done
    ::std::fs::remove_file(&actual).unwrap();
}
