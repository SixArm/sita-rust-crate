use ::assertables::*;
use ::std::process::Command;
use ::std::path::PathBuf;

#[path = "../util.rs"]
mod util;
use util::*;

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
