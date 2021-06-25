use lazy_static::*;
use std::path::PathBuf;

#[path = "util.rs"]
mod util;
use util::*;

lazy_static! {
    pub static ref DIR: PathBuf = TESTS_DIR.join("markdown").join("matter").join("kinds");
}

#[test]
fn test_html() {
    test_with_base_path_and_default_input_actual_expect(
        MY_DIR.join("html")
    );
}

#[test]
fn test_json() {
    test_with_base_path_and_default_input_actual_expect(
        DIR.join("json")
    );
}

#[test]
fn test_toml() {
    test_with_base_path_and_default_input_actual_expect(
        DIR.join("toml")
    );
}

#[test]
fn test_yaml() {
    test_with_base_path_and_default_input_actual_expect(
        DIR.join("yaml")
    );
}
