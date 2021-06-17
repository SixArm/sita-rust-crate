#[path = "util.rs"]
mod util;
use util::*;

#[test]
fn test_matter_html() {
    test_with_base_path_and_default_input_actual_expect(
        TESTS_DIR.join("markdown_matter_kinds").join("html")
    );
}

#[test]
fn test_matter_json() {
    test_with_base_path_and_default_input_actual_expect(
        TESTS_DIR.join("markdown_matter_kinds").join("json")
    );
}

#[test]
fn test_matter_toml() {
    test_with_base_path_and_default_input_actual_expect(
        TESTS_DIR.join("markdown_matter_kinds").join("toml")
    );
}

#[test]
fn test_matter_yaml() {
    test_with_base_path_and_default_input_actual_expect(
        TESTS_DIR.join("markdown_matter_kinds").join("yaml")
    );
}
