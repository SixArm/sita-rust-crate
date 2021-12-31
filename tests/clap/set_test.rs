use std::process::Command;

#[path = "util.rs"]
mod util;
use util::*;

// #[test]
// fn test_set() {
//     assert_command_stdout_contains(
//         COMMAND, 
//         &["--test", "--set", "alpha", "bravo", "--set", "charlie", "delta"], 
//         r#" settings: Some({"alpha": "bravo", "charlie": "delta"})"#
//     );
// }
