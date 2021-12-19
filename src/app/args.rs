//! Args for the application.
//!
//! These args correspond to the matches in the file `clap.rs`.
//! We have these args in their own file in order to be flexible,
//! such as being able to start our app with other arg parsers.

use std::path::PathBuf;
use crate::types::*;

#[derive(Debug)]
pub struct Args {

    /// Input pathable string list.
    /// Example glob: "articles/**/*"
    /// Example file: "article.md"
    pub(crate) input_list_pathable_string: Option<List<PathableString>>,

    /// Input file path list.
    /// This is typically calculated from `input_list_pathable_string`.
    /// Each item points to a file, not a directory, glob, etc.
    pub(crate) input_list_path_buf: Option<List<PathBuf>>,

    /// Input file name extension.
    /// Example: "md" means a Markdown file extension.
    pub(crate) input_file_name_extension: Option<String>,

    /// Language encoding.
    /// Example: "en" means English language encoding.
    pub(crate) language: Option<String>,

    /// Output pathable string list.
    /// Example directory: "results/"
    /// Example file: "article.html"
    pub(crate) output_list_pathable_string: Option<List<PathableString>>,

    /// Output path list.
    /// This is typically calculated from `output_list_pathable_string`.
    /// Each item points to a file, not a directory, glob, etc.
    pub(crate) output_list_path_buf: Option<List<PathBuf>>,

    /// Output file name extension.
    /// Example: "html" means a HTML file extension.
    pub(crate) output_file_name_extension: Option<String>,

    /// Paths that this program will process.
    /// Example: "example.md"
    pub(crate) paths: Option<List<PathBuf>>,

    /// Script URL list.
    /// Example: "https://example.com/script.js" is a JavaScript URL.
    pub(crate) script_url_list: Option<List<UrlString>>,

    /// Settings for the program.
    /// Example: {"alpha" => "bravo", "charlie" => "delta"}
    pub(crate) settings: Option<Map<String, String>>,

    /// Template name that will be used for rendering.
    /// Example: "default" means use the default template.
    pub(crate) template_name: Option<String>,

    /// Template pathable string list.
    /// Example glob: "templates/**/*"
    /// Example file: "template.html"
    pub(crate) template_list_pathable_string: Option<List<PathableString>>,

    /// Template file path buf list.
    /// This is typically calculated from `template_list_pathable_string`.
    /// Each item points to a file, not a directory, glob, etc.
    pub(crate) template_list_path_buf: Option<List<PathBuf>>,

    /// Template HTML set.
    /// Example: "<div>{{ content }}</div>"
    pub(crate) template_html_set: Option<Set<HtmlString>>,

    /// Test flag that sets whether the app prints diagnostics.
    /// Example: true means print diagnostics.
    pub(crate) test: bool,

    /// Title of the page being rendered.
    /// Example: "My Page"
    pub(crate) title: Option<String>,

    /// Log level: 0=none, 1=error, 2=warn, 3=info, 4=trace, 5=debug.
    /// Example: 5 means print debug diagnostics.
    pub(crate) log_level: Option<::log::Level>,
}

impl ::std::default::Default for Args {
    fn default() -> Self { Self {
        input_list_pathable_string: None,
        input_list_path_buf: None,
        input_file_name_extension: None,
        language: None,
        output_list_pathable_string: None,
        output_list_path_buf: None,
        output_file_name_extension: None,
        paths: None,
        settings: None,
        script_url_list: None,
        template_list_pathable_string: None,
        template_list_path_buf: None,
        template_name: None,
        template_html_set: None,
        test: false,
        title: None,
        log_level: None,
    } }
}
