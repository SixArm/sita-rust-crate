//! Main
//!
//!   * `args` - Our arguments struct, set via `clap`.
//!   * `config` - Our configuration struct, set via `confy`.
//!   * `confy` - Tests for `confy` loading and parsing.
//!   * `tera` - Tera template loading and parsing.

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

//// log

#[macro_use]
extern crate log;
extern crate env_logger;

//// maplit

#[macro_use] 
extern crate maplit;

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
    pub(crate) mod front_matter {
        pub(crate) mod front;
        pub(crate) mod kinds {
            pub(crate) mod html;
            pub(crate) mod json;
            pub(crate) mod toml;
            pub(crate) mod yaml;
        }
    }
}
pub(crate) mod templating {
    pub(crate) mod serde;
    pub(crate) mod tera;
    pub(crate) mod xml;
}

fn main() {
    env_logger::init();
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
