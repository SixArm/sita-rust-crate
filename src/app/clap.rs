//! Command line argument parsing (CLAP) for the application.
//!
//! clap is a crate for command line argument parsing.
//! See https://docs.rs/clap/
//!
//! Clap has a variety of setup approachs:
//!
//!   * via typical functions, which favors advanced uses yet is verbose.
//!   * via usage strings, which looks more like writing documentation.
//!   * via macros, which is fast and less verbose, yet atypical to read.
//!   * via YAML file, which favors localization and text file readability.
//!
//! We prefer the typical functions, because they provide maximum capability,
//! and in our experience are the easiest for Rust IDEs to read and debug.
//!
//! We favor our convention of doing clap setup in a file named `clap.rs`,
//! rather than in `main.rs`, because we like the separation of concerns.


use clap::{Arg, App};
use crate::app::args::Args;
use crate::types::*;

/// Create a clap app.
pub fn app() -> App<'static> {
    trace!("clap::app");
    App::new("Sita")
    .version("1.0.0")
    .author("Joel Parker Henderson <joel@joelparkerhenderson.com>")
    .arg(Arg::new("input")
        .short('i')
        .long("input")
        .alias("inputs")
        .value_name("FILE | DIRECTORY | GLOB")
        .takes_value(true)
        .multiple_occurrences(true)
        .multiple_values(true)
        .help("An input path string.\nExample file: --input \"example.html\" …\nExample directory: --input \"examples/\" …\nExample glob: --input \"examples/**/*\" …"))
    // .arg(Arg::new("input_file_name_select_regex_string")
    //     .long("input-select")
    //     .value_name("REGEX")
    //     .takes_value(true)
    //     .help("The input file name select filter regular expression.\nExample: --input-select: \"^foo\" (starts with \"foo\")\nExample: --input-select \"md$\" (ends with \"md\")"))
    .arg(Arg::new("output")
        .short('o')
        .long("output")
        .alias("outputs")
        .value_name("FILE | DIRECTORY | GLOB")
        .takes_value(true)
        .multiple_occurrences(true)
        .multiple_values(true)
        .help("An output path string.\nExample file: --output \"example.html\" …\nExample directory: --output \"examples/\" …\nExample glob: --output \"examples/**/*\" …"))
    .arg(Arg::new("output_file_name_extension")
        .long("output-extension")
        .value_name("EXTENSION")
        .takes_value(true)
        .help("The output file name extension.\nDefault: \"html\".\nExample: --output-extension \"html\""))
    // .arg(Arg::new("script")
    //     .long("script")
    //     .value_name("URL …")
    //     .takes_value(true)
    //     .multiple_occurrences(true)
    //     .multiple_values(true)
    //     .help("A script URL to add to the HTML header.\nExample: --script \"script.js\" …"))
    .arg(Arg::new("template")
        .short('t')
        .long("template")
        .alias("templates")
        .value_name("FILE | DIRECTORY | GLOB")
        .takes_value(true)
        .multiple_occurrences(true)
        .multiple_values(true)
        .help("A template path string.\nExample file: --template \"example.html\" …\nExample directory: --template \"examples/\" …\nExample glob: --template \"examples/**/*\" …"))
    .arg(Arg::new("test")
        .long("test")
        .takes_value(false)
        .help("Print test output for debugging, verifying, tracing, and the like.\nExample: --test"))
    // .arg(Arg::new("set")
    //     .short('s')
    //     .long("set")
    //     .value_names(&["NAME", "VALUE"])
    //     .takes_value(true)
    //     .multiple_occurrences(true)
    //     .multiple_values(true)
    //     .help("Set a variable name to a value.\nExample: --set pi \"3.1415\" …"))
    // .arg(Arg::new("verbose")
    //     .short('v')
    //     .long("verbose")
    //     .takes_value(false)
    //     .multiple_occurrences(true)
    //     .help("Set the verbosity level: 0=none, 1=error, 2=warn, 3=info, 4=debug, 5=trace.\nExample: --verbose …"))
}

/// Create an Args struct initiatied with the clap App settings.
pub fn args() -> Args {
    trace!("clap::args");
    let matches = app().get_matches();
    trace!("clap::args matches: {:?}", matches);

    let input_list_pathable_string = match matches.values_of("input") {
        Some(x) => Some(x.map(|x| PathableString::from(x)).collect::<List<PathableString>>()),
        _ => None,
    };

    // let input_file_name_select_regex_string = match matches.value_of("input_file_name_select_regex_string") {
    //     Some(x) => Some(String::from(x)),
    //     _ => None,
    // };

    let output_list_pathable_string = match matches.values_of("output") {
        Some(x) => Some(x.map(|x| PathableString::from(x)).collect::<List<PathableString>>()),
        _ => None,
    };

    let output_file_name_extension = match matches.value_of("output_file_name_extension") {
        Some(x) => Some(String::from(x)),
        _ => None,
    };

    // let script_url_list = match matches.values_of("script") {
    //     Some(x) => Some(x.map(|x| String::from(x)).collect::<List<UrlString>>()),
    //     _ => None,
    // };

    // let settings = match matches.values_of("set") {
    //     Some(x) => {
    //         let vec: Vec<&str> = x.collect();
    //         Some(from_list_str_into_map_string_string(&vec))
    //     },
    //     _ => None,
    // };

    // let log_level = match matches.occurrences_of("verbose") {
    //     0 => None,
    //     1 => Some(::log::Level::Error),
    //     2 => Some(::log::Level::Warn),
    //     3 => Some(::log::Level::Info),
    //     4 => Some(::log::Level::Debug),
    //     5 => Some(::log::Level::Trace),
    //     _ => Some(::log::Level::Trace),
    // };

    let template_list_pathable_string = match matches.values_of("template") {
        Some(x) => Some(x.map(|x| PathableString::from(x)).collect::<List<PathableString>>()),
        _ => None,
    };

    let test = matches.is_present("test");

    let args = Args {
        input_list_pathable_string: input_list_pathable_string,
        //input_file_name_select_regex_string: input_file_name_select_regex_string,
        output_list_pathable_string: output_list_pathable_string,
        output_file_name_extension: output_file_name_extension,
        // script_url_list: script_url_list,
        // settings: settings,
        template_list_pathable_string: template_list_pathable_string,
        test: test,
        // log_level: log_level,
    };

    trace!("clap::args -> {:?}", args);
    args    
}

