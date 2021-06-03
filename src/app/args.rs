//! Args for the application.
//!
//! These args correspond to the matches in the file `clap.rs`.
//! We have these args in their own file in order to be flexible,
//! such as being able to start our app with other arg parsers.

use ::std::path::PathBuf;
use ::url::Url;

#[derive(Debug)]
pub struct Args {
    pub(crate) input_file_path: Option<PathBuf>,
    pub(crate) input_directory_path: Option<PathBuf>,
    pub(crate) input_extension: Option<String>,
    pub(crate) output_file_path: Option<PathBuf>,
    pub(crate) output_directory_path: Option<PathBuf>,
    pub(crate) output_extension: Option<String>,
    pub(crate) paths: Option<Vec<PathBuf>>,
    pub(crate) template_name: Option<String>,
    pub(crate) template_files: Option<Vec<PathBuf>>,
    pub(crate) template_glob: Option<PathBuf>,
    pub(crate) template_html: Option<String>,
    pub(crate) log_level: Option<::log::Level>,
}

impl ::std::default::Default for Args {
    fn default() -> Self { Self {
        input_file_path: None,
        input_directory_path: None,
        input_extension: None,
        output_file_path: None,
        output_directory_path: None,
        output_extension: None,
        paths: None,
        template_name: None,
        template_files: None,
        template_glob: None,
        template_html: None,
        log_level: None,
    } }
}
