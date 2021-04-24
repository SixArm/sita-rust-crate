//! Args for the application.
//!
//! These args correspond to the matches in the file `clap.rs`.
//! We have these args in their own file in order to be flexible,
//! such as being able to start our app with other arg parsers.

use std::default::Default;
use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct Args {
    pub(crate) output_path: Option<PathBuf>,
    pub(crate) paths: Option<Vec<PathBuf>>,
    pub(crate) template_file: String,
    pub(crate) templates_glob: String,
    pub(crate) verbose: u8,
}
