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

use clap::{Arg,App,Values};
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
        .about("Set the verbosity level"))
    .arg(Arg::new("templates")
        .short('t')
        .long("templates")
        .value_name("GLOB")
        .takes_value(true)
        .required(true)
        .about("The templates file glob, such as \"templates/**/*\""))
    .arg(Arg::new("paths")
        .value_name("FILES")
        .multiple(true))
}
/// Create an Args struct initiatied with the clap App settings.
pub fn args() -> Args {
    let matches = app().get_matches();
    Args {
        verbose: std::cmp::max(3, matches.occurrences_of("verbose") as u8),
        templates_glob: matches.value_of("templates").unwrap_or_else(||{ "templates/**/*" }).into(),
        paths: matches.values_of("paths")
        .unwrap_or_else(||Values::default())
        .map(|x| PathBuf::from(x)).collect(),
    }
}
