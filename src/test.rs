use ::std::path::PathBuf;
use ::lazy_static::lazy_static;

lazy_static! {
    pub static ref TESTS_DIR: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests"].iter().collect::<PathBuf>();
}

lazy_static! {
    pub static ref TMP_DIR: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tmp"].iter().collect::<PathBuf>();
}
