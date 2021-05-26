use ::assertables::*;
use ::std::process::Command;
use ::std::path::PathBuf;

#[path = "util.rs"]
mod util;
use util::*;

#[test]
fn test_command_x_default() {
    // Prep input
    let input_path = TMP_DIR.join("test.md");
    ::std::fs::copy(
        TESTS_DIR.join("input_1.md"), 
        &input_path
    ).expect("prep input");
    // Prep output
    let output_path = TMP_DIR.join("test.html");
    remove_file_if_exists(&output_path).unwrap();
    // Run
    let _output = Command::new(COMMAND)
        .arg(&input_path)
        .output()
        .expect("failure");
    assert_fn_ok_eq!(
        ::std::fs::read_to_string,
        &output_path,
        TESTS_DIR.join("output_1_template_1.html"),
    );
    // Done
    remove_file_if_exists(&input_path).unwrap();
    remove_file_if_exists(&output_path).unwrap();
}

#[test]
fn test_command_x_output_file() {
    // Prep input
    let input_path = TMP_DIR.join("test.md");
    ::std::fs::copy(
        TESTS_DIR.join("input_1.md"), 
        &input_path
    ).expect("prep input");
    // Prep output
    let output_path = TMP_DIR.join("test-output-file.html");
    remove_file_if_exists(&output_path).unwrap();
    // Run
    let _output = Command::new(COMMAND)
        .arg(&input_path)
        .arg("--output-file")
        .arg(&output_path)
        .output()
        .expect("failure");
    assert_fn_ok_eq!(
        ::std::fs::read_to_string,
        &output_path,
        TESTS_DIR.join("output_1_template_1.html"),
    );
    // Done
    remove_file_if_exists(&input_path).unwrap();
    remove_file_if_exists(&output_path).unwrap();
}

#[test]
fn test_command_x_template_name() {
    // Prep input
    let input_path = TMP_DIR.join("test.md");
    ::std::fs::copy(
        TESTS_DIR.join("input_1.md"), 
        &input_path
    ).expect("prep input");
    // Prep output
    let output_path = TMP_DIR.join("test.html");
    remove_file_if_exists(&output_path).unwrap();
    // Prep template
    let template_path = TMP_DIR.join("test-template.html");
    ::std::fs::copy(
        TESTS_DIR.join("template_2.md"), 
        &input_path
    ).expect("prep template");
    // Run
    let _output = Command::new(COMMAND)
        .arg(&input_path)
        .arg("--template-name")
        .arg(&template_path)
        .output()
        .expect("failure");
    assert_fn_ok_eq!(
        ::std::fs::read_to_string,
        &output_path,
        TESTS_DIR.join("output_1_template_2.html"),
    );
    // Done
    remove_file_if_exists(&input_path).unwrap();
    remove_file_if_exists(&output_path).unwrap();
    remove_file_if_exists(&template_path).unwrap();
}
