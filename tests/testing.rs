use std::ffi::OsString;
use assertables::*;
use once_cell::sync::Lazy;
use std::path::{Path, PathBuf};

#[allow(dead_code)]
pub static CARGO_MANIFEST_DIR: Lazy<PathBuf> =
    Lazy::new(|| [env!("CARGO_MANIFEST_DIR")].iter().collect::<PathBuf>());

#[allow(dead_code)]
pub static LOG_DIR: Lazy<PathBuf> = Lazy::new(|| {
    [env!("CARGO_MANIFEST_DIR"), "log"]
        .iter()
        .collect::<PathBuf>()
});

#[allow(dead_code)]
pub static TESTS_DIR: Lazy<PathBuf> = Lazy::new(|| {
    [env!("CARGO_MANIFEST_DIR"), "tests"]
        .iter()
        .collect::<PathBuf>()
});

#[allow(dead_code)]
pub static TMP_DIR: Lazy<PathBuf> = Lazy::new(|| {
    [env!("CARGO_MANIFEST_DIR"), "tmp"]
        .iter()
        .collect::<PathBuf>()
});

#[allow(dead_code)]
pub static TARGET_DIR: Lazy<PathBuf> = Lazy::new(|| {
    [env!("CARGO_MANIFEST_DIR"), "target"]
        .iter()
        .collect::<PathBuf>()
});

#[allow(dead_code)]
pub static DEBUG_DIR: Lazy<PathBuf> = Lazy::new(|| {
    [env!("CARGO_MANIFEST_DIR"), "target", "debug"]
        .iter()
        .collect::<PathBuf>()
});

#[allow(dead_code)]
pub static COMMAND_FILE: Lazy<PathBuf> = Lazy::new(|| {
    [env!("CARGO_MANIFEST_DIR"), "target", "debug", "sita"]
        .iter()
        .collect::<PathBuf>()
});

#[allow(dead_code)]
pub static COMMAND_OS: Lazy<OsString> = Lazy::new(|| {
    OsString::from(
        [env!("CARGO_MANIFEST_DIR"), "target", "debug", "sita"]
            .iter()
            .collect::<PathBuf>(),
    )
});

#[allow(dead_code)]
fn assert_str_contains(outer: &str, inner: &str) {
    assert!(
        outer.contains(inner),
        "outer: {:?}\n inner: {}\n",
        &outer,
        &inner
    );
}

#[allow(dead_code)]
pub fn remove_file_if_exists<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    if path.as_ref().exists() {
        std::fs::remove_file(path)
    } else {
        Ok(())
    }
}

#[cfg(test)]
#[allow(dead_code)]
pub fn test_with_base_path_and_default_template_input_output_expect(base_path: &PathBuf) {
    // Prep
    let template = base_path.join("template.html");
    let input = base_path.join("example.md");
    let output = base_path.join("example.html");
    let expect = base_path.join("example.html=expect.html");
    assert_ok!(remove_file_if_exists(&output));
    assert!(input.exists(), "input path: {:?}", input);
    assert!(expect.exists(), "expect path: {:?}", expect);
    assert_ok!(remove_file_if_exists(&output));
    // Test
    assert!(!output.exists(), "output path: {:?}", output);
    let command_result = std::process::Command::new(&*COMMAND_OS)
        .arg("--template")
        .arg(template.as_os_str())
        .arg("--input")
        .arg(input.as_os_str())
        .arg("--output")
        .arg(output.as_os_str())
        .output();
    assert_ok!(command_result);
    let command_output = command_result.unwrap();
    let stdout_string = String::from_utf8(command_output.stdout).unwrap();
    let stderr_string = String::from_utf8(command_output.stderr).unwrap();
    assert_eq!(stdout_string, "");
    assert_eq!(stderr_string, "");
    assert!(output.exists(), "test_with_base_path_and_default_template_input_output_expect ➡ output file must exist ➡ template: {:?}, input: {:?}, output: {:?}, expect: {:?}", template, input, output, expect);
    assert_fs_read_to_string_eq!(&output, &expect);
    // Done
    assert_ok!(remove_file_if_exists(&output));
}

