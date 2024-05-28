//! Args for the application.
//!
//! These args correspond to the matches in the file `clap.rs`.
//! We have these args in their own file in order to be flexible,
//! such as being able to start our app with other arg parsers.

use std::path::PathBuf;
use crate::types::{list::*, map::*};

//pub const INPUT_FILE_NAME_SELECT_REGEX_AS_STR: &str = "\\.md$";
pub const OUTPUT_FILE_NAME_EXTENSION_AS_STR: &str = "html";

#[derive(Debug)]
pub struct Args {

    /// Input list of path buffers.
    /// Example glob: "articles/**/*"
    /// Example file: "article.md"
    pub(crate) input_list: Option<List<PathBuf>>,

    /// Log level: 0=none, 1=error, 2=warn, 3=info, 4=debug, 5=trace.
    /// Example: 5 means print debug diagnostics.
    pub(crate) log_level: Option<::log::Level>,

    /// Output list of path buffers.
    /// Example directory: "results/"
    /// Example file: "article.html"
    /// TODO: add support for OsStr.
    pub(crate) output_list: Option<List<PathBuf>>,

    /// Output file name extension.
    /// Example: "html" means a HTML file extension.
    /// TODO: add support for OsStr.
    pub(crate) output_file_name_extension: Option<String>,

    /// Settings map for the program.
    /// Example: {"alfa" => "bravo", "charlie" => "delta"}
    /// TODO: add support for OsStr.
    pub(crate) settings: Option<Map<String, String>>,

    /// Template list of path buffers.
    /// Example file: "template.hbs"
    /// Example glob: "templates/**/*"
    /// TODO: add support for OsStr.
    pub(crate) template_list: Option<List<PathBuf>>,

    /// Helper list of path buffers.
    /// Example file: "helper.rhai"
    /// Example glob: "helpers/**/*"
    /// TODO: add support for OsStr.
    pub(crate) helper_list: Option<List<PathBuf>>,

    /// Test flag that sets whether the app prints diagnostics.
    /// Example: true means print diagnostics.
    pub(crate) test: bool,
}

impl ::std::default::Default for Args {
    fn default() -> Self { Self {
        input_list: None,
        log_level: None,
        output_list: None,
        output_file_name_extension: None,
        settings: None,
        template_list: None,
        helper_list: None,
        test: false,
    }}
}
