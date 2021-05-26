use ::std::path::Path;
use ::std::path::PathBuf;
use ::std::process::Command;
use ::std::io::Result;
use ::assertables::*;
use ::lazy_static::lazy_static;

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

pub fn test_with_base_path_and_default_input_expect_actual(base_path: PathBuf) {
    let input = base_path.join("input.md");
    let expect = base_path.join("expect.html");
    let actual = base_path.join("actual.html");
    let _output = Command::new(COMMAND)
        .arg(&input)
        .output()
        .expect("failure");
    assert_fn_ok_eq!(
        ::std::fs::read_to_string,
        &actual,
        &expect,
    );
    remove_file_if_exists(&actual).expect("remove");
}

pub fn test_with_base_path_and_default_input_template_expect_actual(base_path: PathBuf) {
    let input = base_path.join("input.md");
    let _template = base_path.join("template.md"); //TODO use template
    let expect = base_path.join("expect.html");
    let actual = base_path.join("actual.html");
    let _output = Command::new(COMMAND)
        .arg(&input)
        .output()
        .expect("failure");
    assert_fn_ok_eq!(
        ::std::fs::read_to_string,
        &actual,
        &expect,
    );
    remove_file_if_exists(&actual).expect("remove");
}

