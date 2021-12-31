use std::process::Command;

#[path = "util.rs"]
mod util;
use util::*;

// Test that the special argument `--test` is working.
//
// This test must succeed in order for any of the rest of the tests here to
// succeed, because the `--test` argument turns on the runtime output to stdout,
// which includes a typical print debug of the entire `args` structure.
//
#[test]
fn test_test() {
    assert_command_stdout_contains(COMMAND, &["--test"], r#"Args { "#);
}
