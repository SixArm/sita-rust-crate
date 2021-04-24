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

use clap::{Arg, App};
use std::path::PathBuf;
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
    .arg(Arg::new("output_path")
        .short('o')
        .long("output")
        .value_name("FILE")
        .takes_value(true)
        .about("The output file path, such as \"output.html\""))
    .arg(Arg::new("template_file")
        .short('t')
        .long("template")
        .value_name("FILE")
        .takes_value(true)
        .about("The template file, such as \"templates/example.html\""))
    .arg(Arg::new("templates_glob")
        .long("templates")
        .value_name("GLOB")
        .takes_value(true)
        .about("The templates glob, such as \"templates/**/*\""))
    .arg(Arg::new("paths")
        .value_name("FILES")
        .multiple(true))
}
/// Create an Args struct initiatied with the clap App settings.
pub fn args() -> Args {
    let matches = app().get_matches();
    Args {
        output_path: match matches.value_of("output_path") {
            Some(x) => Some(PathBuf::from(x)),
            _ => None,
        },
        paths: match matches.values_of("paths") {
            Some(x) => Some(x.map(|x| PathBuf::from(x)).collect()),
            _ => None,
        },
        template_file: match matches.value_of("template_file") {
            Some(x) => x.into(),
            _ =>  "example.html".into(),
        },
        templates_glob: match matches.value_of("templates_glob") {
            Some(x) => x.into(),
            _ =>  "templates/**/*".into(),
        },
        verbose: std::cmp::max(3, matches.occurrences_of("verbose") as u8),
    }
}
