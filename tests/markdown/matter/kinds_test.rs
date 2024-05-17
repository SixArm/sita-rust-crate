use std::path::PathBuf;
use once_cell::sync::Lazy;

#[path = "testing.rs"]
mod testing;
use testing::*;

pub static DIR: Lazy<PathBuf> = Lazy::new(||
    crate::test::TESTS_DIR
    .join("markdown")
    .join("matter")
    .join("kinds")
);

#[test]
fn test_html() {
    test_with_base_path_and_default_input_actual_expect(
        DIR.join("html")
    );
}

#[test]
fn test_json() {
    test_with_base_path_and_default_input_actual_expect(
        DIR.join("json")
    );
}

#[test]
fn test_markdown_comments() {
    test_with_base_path_and_default_input_actual_expect(
        DIR.join("markdown_comments")
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
