#[path = "util.rs"]
mod util;
use util::*;

#[test]
pub fn test_01_input() {
    // Prep
    let dir: PathBuf = TESTS_DIR.join("tutorial/01_input");
    let input = dir.join("hello.md");
    let actual = dir.join("hello.html");
    let expect = dir.join("hello.html.expect.html");
    remove_file_if_exists(&actual).expect("remove");
    // Test
    let _output = Command::new(COMMAND)
        .arg("--input")
        .arg(&input)
        .output()
        .expect("failure");
    assert_fn_ok_eq!(
        ::std::fs::read_to_string,
        &actual,
        &expect,
    );
}

#[test]
pub fn test_02_output() {
    // Prep
    let dir: PathBuf = TESTS_DIR.join("tutorial/02_output");
    let input = dir.join("hello.md");
    let actual = dir.join("world.html");
    let expect = dir.join("world.html.expect.html");
    remove_file_if_exists(&actual).expect("remove");
    // Test
    let _output = Command::new(COMMAND)
        .arg("--input")
        .arg(&input)
        .arg("--output")
        .arg(&actual)
        .output()
        .expect("failure");
    assert_fn_ok_eq!(
        ::std::fs::read_to_string,
        &actual,
        &expect,
    );
}

#[test]
pub fn test_03_template() {
    // Prep
    let dir: PathBuf = TESTS_DIR.join("tutorial/03_template");
    let template = dir.join("template.html");
    let input = dir.join("hello.md");
    let actual = dir.join("hello.html");
    let expect = dir.join("hello.html.expect.html");
    remove_file_if_exists(&actual).expect("remove");
    // Test
    let _output = Command::new(COMMAND)
        .arg("--template")
        .arg(&template)
        .arg("--input")
        .arg(&input)
        .output()
        .expect("failure");
    assert_fn_ok_eq!(
        ::std::fs::read_to_string,
        &actual,
        &expect,
    );
}
