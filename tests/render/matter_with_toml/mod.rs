use crate::testing::*;
use once_cell::sync::Lazy;
use std::path::PathBuf;

pub static DIR: Lazy<PathBuf> = Lazy::new(|| {
    crate::testing::TESTS_DIR
        .join("render")
        .join("matter_with_toml")
});

#[test]
fn test() {
    test_with_base_path_and_default_input_template_actual_expect(&DIR);
}
