use assertables::*;
use std::path::PathBuf;
use std::process::Command;

#[path = "testing.rs"]
mod testing;
use testing::*;

//TODO
// #[test]
// fn test_command_x_output() {
//     // Given
//     let input: PathBuf = DIR.join("example.md");
//     let actual: PathBuf = DIR.join("output.html");
//     let expect: PathBuf = DIR.join("output.html=expect.html");
//     assert!(input.exists(), "input path: {:?}", input);
//     assert!(expect.exists(), "expect path: {:?}", expect);
//     remove_file_if_exists(&actual).expect("remove");
//     // When
//     assert!(!actual.exists(), "!actual.exists() path: {:?}", actual);
//     let _output = Command::new(COMMAND)
//         .arg("--input")
//         .arg(input.as_os_str())
//         .arg("--output")
//         .arg(actual.as_os_str())
//         .output()
//         .expect("failure");
//     // Then
//     assert!(actual.exists(), "actual.exists() path: {:?}", actual);
//     assert_fn_ok_eq_other!(
//         ::std::fs::read_to_string,
//         &actual,
//         &expect,
//     );
//     // Done
//     remove_file_if_exists(&actual).expect("remove");
// }