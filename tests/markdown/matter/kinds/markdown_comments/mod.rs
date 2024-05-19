use std::path::PathBuf;
use once_cell::sync::Lazy;
use crate::testing::*;

pub static DIR: Lazy<PathBuf> = Lazy::new(||
    crate::testing::TESTS_DIR
    .join("markdown")
    .join("matter")
    .join("kinds")
    .join("markdown_comments")
);

#[test]
fn test() {
    test_with_base_path_and_default_input_actual_expect(
        &DIR
    );
}
