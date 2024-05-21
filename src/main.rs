//! Main

//// error-chain

// Simple and robust error handling with error-chain!

// `error_chain!` can recurse deeply, so limit it.
#![recursion_limit = "1024"]

// Import the macro. Be sure to add `error-chain` in your `Cargo.toml`.
#[macro_use]
extern crate error_chain;

// We put our errors in an `errors` module, and other modules in
// this crate will `use errors::*;` to get access to everything
// that `error_chain!` creates.
pub mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain! { }
}

#[allow(unused_imports)]
pub use errors::*;

//// log
#[macro_use]
extern crate log;
extern crate env_logger;

//// assertables
//#[macro_use]
extern crate assertables;

//// maplit
// #[macro_use]
// extern crate maplit;

//// Modules

#[macro_use] pub(crate) mod types; // Type aliases
pub(crate) mod util; // Utilties
pub(crate) mod testing; // Test helpers

pub(crate) mod app { // Application
    pub(crate) mod args; // Arguments struct, such as set via `clap`.
    pub(crate) mod clap; // Command line argument parser
    pub(crate) mod config; // Configuration struct, such as set via `confy`
    pub(crate) mod confy; // Configuration tests for loading and parsing
    pub(crate) mod run; // Run function that handles everything
}

pub(crate) mod f { // Functions
    pub(crate) mod dir_entry_is_hidden; // DirEntry is hidden i.e. basename starts with a period.
    pub(crate) mod dir_entry_is_visible; // DirEntry is visible i.e. basename starts with a non-period.
    pub(crate) mod dir_entry_is_in_extension_set; // DirEntry ends with e.g. Markdown file extension "md" or "markdown".
    pub(crate) mod dir_entry_first_with_expect; // Get the first DirEntry from a directory.
    pub(crate) mod from_html_str_into_headline_str; // from HtmlStr into headline str
    pub(crate) mod from_list_pathable_string_into_list_path_buf; // from List<PathableString> into List<PathBuf>
    pub(crate) mod from_list_str_into_map_string_string; // from List<&str> into Map<String, String>
    pub(crate) mod from_input_pathable_string_and_output_pathable_string_into_map; // from input Pathable string and output Pathable string into Map<PathBuf, PathBuf>
    pub(crate) mod from_input_dir_and_output_dir_into_map; // from input directory path and output directory path into Map<PathBuf, PathBuf>
    pub(crate) mod from_path_buf_into_sibling; // from PathBuf into sibling PathBuf
    pub(crate) mod from_pathable_string_into_list_path_buf; // from PathableString into List<PathBuf>
    pub(crate) mod from_set_pathable_string_into_set_path_buf; // from Set<PathableString> into Set<PathBuf>
    pub(crate) mod remove_file_if_exists;
    pub(crate) mod walkdir_dir_entry_is_hidden; // DirEntry is hidden i.e. basename starts with a period.
    pub(crate) mod walkdir_dir_entry_is_visible; // DirEntry is visible i.e. basename starts with a non-period.
    pub(crate) mod walkdir_dir_entry_is_in_extension_set; // DirEntry ends with e.g. Markdown file extension "md" or "markdown".
    pub(crate) mod walkdir_dir_entry_first_with_expect; // Get the first DirEntry from a directory.
}

pub(crate) mod markdown {
    pub(crate) mod markdown_parser;
}

pub(crate) mod matter {
    pub(crate) mod matter_parser_mutex;
    pub(crate) mod matter_parser_trait;
    pub(crate) mod matter_parser_with_html;
    pub(crate) mod matter_parser_with_json;
    pub(crate) mod matter_parser_with_markdown_comments;
    pub(crate) mod matter_parser_with_toml;
    pub(crate) mod matter_parser_with_yaml;
}

pub(crate) mod rewriting {
    pub(crate) mod lol;
}

pub(crate) mod state {
    pub(crate) mod state_enum;
    pub(crate) mod state_trait;
    pub(crate) mod state_with_map;
    pub(crate) mod state_with_json;
    pub(crate) mod state_with_toml;
    pub(crate) mod state_with_yaml;
}

pub(crate) mod templating {
    pub(crate) mod serde;
    pub(crate) mod tags;
}

pub(crate) mod templater {
    pub(crate) mod templater_enum;
    pub(crate) mod templater_trait;
    pub(crate) mod templater_with_handlebars;
    //pub(crate) mod templater_with_liquid;
    pub(crate) mod templater_with_tera;
}

//// Main error-chain
fn main() {
    env_logger::init();
    if let Err(ref e) = crate::app::run::run() {
        println!("error: {}", e);
        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }
        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:?}", backtrace);
        }
        std::process::exit(1);
    }
}
