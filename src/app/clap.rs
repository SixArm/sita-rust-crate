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
use std::path::PathBuf;
use crate::app::args::Args;
use crate::types::*;
use crate::fun::from_list_pathable_string_into_list_path_buf::*;
use crate::fun::from_list_str_into_map_string_string::*;

/// Create a clap app.
pub fn app() -> App<'static> {
    trace!("clap::app");
    App::new("Sita")
    .version("1.0.0")
    .author("Joel Parker Henderson <joel@joelparkerhenderson.com>")
    .help("Sita static site generator")
    .arg(Arg::new("input")
        .short('i')
        .long("input")
        .alias("inputs")
        .value_name("FILE | DIRECTORY | GLOB")
        .takes_value(true)
        .multiple_occurrences(true)
        .multiple_values(true)
        .help("An input path string. Example file: --input \"input.html\" … Example directory: --input \"inputs/\" … Example glob: --input \"inputs/**/*\" …"))
    .arg(Arg::new("input_file_name_extension")
        .long("input-extension")
        .value_name("EXTENSION")
        .takes_value(true)
        .help("The input file name extension. Default: \"md\". Example: --input-extension \"md\" …"))
    .arg(Arg::new("language")
        .long("language")
        .value_name("LANGUAGE_ENCODING")
        .takes_value(true)
        .help("The language encoding; Default: \"en\" for English. Example: --language \"en\""))
    .arg(Arg::new("output")
        .short('o')
        .long("output")
        .alias("outputs")
        .value_name("FILE | DIRECTORY | GLOB")
        .takes_value(true)
        .multiple_occurrences(true)
        .multiple_values(true)
        .help("An output path string. Example file: --output \"output.html\" … Example directory: --output \"outputs/\" … Example glob: --output \"outputs/**/*\" …"))
    .arg(Arg::new("output_file_name_extension")
        .long("output-extension")
        .value_name("EXTENSION")
        .takes_value(true)
        .help("The output file name extension. Default: \"html\". Example: --output-extension \"html\" …"))
    .arg(Arg::new("script")
        .long("script")
        .value_name("URL …")
        .takes_value(true)
        .multiple_occurrences(true)
        .multiple_values(true)
        .help("A script URL to add to the HTML header. Example: --script \"script.js\" …"))
    .arg(Arg::new("template")
        .short('t')
        .long("template")
        .alias("templates")
        .value_name("FILE | DIRECTORY | GLOB")
        .takes_value(true)
        .multiple_occurrences(true)
        .multiple_values(true)
        .help("A template path string. Example file: --template \"template.html\" … Example directory: --template \"templates/\" … Example glob: --template \"templates/**/*\" …"))
    .arg(Arg::new("template_name")
        .long("template-name")
        .value_name("NAME")
        .takes_value(true)
        .multiple_occurrences(true)
        .multiple_values(true)
        .help("The template name to use for this rendering. Example: \"--template-name foo\" …"))
    .arg(Arg::new("test")
        .long("test")
        .takes_value(false)
        .help("Print test output for debugging, verifying, tracing, and the like. Example: --test …"))
    .arg(Arg::new("title")
        .long("title")
        .value_name("TEXT")
        .takes_value(true)
        .help("The HTML title. Example: --title \"Welcome\" …"))
    .arg(Arg::new("set")
        .short('s')
        .long("set")
        .value_names(&["NAME", "VALUE"])
        .takes_value(true)
        .multiple_occurrences(true)
        .multiple_values(true)
        .help("Set a variable name to a value. Example: --set pi \"3.1415\" …"))
    .arg(Arg::new("verbose")
        .short('v')
        .long("verbose")
        .takes_value(false)
        .multiple_occurrences(true)
        .help("Set the verbosity level: 0=none, 1=error, 2=warn, 3=info, 4=debug, 5=trace. Example: --verbose …"))
}

/// Create an Args struct initiatied with the clap App settings.
pub fn args() -> Args {
    trace!("clap::args");
    let matches = app().get_matches();
    trace!("clap::args matches={:?}", matches);

    let input_list_pathable_string = match matches.values_of("input") {
        Some(x) => Some(x.map(|x| PathableString::from(x)).collect::<List<PathableString>>()),
        _ => None,
    };

    let input_file_name_extension = match matches.value_of("input_file_name_extension") {
        Some(x) => Some(String::from(x)),
        _ => None,
    };

    let language = match matches.value_of("language") {
        Some(x) => Some(x.into()),
        _ =>  None,
    };

    let output_list_pathable_string = match matches.values_of("output") {
        Some(x) => Some(x.map(|x| PathableString::from(x)).collect::<List<PathableString>>()),
        _ => None,
    };

    let output_file_name_extension = match matches.value_of("output_file_name_extension") {
        Some(x) => Some(String::from(x)),
        _ => None,
    };

    let script_url_list = match matches.values_of("script") {
        Some(x) => Some(x.map(|x| String::from(x)).collect::<List<UrlString>>()),
        _ => None,
    };

    let settings = match matches.values_of("set") {
        Some(x) => {
            let vec: Vec<&str> = x.collect();
            Some(from_list_str_into_map_string_string(&vec))
        },
        _ => None,
    };

    let log_level = match matches.occurrences_of("verbose") {
        0 => None,
        1 => Some(::log::Level::Error),
        2 => Some(::log::Level::Warn),
        3 => Some(::log::Level::Info),
        4 => Some(::log::Level::Debug),
        5 => Some(::log::Level::Trace),
        _ => Some(::log::Level::Trace),
    };

    let template_list_pathable_string = match matches.values_of("template") {
        Some(x) => Some(x.map(|x| PathableString::from(x)).collect::<List<PathableString>>()),
        _ => None,
    };

    let template_name = match matches.value_of("template_name") {
        Some(x) => Some(x.into()),
        _ =>  None,
    };

    let test = matches.is_present("test");

    let title = match matches.value_of("title") {
        Some(x) => Some(x.into()),
        _ =>  None,
    };

    let mut args = Args {
        input_list_pathable_string: input_list_pathable_string,
        input_list_path_buf: None, // Set below
        input_file_name_extension: input_file_name_extension,
        language: language,
        output_list_pathable_string: output_list_pathable_string,
        output_list_path_buf: None, // Set below
        output_file_name_extension: output_file_name_extension,
        script_url_list: script_url_list,
        settings: settings,
        template_list_pathable_string: template_list_pathable_string,
        template_list_path_buf: None, // Set below
        template_name: template_name,
        test: test,
        title: title,
        log_level: log_level,
    };

    trace!("clap::args -> {:?}", args);

    if let Some(ref x) = args.input_list_pathable_string {
        args.input_list_path_buf = Some(from_list_pathable_string_into_list_path_buf(x));
    }

    if let Some(ref x) = args.output_list_pathable_string {
        args.output_list_path_buf = Some(from_list_pathable_string_into_list_path_buf(x));
    }

    if let Some(ref x) = args.template_list_pathable_string {
        args.template_list_path_buf = Some(from_list_pathable_string_into_list_path_buf(x));
    }

    trace!("clap::args -> {:?}", args);
    args    
}

