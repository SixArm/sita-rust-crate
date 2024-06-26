use std::ffi::OsString;
use std::path::{Path, PathBuf};
use once_cell::sync::Lazy;

#[allow(dead_code)]
pub static CARGO_MANIFEST_DIR: Lazy<PathBuf> = Lazy::new(||
    [env!("CARGO_MANIFEST_DIR")].iter().collect::<PathBuf>()
);

#[allow(dead_code)]
pub static LOG_DIR: Lazy<PathBuf> = Lazy::new(||
    [env!("CARGO_MANIFEST_DIR"), "log"].iter().collect::<PathBuf>()
);

#[allow(dead_code)]
pub static TESTS_DIR: Lazy<PathBuf> = Lazy::new(||
    [env!("CARGO_MANIFEST_DIR"), "tests"].iter().collect::<PathBuf>()
);

#[allow(dead_code)]
pub static TMP_DIR: Lazy<PathBuf> = Lazy::new(||
    [env!("CARGO_MANIFEST_DIR"), "tmp"].iter().collect::<PathBuf>()
);

#[allow(dead_code)]
pub static TARGET_DIR: Lazy<PathBuf> = Lazy::new(||
    [env!("CARGO_MANIFEST_DIR"), "target"].iter().collect::<PathBuf>()
);

#[allow(dead_code)]
pub static DEBUG_DIR: Lazy<PathBuf> = Lazy::new(||
    [env!("CARGO_MANIFEST_DIR"), "target", "debug"].iter().collect::<PathBuf>()
);

#[allow(dead_code)]
pub static COMMAND_FILE: Lazy<PathBuf> = Lazy::new(||
    [env!("CARGO_MANIFEST_DIR"), "target", "debug", "sita"].iter().collect::<PathBuf>()
);

#[allow(dead_code)]
pub static COMMAND_OS: Lazy<OsString> = Lazy::new(||
    OsString::from([env!("CARGO_MANIFEST_DIR"), "target", "debug", "sita"].iter().collect::<PathBuf>())
);

#[allow(dead_code)]
fn assert_str_contains(outer: &str, inner: &str) {
    assert!(
        outer.contains(inner),
        "outer: {:?}\n inner: {}\n", &outer, &inner
    );
}

#[allow(dead_code)]
pub fn remove_file_if_exists<P: AsRef<Path>>(path: P) -> std::io::Result<()>
{
    if path.as_ref().exists() {
        std::fs::remove_file(path)
    } else {
        Ok(())
    }
}
