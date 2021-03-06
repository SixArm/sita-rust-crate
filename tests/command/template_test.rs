use assertables::*;
use std::path::PathBuf;
use std::process::Command;
use lazy_static::*;

#[path = "testing.rs"]
mod testing;
use testing::*;

lazy_static! {
    pub static ref DIR = TESTS_DIR.join("command").join("template");
}

#[test]
fn test_command_x_template() {
    // Given
    let template: PathBuf = DIR.join("custom-template.html");
    let input: PathBuf = DIR.join("example.md");
    let actual: PathBuf = DIR.join("example.html");
    let expect: PathBuf = DIR.join("example.html=expect.html");
    assert!(input.exists(), "input path: {:?}", input);
    assert!(template.exists(), "template path: {:?}", template);
    assert!(expect.exists(), "expect path: {:?}", expect);
    remove_file_if_exists(&actual).expect("remove");
    // When
    assert!(!actual.exists(), "!actual.exists() path: {:?}", actual);
    let _output = ::std::process::Command::new(&*COMMAND_OS)
        .arg("--input")
        .arg(input.as_os_str())
        .arg("--template")
        .arg(template.as_os_str())
        .output()
        .expect("failure");
    // Then
    assert!(actual.exists(), "actual.exists() path: {:?}", actual);
    assert_fn_ok_eq_other!(
        ::std::fs::read_to_string,
        &actual,
        &expect,
    );
    // Done
    remove_file_if_exists(&actual).expect("remove");
}
