use std::process::Command;

#[path = "util.rs"]
mod util;
use util::*;

// #[test]
// fn test_clap_script() {
//     assert_command_stdout_contains(
//         COMMAND, 
//         &["--test", "--script", "alpha", "bravo", "--script", "charlie", "delta"], 
//         r#" script_url_list: Some(["alpha", "bravo", "charlie", "delta"])"#
//     );
// }
