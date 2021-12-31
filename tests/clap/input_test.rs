use std::process::Command;

#[path = "util.rs"]
mod util;
use util::*;

const s1: &str = "alpha";
const s2: &str = "bravo";
const s3: &str = "charlie";
const s4: &str = "delta";

const target: &str = format!(" input_list_pathable_string: Some([\"{}\", \"{}\", \"{}\", \"{}\"])", &s1, &s2, &s3, &s4);

#[test]
fn test_input_x_short_i_with_multiple_occurances_and_multiple_values() {
    let target = format!(" input_list_pathable_string: Some([\"{}\", \"{}\", \"{}\", \"{}\"])", &s1, &s2, &s3, &s4);
    assert_command_stdout_contains(
        COMMAND, 
        &["--test", "-i", &s1, &s2, "-i", &s3, &s4], 
        &target
    );
}

#[test]
fn test_input_x_long_input_with_multiple_occurances_and_multiple_values() {
    assert_command_stdout_contains(
        COMMAND, 
        &["--test", "--input", &s1, &s2, "--input", &s3, &s4], 
        &target
    );
}

#[test]
fn test_input_x_alias_inputs_with_multiple_occurances_and_multiple_values() {
    assert_command_stdout_contains(
        COMMAND, 
        &["--test", "--inputs", &s1, &s2, "--inputs", &s3, &s4], 
        &target
    );
}
