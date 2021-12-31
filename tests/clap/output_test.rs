use std::process::Command;

#[path = "util.rs"]
mod util;
use util::*;

#[test]
fn test_clap_output() {
    let s1 = "alpha";
    let s2 = "bravo";
    let s3 = "charlie";
    let s4 = "delta";
    let target = format!(" output_list_pathable_string: Some([\"{}\", \"{}\", \"{}\", \"{}\"])", &s1, &s2, &s3, &s4);
    // Test short `-o` with multiple occurrences and multiple values
    assert_command_stdout_contains(
        COMMAND, 
        &["--test", "-o", &s1, &s2, "-o", &s3, &s4], 
        &target
    );
    // Test long `--output` with multiple occurrences and multiple values
    assert_command_stdout_contains(
        COMMAND, 
        &["--test", "--output", &s1, &s2, "--output", &s3, &s4], 
        &target
    );
    // Test alias `--outputs` with multiple occurrences and multiple values
    assert_command_stdout_contains(
        COMMAND, 
        &["--test", "--outputs", &s1, &s2, "--outputs", &s3, &s4], 
        &target
    );
}
