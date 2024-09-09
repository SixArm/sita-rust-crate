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
pub fn test_with_base_path_and_default_input_actual_expect(base_path: &PathBuf) {
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
    let _output = std::process::Command::new(&*COMMAND_OS)
        .arg("--input")
        .arg(input.as_os_str())
        .output()
        .expect("failure");
    assert!(actual.exists(), "actual path: {:?}", actual);
    assert_fs_read_to_string_eq!(&actual, &expect);
    // Done
    std::fs::remove_file(&actual).expect("remove");
}

#[cfg(test)]
#[allow(dead_code)]
pub fn test_with_base_path_and_default_input_template_actual_expect(base_path: &PathBuf) {
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
    let _output = std::process::Command::new(&*COMMAND_OS)
        .arg("--input")
        .arg(input.as_os_str())
        .arg("--template")
        .arg(&template)
        .output()
        .expect("failure");
    assert!(actual.exists(), "actual path: {:?}", actual);
    assert_fs_read_to_string_eq!(&actual, &expect);
    // Done
    std::fs::remove_file(&actual).expect("remove");
}
