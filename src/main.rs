//! Main

//// error-chain

// Simple and robust error handling with error-chain!

// `error_chain!` can recurse deeply, so limit it.
#![recursion_limit = "1024"]

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

pub(crate) mod testing; // Test helpers

pub(crate) mod app { // Application
    pub(crate) mod args; // Arguments struct, such as set via `clap`.
    pub(crate) mod clap; // Command line argument parser
    pub(crate) mod config; // Configuration struct, such as set via `confy`
    pub(crate) mod confy; // Configuration tests for loading and parsing
    pub(crate) mod run; // Run function that handles everything
}

pub(crate) mod f { // Functions
    pub(crate) mod from_html_str_into_headline_str; // from HtmlStr into headline str
    pub(crate) mod from_list_pathable_string_into_list_path_buf; // from List<PathableString> into List<PathBuf>
    pub(crate) mod from_list_str_into_map_string_string; // from List<&str> into Map<String, String>
    pub(crate) mod from_input_dir_and_output_dir_into_map; // from input dir and output dir into Map<PathBuf, PathBuf>
    pub(crate) mod from_input_path_buf_and_output_path_buf_into_map; // from input path buffer and output path buffer into Map<PathBuf, PathBuf>
    pub(crate) mod from_path_buf_into_sibling_extension; // from PathBuf into sibling PathBuf
    pub(crate) mod from_pathable_string_into_list_path_buf; // from PathableString into List<PathBuf>
    pub(crate) mod from_set_pathable_string_into_set_path_buf; // from Set<PathableString> into Set<PathBuf>
    pub(crate) mod remove_file_if_exists;
    pub(crate) mod vet_input_file_path_buf_exists; // Vet an input file PathBuf exists.
    pub(crate) mod vet_input_file_path_buf_metadata; // Vet an input file PathBuf.metadata() exists.
    pub(crate) mod walkdir_prefer_iter;
    pub(crate) mod walkdir_dir_entry_first_with_expect;
    pub(crate) mod walkdir_dir_entry_is_in_extension_set; // DirEntry ends with e.g. Markdown file extension "md" or "markdown".
    pub(crate) mod walkdir_dir_entry_is_visible;
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
    //pub(crate) mod templater_enum;
    pub(crate) mod templater_trait;
    pub(crate) mod templater_with_handlebars;
    //pub(crate) mod templater_with_liquid;
    //pub(crate) mod templater_with_tera;
}

fn main() {
    env_logger::init();
    match crate::app::run::run() {
        Ok(()) => {
            std::process::exit(0);
        }
        Err(err) => {
            error!("{:?}", err);
            std::process::exit(1);
        }
    }
}
