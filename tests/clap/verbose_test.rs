use std::process::Command;

#[path = "util.rs"]
mod util;
use util::*;

// Test that the special argument `--verbose` is working.
//
// This test must succeed in order for any of the rest of the tests here to
// show diagnostics, because the `--verbose` argument turns on logging output,
// which can include debugging messages, warnings, errors, and so on.
//
// #[test]
// fn test_verbose() {
//     assert_command_stdout_contains(COMMAND, &["--test"], r#" log_level: None"#);
//     assert_command_stdout_contains(COMMAND, &["--test", "-v"], r#" log_level: Some(Error)"#);
//     assert_command_stdout_contains(COMMAND, &["--test", "-vv"], r#" log_level: Some(Warn)"#);
//     assert_command_stdout_contains(COMMAND, &["--test", "-vvv"], r#" log_level: Some(Info)"#);
//     assert_command_stdout_contains(COMMAND, &["--test", "-vvvv"], r#" log_level: Some(Debug)"#);
//     assert_command_stdout_contains(COMMAND, &["--test", "-vvvvv"], r#" log_level: Some(Trace)"#);
//     assert_command_stdout_contains(COMMAND, &["--test", "--verbose"], r#" log_level: Some(Error)"#);
//     assert_command_stdout_contains(COMMAND, &["--test", "--verbose", "--verbose"], r#" log_level: Some(Warn)"#);
//     assert_command_stdout_contains(COMMAND, &["--test", "--verbose", "--verbose", "--verbose"], r#" log_level: Some(Info)"#);
//     assert_command_stdout_contains(COMMAND, &["--test", "--verbose", "--verbose", "--verbose", "--verbose"], r#" log_level: Some(Debug)"#);
//     assert_command_stdout_contains(COMMAND, &["--test", "--verbose", "--verbose", "--verbose", "--verbose", "--verbose"], r#" log_level: Some(Trace)"#);
// }
