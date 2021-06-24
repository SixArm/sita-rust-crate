use ::std::process::Command;

#[path = "util.rs"]
mod util;
use util::*;

#[allow(dead_code)]
fn assert_str_contains(outer: &str, inner: &str) {
    assert!(
        outer.contains(inner), 
        "outer: {:?}\n inner: {}\n", &outer, &inner
    );
}

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
        "command: {:?}\nargs: {:?}\nactual: {:?}\ntarget: {}\n", &command_name, &command_args, &actual, &target
    );
}

/// High-priority CLAP command args tests

// Test that the special argument `--test` is working.
//
// This test must succeed in order for any of the rest of the tests here to
// succeed, because the `--test` argument turns on the runtime output to stdout,
// which includes a typical print debug of the entire `args` structure.
//
#[test]
fn test_clap_test() {
    assert_command_stdout_contains(COMMAND, &["--test"], r#"Args { "#);
}

/// Normal-priority CLAP command args tests

#[test]
fn test_clap_input_pathable_list() {
    let target = r#" input_pathable_list: Some(["alpha", "bravo", "charlie", "delta"])"#;
    assert_command_stdout_contains(
        COMMAND, 
        &["--test", "-i", "alpha", "bravo", "-i", "charlie", "delta"], 
        target
    );
    assert_command_stdout_contains(
        COMMAND, 
        &["--test", "--input", "alpha", "bravo", "--input", "charlie", "delta"], 
        target
    );
    assert_command_stdout_contains(
        COMMAND, 
        &["--test", "--inputs", "alpha", "bravo", "--inputs", "charlie", "delta"], 
        target
    );
}

#[test]
fn test_clap_input_extension() {
    assert_command_stdout_contains(
        COMMAND, 
        &["--test", "--input-extension", "alpha"], 
        r#" input_extension: Some("alpha")"#
    );
}

#[test]
fn test_clap_language() {
    assert_command_stdout_contains(
        COMMAND, 
        &["--test", "--language", "alpha"], 
        r#" language: Some("alpha")"#
    );
}

#[test]
fn test_clap_output_file() {
    assert_command_stdout_contains(
        COMMAND, 
        &["--test", "--output-file", "alpha"], 
        r#" output_file_path: Some("alpha")"#
    );
}

#[test]
fn test_clap_output_directory() {
    assert_command_stdout_contains(
        COMMAND, 
        &["--test", "--output-directory", "alpha"], 
        r#" output_directory_path: Some("alpha")"#
    );
}

#[test]
fn test_clap_output_extension() {
    assert_command_stdout_contains(
        COMMAND, 
        &["--test", "--output-extension", "alpha"], 
        r#" output_extension: Some("alpha")"#
    );
}

#[test]
fn test_clap_script() {
    assert_command_stdout_contains(
        COMMAND, 
        &["--test", "--script", "alpha", "bravo", "--script", "charlie", "delta"], 
        r#" script_url_list: Some(["alpha", "bravo", "charlie", "delta"])"#
    );
}

#[test]
fn test_clap_set() {
    assert_command_stdout_contains(
        COMMAND, 
        &["--test", "--set", "alpha", "bravo", "--set", "charlie", "delta"], 
        r#" settings: Some({"alpha": "bravo", "charlie": "delta"})"#
    );
}

#[test]
fn test_clap_stylesheet() {
    assert_command_stdout_contains(
        COMMAND, 
        &["--test", "--stylesheet", "alpha", "bravo", "--stylesheet", "charlie", "delta"],
        r#" stylesheet_url_list: Some(["alpha", "bravo", "charlie", "delta"])"#
    );
}

#[test]
fn test_clap_template_name() {
    assert_command_stdout_contains(
        COMMAND, 
        &["--test", "--template-name", "alpha"], 
        r#" template_name: Some("alpha")"#
    );
}

#[test]
fn test_clap_template_glob_set() {
    let dir = "glob_string_set_to_path_buf_set/";
    assert_command_stdout_contains(
        COMMAND, 
        &[
            "--test", 
            "--template-glob", 
            &format!("{}{}", &dir, "a/**/*"), 
            &format!("{}{}", &dir, "b/**/*"), 
            "--template-glob", 
            &format!("{}{}", &dir, "c/**/*"), 
            &format!("{}{}", &dir, "d/**/*"), 
        ], 
        &format!(" template_glob_set: Some({{\"{}\", \"{}\", \"{}\", \"{}\"}})",
            &format!("{}{}", &dir, "a/**/*"), 
            &format!("{}{}", &dir, "b/**/*"), 
            &format!("{}{}", &dir, "c/**/*"), 
            &format!("{}{}", &dir, "d/**/*"), 
        )
    );
}

// #[test]
// fn test_clap_template_glob_to_template_path_set() {
//     let dir = "glob_string_set_to_path_buf_set/";
//     assert_command_stdout_contains(
//         COMMAND, 
//         &[
//             "--test", 
//             "--template-glob", 
//             &format!("{}{}", &dir, "a/**/*"), 
//             &format!("{}{}", &dir, "b/**/*"), 
//             "--template-glob", 
//             &format!("{}{}", &dir, "c/**/*"), 
//             &format!("{}{}", &dir, "d/**/*"), 
//         ], 
//         &format!(" template_path_set:  Some([\"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\"]",
//             &format!("{}{}", &dir, "a/aa"),
//             &format!("{}{}", &dir, "a/aa/aaa"),
//             &format!("{}{}", &dir, "a/aa/aab"),
//             &format!("{}{}", &dir, "a/ab"),
//             &format!("{}{}", &dir, "a/ab/aba"),
//             &format!("{}{}", &dir, "a/ab/abb"),
//             &format!("{}{}", &dir, "b/ba"),
//             &format!("{}{}", &dir, "b/ba/baa"),
//             &format!("{}{}", &dir, "b/ba/bab"),
//             &format!("{}{}", &dir, "b/bb"),
//             &format!("{}{}", &dir, "b/bb/bba"),
//             &format!("{}{}", &dir, "b/bb/bbb"),
//             &format!("{}{}", &dir, "b/bb/bbb"),
//             &format!("{}{}", &dir, "c/ca/caa"),
//             &format!("{}{}", &dir, "c/ca/cab"),
//             &format!("{}{}", &dir, "c/cb/cba"),
//             &format!("{}{}", &dir, "c/cb/cbb"),
//             &format!("{}{}", &dir, "d/da/daa"),
//             &format!("{}{}", &dir, "c/da/dab"),
//             &format!("{}{}", &dir, "d/db/dba"),
//             &format!("{}{}", &dir, "d/db/dbb"),
//         )
//     );
// }

#[test]
fn test_clap_template_html_set() {
    assert_command_stdout_contains(
        COMMAND, 
        &["--test", "--template-html", "<p>alpha</p>", "<p>bravo</p>", "--template-html", "<p>charlie</p>", "<p>delta</p>"], 
        r#" template_html_set: Some({"<p>alpha</p>", "<p>bravo</p>", "<p>charlie</p>", "<p>delta</p>"})"#
    );
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
