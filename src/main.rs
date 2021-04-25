//! Main
//!
//!   * `args` - Our arguments struct, set via `clap`.
//!   * `config` - Our configuration struct, set via `confy`.
//!   * `confy` - Tests for `confy` loading and parsing.
//!   * `tera` - Tera template loading and parsing.
//!   * `vars` - Our variables for Tera template context.

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
mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain! { }
}

#[allow(unused_imports)]
use errors::*;

////

#[macro_use]
extern crate log;

////

pub(crate) mod run;

pub(crate) mod app {
    pub(crate) mod args;
    pub(crate) mod clap;
    pub(crate) mod config;
    pub(crate) mod confy;
}
pub(crate) mod markdown {
    pub(crate) mod markdown_parser;
    pub(crate) mod markdown_front_matter;
}
pub(crate) mod templating {
    pub(crate) mod serde;
    pub(crate) mod tera;
    pub(crate) mod vars;
}

fn main() {
    if let Err(ref e) = crate::run::run() {
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
