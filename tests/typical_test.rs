use ::assertables::*;
use ::std::process::Command;
use ::std::path::PathBuf;

#[path = "util.rs"]
mod util;
use util::*;


fn test_with_base_path_and_default_input_expect_actual(base_path: PathBuf) {
    let input = base_path.join("input.md");
    let expect = base_path.join("expect.html");
    let actual = base_path.join("actual.html");
    let _output = Command::new(COMMAND)
        .arg(&input)
        .output()
        .expect("failure");
    assert_fn_ok_eq!(
        ::std::fs::read_to_string,
        &actual,
        &expect,
    );
    remove_file_if_exists(&actual).expect("remove");
}

fn test_with_base_path_and_default_input_template_expect_actual(base_path: PathBuf) {
    let input = base_path.join("input.md");
    let template = base_path.join("template.md");
    let expect = base_path.join("expect.html");
    let actual = base_path.join("actual.html");
    let _output = Command::new(COMMAND)
        .arg(&input)
        .output()
        .expect("failure");
    assert_fn_ok_eq!(
        ::std::fs::read_to_string,
        &actual,
        &expect,
    );
    remove_file_if_exists(&actual).expect("remove");
}

#[test]
fn test_minimal_html() {
    test_with_base_path_and_default_input_expect_actual(
        TESTS_DIR.join("minimal_html")
    );
}

#[test]
fn test_front_matter_html() {
    test_with_base_path_and_default_input_expect_actual(
        TESTS_DIR.join("front_matter").join("html")
    );
}

#[test]
fn test_front_matter_json() {
    test_with_base_path_and_default_input_expect_actual(
        TESTS_DIR.join("front_matter").join("json")
    );
}

#[test]
fn test_front_matter_toml() {
    test_with_base_path_and_default_input_expect_actual(
        TESTS_DIR.join("front_matter").join("toml")
    );
}

#[test]
fn test_front_matter_yaml() {
    test_with_base_path_and_default_input_expect_actual(
        TESTS_DIR.join("front_matter").join("yaml")
    );
}
