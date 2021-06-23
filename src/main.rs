//! Main

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
pub mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain! { }
}

#[allow(unused_imports)]
pub use errors::*;

//// log
#[macro_use]
extern crate log;
extern crate env_logger;

//// maplit
// #[macro_use]
// extern crate maplit;

////

#[macro_use] pub(crate) mod run; // Main run logic
#[macro_use] pub(crate) mod types; // Type aliases
#[macro_use] pub(crate) mod util; // Generic utilities

pub(crate) mod app { // Application
    pub(crate) mod args; // Arguments struct, such as set via `clap`.
    pub(crate) mod clap; // Command line argument parser
    pub(crate) mod config; // Configuration struct, such as set via `confy`
    pub(crate) mod confy; // Configuration tests for loading and parsing
}
pub(crate) mod markdown {
    pub(crate) mod markdown_parser;
    pub(crate) mod matter {
        pub(crate) mod state;
        pub(crate) mod util;
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
    pub(crate) mod tags;
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
