//! Args for the application.
//!
//! These args correspond to the matches in the file `clap.rs`.
//! We have these args in their own file in order to be flexible,
//! such as being able to start our app with other arg parsers.

use ::std::path::PathBuf;
use crate::types::*;

#[derive(Debug)]
pub struct Args {

    /// Input pathable list
    /// Example glob: "posts/**/*"
    /// Example file: "posts/example.md"
    pub(crate) input_pathable_list: Option<List<PathableString>>,

    /// Input file name extension.
    /// Example: "md" means a Markdown file extension.
    pub(crate) input_extension: Option<String>,

    /// Language encoding.
    /// Example: "en" means English language encoding.
    pub(crate) language: Option<String>,

    /// Output directory path.
    /// Example: "/tmp/build" is the output directory.
    pub(crate) output_directory_path: Option<PathBuf>,

    /// Output file path.
    /// Example: "/tmp/build/example.html" is the output file.
    pub(crate) output_file_path: Option<PathBuf>,

    /// Output file name extension.
    /// Example: "html" means a HTML file extension.
    pub(crate) output_extension: Option<String>,

    /// Paths that this program will process.
    /// Example: "example.md"
    pub(crate) paths: Option<List<PathBuf>>,

    /// Script URL list.
    /// Example: "https://example.com/script.js" is a JavaScript URL.
    pub(crate) script_url_list: Option<List<UrlString>>,

    /// Settings for the program.
    /// Example: {"alpha" => "bravo", "charlie" => "delta"}
    pub(crate) settings: Option<Map<String, String>>,

    /// Stylesheet URL list.
    /// Example: "https://example.com/stylesheet.css" is a stylesheet URL.
    pub(crate) stylesheet_url_list: Option<List<UrlString>>,

    /// Template name that will be used for rendering.
    /// Example: "default" means use the default template.
    pub(crate) template_name: Option<String>,

    /// Template glob set.
    /// Example: "templates/**/*.html"
    pub(crate) template_glob_set: Option<Set<GlobString>>,

    /// Template HTML set.
    /// Example: "<div>{{ content }}</div>"
    pub(crate) template_html_set: Option<Set<HtmlString>>,

    /// Test flag that enables print diagnostics.
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
        input_pathable_list: None,
        input_extension: None,
        language: None,
        output_file_path: None,
        output_directory_path: None,
        output_extension: None,
        paths: None,
        settings: None,
        script_url_list: None,
        stylesheet_url_list: None,
        template_name: None,
        template_glob_set: None,
        template_html_set: None,
        test: false,
        title: None,
        log_level: None,
    } }
}
