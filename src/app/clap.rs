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
        .about("Set the verbosity level"))
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
        verbose: std::cmp::max(3, matches.occurrences_of("verbose") as u8),
    }
}
