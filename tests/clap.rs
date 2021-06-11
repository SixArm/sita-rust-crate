use ::std::process::Command;

#[path = "util.rs"]
mod util;
use util::*;

fn assert_command_stdout_contains(command_name: &str, command_args: &[&str], target: &str) {
    let output = Command::new(command_name)
    .args(command_args)
    .output()
    .expect("failure");
    let actual = ::std::str::from_utf8(&output.stdout)
    .unwrap()
    .to_string();
    assert!(
        actual.contains(target), 
        format!("command: {:?}\nargs: {:?}\nactual: {:?}\ntarget: {}\n", &command_name, &command_args, &actual, &target)
    );
}

#[test]
fn test_clap_test() {
    assert_command_stdout_contains(COMMAND, &["--test"], r#"Args { "#);
}

#[test]
fn test_clap_verbose() {
    assert_command_stdout_contains(COMMAND, &["--test"], r#" log_level: None"#);
    assert_command_stdout_contains(COMMAND, &["--test", "-v"], r#" log_level: Some(Error)"#);
    assert_command_stdout_contains(COMMAND, &["--test", "-vv"], r#" log_level: Some(Warn)"#);
    assert_command_stdout_contains(COMMAND, &["--test", "-vvv"], r#" log_level: Some(Info)"#);
    assert_command_stdout_contains(COMMAND, &["--test", "-vvvv"], r#" log_level: Some(Debug)"#);
    assert_command_stdout_contains(COMMAND, &["--test", "-vvvvv"], r#" log_level: Some(Trace)"#);
}

#[test]
fn test_clap_set() {
    assert_command_stdout_contains(
        COMMAND, 
        &["--test", "--set", "alpha", "bravo", "--set", "charlie", "delta"], 
        r#" settings: Some({"alpha": "bravo", "charlie": "delta"})"#
    );
}
