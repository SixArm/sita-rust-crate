#[path = "util.rs"]
mod util;
use util::*;

#[test]
pub fn test_01_input() {
    // Given
    let dir = TESTS_DIR.join("tutorial/01_input");
    let input = dir.join("hello.md");
    let actual = dir.join("hello.html");
    let expect = dir.join("hello.html=expect.html");
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
    assert_fn_ok_eq_other!(
        ::std::fs::read_to_string,
        &actual,
        &expect,
    );
    // Done
    remove_file_if_exists(&actual).expect("remove");
}

#[test]
pub fn test_02_output() {
    // Given
    let dir = TESTS_DIR.join("tutorial/02_output");
    let input = dir.join("hello.md");
    let actual = dir.join("world.html");
    let expect = dir.join("world.html=expect.html");
    remove_file_if_exists(&actual).expect("remove");
    // When
    assert!(!actual.exists(), "actual path: {:?}", actual);
    let _output = Command::new(COMMAND)
        .arg("--input")
        .arg(input.as_os_str())
        .arg("--output")
        .arg(&actual)
        .output()
        .expect("failure");
    // Then
    assert!(actual.exists(), "actual path: {:?}", actual);
    assert_fn_ok_eq_other!(
        ::std::fs::read_to_string,
        &actual,
        &expect,
    );
    // Done
    remove_file_if_exists(&actual).expect("remove");
}

#[test]
pub fn test_03_template() {
    // Given
    let dir = TESTS_DIR.join("tutorial/03_template");
    let template = dir.join("template.html");
    let input = dir.join("hello.md");
    let actual = dir.join("hello.html");
    let expect = dir.join("hello.html=expect.html");
    remove_file_if_exists(&actual).expect("remove");
    // When
    assert!(!actual.exists(), "actual path: {:?}", actual);
    let _output = Command::new(COMMAND)
        .arg("--template")
        .arg(&template)
        .arg("--input")
        .arg(input.as_os_str())
        .output()
        .expect("failure");
    // Then
    assert!(actual.exists(), "actual path: {:?}", actual);
    assert_fn_ok_eq_other!(
        ::std::fs::read_to_string,
        &actual,
        &expect,
    );
    // Done
    remove_file_if_exists(&actual).expect("remove");
}
