use crate::testing::*;
use once_cell::sync::Lazy;
use std::path::PathBuf;

pub static DIR: Lazy<PathBuf> = Lazy::new(|| {
    crate::testing::TESTS_DIR
        .join("markdown")
        .join("matter")
        .join("kinds")
        .join("html")
});

#[test]
fn test() {
    test_with_base_path_and_default_template_input_output_expect(&DIR);
}
