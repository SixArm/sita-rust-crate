use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::io::Result;
use assertables::*;
use lazy_static::lazy_static;

pub const COMMAND: &str = "./target/debug/sita";

lazy_static! {
    pub static ref COMMAND_FILE: PathBuf = [env!("CARGO_MANIFEST_DIR"), "target", "debug", "sita"].iter().collect::<PathBuf>();
}

lazy_static! {
    pub static ref TESTS_DIR: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests"].iter().collect::<PathBuf>();
}

lazy_static! {
    pub static ref TMP_DIR: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tmp"].iter().collect::<PathBuf>();
}

#[allow(dead_code)]
pub fn remove_file_if_exists<P: AsRef<Path>>(path: P) -> Result<()>
{
    if path.as_ref().exists() {
        ::std::fs::remove_file(path)
    } else {
        Ok(())
    }
}


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

#[cfg(test)]
#[allow(dead_code)]
pub fn test_with_base_path_and_default_input_actual_expect(base_path: PathBuf) {
    // Prep
    let input = base_path.join("example.md");
    let actual = base_path.join("example.html");
    let expect = base_path.join("example.html=expect.html");
    remove_file_if_exists(&actual).expect("remove");
    assert!(input.exists(), "input path: {:?}", input);
    assert!(expect.exists(), "expect path: {:?}", expect);
    remove_file_if_exists(&actual).expect("remove");
    // Test
    assert!(!actual.exists(), "actual path: {:?}", actual);
    let _output = Command::new(COMMAND)
        .arg("--input")
        .arg(input.as_os_str())
        .output()
        .expect("failure");
    assert!(actual.exists(), "actual path: {:?}", actual);
        assert_fn_ok_eq!(
        ::std::fs::read_to_string,
        &actual,
        &expect,
    );
    // Done
    ::std::fs::remove_file(&actual).expect("remove");
}

#[cfg(test)]
#[allow(dead_code)]
pub fn test_with_base_path_and_default_input_template_actual_expect(base_path: PathBuf) {
    // Prep
    let input = base_path.join("example.md");
    let template = base_path.join("template.html");
    let actual = base_path.join("example.html");
    let expect = base_path.join("example.html=expect.html");
    assert!(input.exists(), "input path: {:?}", input);
    assert!(template.exists(), "template path: {:?}", template);
    assert!(expect.exists(), "expect path: {:?}", expect);
    remove_file_if_exists(&actual).expect("remove");
    // Test
    assert!(!actual.exists(), "actual path: {:?}", actual);
    let _output = Command::new(COMMAND)
        .arg("--input")
        .arg(input.as_os_str())
        .arg("--template")
        .arg(&template)
        .output()
        .expect("failure");
    assert!(actual.exists(), "actual path: {:?}", actual);
    assert_fn_ok_eq!(
        ::std::fs::read_to_string,
        &actual,
        &expect,
    );
    // Done
    ::std::fs::remove_file(&actual).expect("remove");
}
