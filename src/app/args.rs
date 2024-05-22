//! Args for the application.
//!
//! These args correspond to the matches in the file `clap.rs`.
//! We have these args in their own file in order to be flexible,
//! such as being able to start our app with other arg parsers.

use crate::types::{list::*, map::*, pathable::*};

//pub const INPUT_FILE_NAME_SELECT_REGEX_AS_STR: &str = "\\.md$";
pub const OUTPUT_FILE_NAME_EXTENSION_AS_STR: &str = "html";

#[derive(Debug)]
pub struct Args {

    /// Input pathable string list.
    /// Example glob: "articles/**/*"
    /// Example file: "article.md"
    pub(crate) input_list_pathable_string: Option<List<PathableString>>,

    /// Log level: 0=none, 1=error, 2=warn, 3=info, 4=debug, 5=trace.
    /// Example: 5 means print debug diagnostics.
    pub(crate) log_level: Option<::log::Level>,

    /// Output pathable string list.
    /// Example directory: "results/"
    /// Example file: "article.html"
    pub(crate) output_list_pathable_string: Option<List<PathableString>>,

    /// Output file name extension.
    /// Example: "html" means a HTML file extension.
    pub(crate) output_file_name_extension: Option<String>,

    /// Settings map for the program.
    /// Example: {"alpha" => "bravo", "charlie" => "delta"}
    pub(crate) settings: Option<Map<String, String>>,

    /// Template pathable string list.
    /// Example file: "template.hbs"
    /// Example glob: "templates/**/*"
    pub(crate) template_list_pathable_string: Option<List<PathableString>>,

    /// Helper pathable string list.
    /// Example file: "helper.rhai"
    /// Example glob: "helpers/**/*"
    pub(crate) helper_list_pathable_string: Option<List<PathableString>>,

    /// Test flag that sets whether the app prints diagnostics.
    /// Example: true means print diagnostics.
    pub(crate) test: bool,
}

impl ::std::default::Default for Args {
    fn default() -> Self { Self {
        input_list_pathable_string: None,
        log_level: None,
        output_list_pathable_string: None,
        output_file_name_extension: None,
        settings: None,
        template_list_pathable_string: None,
        helper_list_pathable_string: None,
        test: false,
    }}
}
