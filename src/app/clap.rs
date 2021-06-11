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
use ::std::collections::HashMap;
use crate::app::args::Args;
use crate::types::*;

/// Create a clap app.
pub fn app() -> App<'static> {
    App::new("Sita")
    .version("1.0.0")
    .author("Joel Parker Henderson <joel@joelparkerhenderson.com>")
    .about("Sita static site generator")
    .arg(Arg::new("verbose")
        .short('v')
        .long("verbose")
        .multiple(true)
        .takes_value(false)
        .about("Set the verbosity level: 0=none, 1=error, 2=warn, 3=info, 4=debug, 5=trace"))
    .arg(Arg::new("input_file")
        .short('i')
        .long("input-file")
        .value_name("FILE")
        .takes_value(true)
        .about("The input file, such as \"input.html\""))
    .arg(Arg::new("input_directory")
        .short('I')
        .long("input-directory")
        .value_name("DIRECTORY")
        .takes_value(true)
        .about("The input directory, such as \"~/input/\""))
    .arg(Arg::new("input_extension")
        .long("input-extension")
        .value_name("EXTENSION")
        .takes_value(true)
        .about("The input file name extension; default \"md\""))
    .arg(Arg::new("lang")
        .long("lang")
        .value_name("LANGUAGE")
        .takes_value(true)
        .about("The language encoding, such as \"en\" for English"))
    .arg(Arg::new("output_file")
        .short('o')
        .long("output-file")
        .value_name("FILE")
        .takes_value(true)
        .about("The output file, such as \"output.html\""))
    .arg(Arg::new("output_directory")
        .short('O')
        .long("output-directory")
        .value_name("DIRECTORY")
        .takes_value(true)
        .about("The output directory, such as \"~/output/\""))
    .arg(Arg::new("output_extension")
        .long("output-extension")
        .value_name("EXTENSION")
        .takes_value(true)
        .about("The output file name extension; default \"html\""))
    .arg(Arg::new("script")
        .long("script")
        .value_name("URL")
        .takes_value(true)
        .multiple(true)
        .number_of_values(1)
        .about("A script URL to add to the HTML header, such as \"script.js\""))
    .arg(Arg::new("stylesheet")
        .long("stylesheet")
        .value_name("URL")
        .takes_value(true)
        .multiple(true)
        .number_of_values(1)
        .about("A stylesheet URL to add to the HTML header, such as \"stylesheet.css\""))
    .arg(Arg::new("template_name")
        .short('t')
        .long("template-name")
        .value_name("NAME")
        .takes_value(true)
        .about("The template name; for example \"my-template.html\""))
    .arg(Arg::new("template_file")
        .short('T')
        .long("template-file")
        .value_name("FILE")
        .takes_value(true)
        .multiple(true)
        .number_of_values(1)
        .about("The template file; for example \"my-template.html\""))
    .arg(Arg::new("template_glob")
        .long("template-glob")
        .value_name("GLOB")
        .takes_value(true)
        .about("The template glob; for example \"templates/**/*\""))
    .arg(Arg::new("template_html")
        .long("template-html")
        .value_name("HTML")
        .takes_value(true)
        .about("The template HTML; for example \"<p>{{ content }}</p>\""))
    .arg(Arg::new("test")
        .long("test")
        .takes_value(false)
        .about("Print test output for debugging, verifying, tracing, and the like"))
    .arg(Arg::new("title")
        .long("title")
        .value_name("TEXT")
        .takes_value(true)
        .about("The HTML title; for example \"Welcome\""))
    .arg(Arg::new("set")
        .short('s')
        .long("set")
        .value_names(&["NAME", "VALUE"])
        .multiple(true))
    .arg(Arg::new("paths")
        .value_name("FILES")
        .multiple(true))
}

/// Create an Args struct initiatied with the clap App settings.
pub fn args() -> Args {
    let matches = app().get_matches();
    Args {
        input_file_path: match matches.value_of("input_file") {
            Some(x) => Some(PathBuf::from(x)),
            _ => None,
        },
        input_directory_path: match matches.value_of("input_directory") {
            Some(x) => Some(PathBuf::from(x)),
            _ => None,
        },
        input_extension: match matches.value_of("input_extension") {
            Some(x) => Some(String::from(x)),
            _ => None,
        },
        lang: match matches.value_of("lang") {
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
        script_urls: match matches.values_of_os("script") {
            Some(x) => Some(x.map(|x| String::from(x.to_string_lossy())).collect::<Vec<UrlString>>()),
            _ => None,
        },
        settings: match matches.values_of("set") {
            Some(x) => {
                let vec: Vec<&str> = x.collect();
                Some(vec_str_to_hash_map_string_string(&vec))
            },
            _ => None,
        },
        stylesheet_urls: match matches.values_of_os("stylesheet") {
            Some(x) => Some(x.map(|x| String::from(x.to_string_lossy())).collect::<Vec<UrlString>>()),
            _ => None,
        },
        template_name: match matches.value_of("template_name") {
            Some(x) => Some(x.into()),
            _ =>  None,
        },
        template_files: match matches.values_of_os("template_file") {
            Some(x) => Some(x.map(|x| PathBuf::from(x)).collect()),
            _ => None,
        },
        template_glob: match matches.value_of_os("template_glob") {
            Some(x) => Some(x.into()),
            _ =>  None,
        },
        template_html: match matches.value_of("template_html") {
            Some(x) => Some(x.into()),
            _ =>  None,
        },
        test: Some(matches.is_present("input_file")),
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
    }

}

pub fn vec_str_to_hash_map_string_string(vec_str: &Vec<&str>) -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();
    for i in (0..vec_str.len()-1).step_by(2) {
        let k = String::from(vec_str[i]);
        let v = String::from(vec_str[i+1]);
        map.insert(k, v);
    }
    map
}
