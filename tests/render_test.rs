#[path = "testing.rs"]
mod testing;
use testing::*;

#[test]
fn test_minimal() {
    test_with_base_path_and_default_input_template_actual_expect(
        TESTS_DIR
        .join("render")
        .join("minimal")
    );
}

#[test]
fn test_matter_x_html() {
    test_with_base_path_and_default_input_template_actual_expect(
        TESTS_DIR
        .join("render")
        .join("matter_x_html")
    );
}

#[test]
fn test_matter_x_json() {
    test_with_base_path_and_default_input_template_actual_expect(
        TESTS_DIR
        .join("render")
        .join("matter_x_json")
    );
}

#[test]
fn test_matter_x_markdown_comments() {
    test_with_base_path_and_default_input_template_actual_expect(
        TESTS_DIR
        .join("render")
        .join("matter_x_markdown_comments")
    );
}

#[test]
fn test_matter_x_toml() {
    test_with_base_path_and_default_input_template_actual_expect(
        TESTS_DIR
        .join("render")
        .join("matter_x_toml")
    );
}

#[test]
fn test_matter_x_yaml() {
    test_with_base_path_and_default_input_template_actual_expect(
        TESTS_DIR
        .join("render")
        .join("matter_x_yaml")
    );
}

#[test]
fn test_title() {
    test_with_base_path_and_default_input_template_actual_expect(
        TESTS_DIR
        .join("render")
        .join("title")
    );
}
