//! Args for the application.
//!
//! These args correspond to the matches in the file `clap.rs`.
//! We have these args in their own file in order to be flexible,
//! such as being able to start our app with other arg parsers.

use std::default::Default;
use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct Args {
    pub(crate) verbose: u8,
    pub(crate) templates_glob: String,
    pub(crate) paths: Vec<PathBuf>,
}
