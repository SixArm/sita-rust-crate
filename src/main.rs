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

use errors::*;

////

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

use ::tera::Tera;
use ::std::path::PathBuf;
use ::std::ffi::OsStr;
use crate::app::args::Args;
use crate::app::config::Config;
use crate::errors::*;
use crate::templating::vars::Vars;

fn main() {
    if let Err(ref e) = run() {
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

fn run() -> Result<()> {
    let _config: Config = ::confy::load("sita")
        .chain_err(|| "configuration load error")?;
    let args: Args = crate::app::clap::args();
    let tera: Tera = crate::templating::tera::init(&*args.template_glob);
    let vars = Vars {
        title: Some("my title".into()),
        content: Some("my content".into()),
    };
    if let Some(paths) = args.paths {
        for input_path in paths {
            do_path(&tera, "example.html", &vars, &input_path)?;
        }
    };
    Ok(())
}

fn do_path(tera: &Tera, template: &str, vars: &Vars, input_path: &PathBuf) -> Result<()> {
    let os_md = OsStr::new("md");
    let extension = input_path.extension();
    if extension != Some(&os_md) {
        return Ok(());
    }
    let mut output_path = PathBuf::from(input_path);
    output_path.set_extension("html");
    let input_markdown = ::std::fs::read_to_string(input_path)
        .chain_err(|| "read input")?;
    let context = ::tera::Context::from_serialize(&vars)
        .chain_err(|| "create context")?;
    let output_html = tera.render(template, &context)
        .chain_err(|| "render template")?;
    ::std::fs::write(output_path, output_html)
        .chain_err(|| "write output")?;
    Ok(())
}
