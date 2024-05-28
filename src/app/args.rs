//! Args for the application.
//!
//! These args correspond to the matches in the file `clap.rs`.
//! We have these args in their own file in order to be flexible,
//! such as being able to start our app with other arg parsers.

use std::path::PathBuf;
use crate::types::{list::*, map::*};
use once_cell::sync::Lazy;

//pub const INPUT_FILE_NAME_SELECT_REGEX_AS_STR: &str = "\\.md$";
pub static OUTPUT_FILE_NAME_EXTENSION_AS_PATH_BUF: Lazy<PathBuf> = Lazy::new(|| PathBuf::from(OUTPUT_FILE_NAME_EXTENSION_AS_STR) );
pub const OUTPUT_FILE_NAME_EXTENSION_AS_STR: &str = "html";
pub const FILE_NAME_IS_NONE_AS_STR: &str = "?";

#[derive(Debug)]
pub struct Args {

    /// Input list of path buffers.
    /// Example glob: "articles/**/*"
    /// Example file: "article.md"
    pub(crate) input_list: Option<List<PathBuf>>,

    /// Output list of path buffers.
    /// Example directory: "results/"
    /// Example file: "article.html"
    /// TODO: add support for OsStr.
    pub(crate) output_list: Option<List<PathBuf>>,

    /// Template list of path buffers.
    /// Example file: "template.hbs"
    /// Example glob: "templates/**/*"
    /// TODO: add support for OsStr.
    pub(crate) template_list: Option<List<PathBuf>>,

    /// Extra list of path buffers.
    /// Example file: "script.rhai"
    /// Example glob: "scripts/**/*"
    /// TODO: add support for OsStr.
    pub(crate) extra_list: Option<List<PathBuf>>,

    /// Log level: 0=none, 1=error, 2=warn, 3=info, 4=debug, 5=trace.
    /// Example: 5 means print debug diagnostics.
    pub(crate) log_level: Option<::log::Level>,

    /// Output file name extension.
    /// Example: "html" means a HTML file extension.
    /// TODO: add support for OsStr.
    pub(crate) output_file_name_extension: Option<PathBuf>,

    /// Settings map for the program.
    /// Example: {"alfa" => "bravo", "charlie" => "delta"}
    /// TODO: add support for OsStr.
    #[allow(dead_code)] //TODO live
    pub(crate) settings: Option<Map<String, String>>,

    /// Test flag that sets whether the app prints diagnostics.
    /// Example: true means print diagnostics.
    pub(crate) test: bool,
}

impl std::default::Default for Args {
    fn default() -> Self { Self {
        input_list: None,
        output_list: None,
        template_list: None,
        extra_list: None,
        log_level: None,
        output_file_name_extension: None,
        settings: None,
        test: false,
    }}
}
