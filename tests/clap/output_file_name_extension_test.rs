use std::process::Command;

#[path = "util.rs"]
mod util;
use util::*;

#[test]
fn test_clap_output_file_name_extension() {
    assert_command_stdout_contains(
        COMMAND, 
        &["--test", "--output-extension", "alpha"], 
        r#" output_file_name_extension: Some("alpha")"#
    );
}
