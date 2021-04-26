use ::std::process::Command;
use ::std::path::PathBuf;

const SITA: &str = "./target/debug/sita";

fn fab_tests_files_path(s: &str) -> PathBuf {
    [env!("CARGO_MANIFEST_DIR"), "tests", "files", s].iter().collect::<PathBuf>()
}

fn fab_tmp_path(s: &str) -> PathBuf {
    [env!("CARGO_MANIFEST_DIR"), "tmp", s].iter().collect::<PathBuf>()
}

fn remove_file_if_exists(path: &PathBuf) {
    if path.exists() {
        ::std::fs::remove_file(path).expect("remove");
    }
}

fn assert_file_string_eq(a: &PathBuf, b: &PathBuf) {
    let a_string = ::std::fs::read_to_string(a).expect("a");
    let b_string = ::std::fs::read_to_string(b).expect("b");
    assert_eq!(a_string, b_string);
}

#[test]
fn test_command_x_default() {
    // Prep input
    let input_path = fab_tmp_path("test.md");
    ::std::fs::copy(
        &fab_tests_files_path("input_1.md"), 
        &input_path
    ).expect("prep input");
    // Prep output
    let output_path = fab_tmp_path("test.html");
    remove_file_if_exists(&output_path);
    // Run
    let _output = Command::new(SITA)
        .arg(&input_path)
        .output()
        .expect("failure");
    assert_fn_eq(
        ::std::fs::read_to_string,
        &output_path,
        &fab_tests_files_path("output_1_template_1.html"),
    );
    // Done
    remove_file_if_exists(&input_path);
    remove_file_if_exists(&output_path);
}

#[test]
fn test_command_x_output_file() {
    // Prep input
    let input_path = fab_tmp_path("test.md");
    ::std::fs::copy(
        &fab_tests_files_path("input_1.md"), 
        &input_path
    ).expect("prep input");
    // Prep output
    let output_path = fab_tmp_path("test-output-file.html");
    remove_file_if_exists(&output_path);
    // Run
    let _output = Command::new(SITA)
        .arg(&input_path)
        .arg("--output-file")
        .arg(&output_path)
        .output()
        .expect("failure");
    assert_file_string_eq(
        &output_path,
        &fab_tests_files_path("output_1_template_1.html"),
    );
    // Done
    remove_file_if_exists(&input_path);
    remove_file_if_exists(&output_path);
}

#[test]
fn test_command_x_template_name() {
    // Prep input
    let input_path = fab_tmp_path("test.md");
    ::std::fs::copy(
        &fab_tests_files_path("input_1.md"), 
        &input_path
    ).expect("prep input");
    // Prep output
    let output_path = fab_tmp_path("test.html");
    remove_file_if_exists(&output_path);
    // Prep template
    let template_path = fab_tmp_path("test-template.html");
    ::std::fs::copy(
        &fab_tests_files_path("template_2.md"), 
        &input_path
    ).expect("prep template");
    // Run
    let _output = Command::new(SITA)
        .arg(&input_path)
        .arg("--template-name")
        .arg(&template_path)
        .output()
        .expect("failure");
    assert_file_string_eq(
        &output_path,
        &fab_tests_files_path("output_1_template_2.html"),
    );
    // Done
    remove_file_if_exists(&input_path);
    remove_file_if_exists(&output_path);
    remove_file_if_exists(&template_path);
}
