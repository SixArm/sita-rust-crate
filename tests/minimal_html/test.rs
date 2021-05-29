use ::assertables::*;
use ::std::process::Command;
use ::std::path::PathBuf;

#[path = "../util.rs"]
mod util;
use util::*;

#[test]
fn test_minimal_html() {
    test_with_base_path_and_default_input_expect_actual(
        TESTS_DIR.join("minimal_html")
    );
}