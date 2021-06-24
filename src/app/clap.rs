//! Command line argument parsing (CLAP).
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


use ::clap::{Arg, App};
use ::std::path::PathBuf;
use crate::app::args::Args;
use crate::types::*;

/// Create a clap app.
pub fn app() -> App<'static> {
    App::new("Sita")
    .version("1.0.0")
    .author("Joel Parker Henderson <joel@joelparkerhenderson.com>")
    .about("Sita static site generator")
    .arg(Arg::new("input")
        .short('i')
        .long("input")
        .alias("inputs")
        .value_name("FILE|GLOB …")
        .takes_value(true)
        .multiple(true)
        .about("An input glob string. Example glob: --input \"posts/**/*\" … Example file: --input \"input.html\" …"))
    .arg(Arg::new("input_extension")
        .long("input-extension")
        .value_name("EXTENSION")
        .takes_value(true)
        .about("The input file name extension. Default: \"md\". Example: --input-extension \"md\" …"))
    .arg(Arg::new("language")
        .long("language")
        .value_name("LANGUAGE_ENCODING")
        .takes_value(true)
        .about("The language encoding; Default: \"en\" for English. Example: --input-language \"en\" …"))
    .arg(Arg::new("output_file")
        .short('o')
        .long("output-file")
        .value_name("FILE")
        .takes_value(true)
        .about("The output file. Example: --output-file \"output.html\" …"))
    .arg(Arg::new("output_directory")
        .short('O')
        .long("output-directory")
        .value_name("DIRECTORY")
        .takes_value(true)
        .about("The output directory. Example --output-directory \"~/output/\" …"))
    .arg(Arg::new("output_extension")
        .long("output-extension")
        .value_name("EXTENSION")
        .takes_value(true)
        .about("The output file name extension. Default: \"html\". Example: --output-extension \"html\" …"))
    .arg(Arg::new("script")
        .long("script")
        .value_name("URL …")
        .takes_value(true)
        .multiple(true)
        .about("A script URL to add to the HTML header. Example: --script \"script.js\" …"))
    .arg(Arg::new("stylesheet")
        .long("stylesheet")
        .value_name("URL, ...")
        .takes_value(true)
        .multiple(true)
        .about("A stylesheet URL to add to the HTML header. Example: --stylesheet \"stylesheet.css\" …"))
    .arg(Arg::new("template")
        .short('t')
        .long("template")
        .alias("templates")
        .value_name("FILE|GLOB …")
        .takes_value(true)
        .multiple(true)
        .about("A template glob string. Example glob: --template-glob \"templates/**/*\" … Example file: --template-glob \"template.html\" …"))
    .arg(Arg::new("template_html_set")
        .long("template-html")
        .value_name("HTML …")
        .takes_value(true)
        .multiple(true)
        .about("A template HTML string. Example: --template-html \"<p>{{ content }}</p>\" …"))
    .arg(Arg::new("template_name")
        .long("template-name")
        .value_name("NAME")
        .takes_value(true)
        .multiple(true)
        .about("The template name to use for this rendering. Example: \"--template-name foo\" …"))
    .arg(Arg::new("test")
        .long("test")
        .takes_value(false)
        .about("Print test output for debugging, verifying, tracing, and the like. Example: --test …"))
    .arg(Arg::new("title")
        .long("title")
        .value_name("TEXT")
        .takes_value(true)
        .about("The HTML title. Example: --title \"Welcome\" …"))
    .arg(Arg::new("set")
        .short('s')
        .long("set")
        .value_names(&["NAME", "VALUE"])
        .multiple(true)
        .about("Set a variable name to a value. Example: --set pi \"3.1415\" …"))
    .arg(Arg::new("verbose")
        .short('v')
        .long("verbose")
        .multiple(true)
        .takes_value(false)
        .about("Set the verbosity level: 0=none, 1=error, 2=warn, 3=info, 4=debug, 5=trace. Example: --verbose …"))
}

/// Create an Args struct initiatied with the clap App settings.
pub fn args() -> Args {
    let matches = app().get_matches();
    let mut args = Args {
        input_pathable_string_list: match matches.values_of("input") {
            Some(x) => Some(x.map(|x| PathableString::from(x)).collect::<List<PathableString>>()),
            _ => None,
        },
        input_path_buf_list: None,
        input_extension: match matches.value_of("input_extension") {
            Some(x) => Some(String::from(x)),
            _ => None,
        },
        language: match matches.value_of("language") {
            Some(x) => Some(x.into()),
            _ =>  None,
        },
        output_file_path: match matches.value_of("output_file") {
            Some(x) => Some(PathBuf::from(x)),
            _ => None,
        },
        output_directory_path: match matches.value_of("output_directory") {
            Some(x) => Some(PathBuf::from(x)),
            _ => None,
        },
        output_extension: match matches.value_of("output_extension") {
            Some(x) => Some(String::from(x)),
            _ => None,
        },
        paths: match matches.values_of_os("paths") {
            Some(x) => Some(x.map(|x| PathBuf::from(x)).collect()),
            _ => None,
        },
        script_url_list: match matches.values_of("script") {
            Some(x) => Some(x.map(|x| String::from(x)).collect::<List<UrlString>>()),
            _ => None,
        },
        settings: match matches.values_of("set") {
            Some(x) => {
                let vec: Vec<&str> = x.collect();
                Some(vec_str_to_map_string_string(&vec))
            },
            _ => None,
        },
        stylesheet_url_list: match matches.values_of("stylesheet") {
            Some(x) => Some(x.map(|x| String::from(x)).collect::<List<UrlString>>()),
            _ => None,
        },
        template_pathable_string_list: match matches.values_of("template") {
            Some(x) => Some(x.map(|x| PathableString::from(x)).collect::<List<PathableString>>()),
            _ => None,
        },
        template_path_buf_list: None,
        template_html_set: match matches.values_of("template_html_set") {
            Some(x) => Some(x.map(|x| String::from(x)).collect::<Set<HtmlString>>()),
            _ =>  None,
        },
        template_name: match matches.value_of("template_name") {
            Some(x) => Some(x.into()),
            _ =>  None,
        },
        test: matches.is_present("test"),
        title: match matches.value_of("title") {
            Some(x) => Some(x.into()),
            _ =>  None,
        },
        log_level: match matches.occurrences_of("verbose") {
            0 => None,
            1 => Some(::log::Level::Error),
            2 => Some(::log::Level::Warn),
            3 => Some(::log::Level::Info),
            4 => Some(::log::Level::Debug),
            5 => Some(::log::Level::Trace),
            _ => Some(::log::Level::Trace),
        },
    };

    if let Some(ref x) = args.input_pathable_string_list {
        args.input_path_buf_list = Some(crate::util::pathable_string_list_to_path_buf_list(x));
    }

    if let Some(ref x) = args.template_pathable_string_list {
        args.template_path_buf_list = Some(crate::util::pathable_string_list_to_path_buf_list(x));
    }

    args    
}

pub fn vec_str_to_map_string_string(vec_str: &Vec<&str>) -> Map<String, String> {
    let mut map: Map<String, String> = Map::new();
    for i in (0..vec_str.len()-1).step_by(2) {
        let k = String::from(vec_str[i]);
        let v = String::from(vec_str[i+1]);
        map.insert(k, v);
    }
    map
}
