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

#[macro_use]
extern crate log;

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
    let tera: Tera = crate::templating::tera::init(&*args.templates_glob);
    if let Some(paths) = &args.paths {
        for input_path in paths {
            do_path(&args, &tera, &args.template_file, &input_path)?;
        }
    };
    Ok(())
}

fn do_path(args: &Args, tera: &Tera, template: &str, input_path: &PathBuf) -> Result<()> {
    if args.verbose > 0 {
        info!("do path → start → input:{:?}", input_path);
    }
    vet_input_path_file(input_path)?;
    vet_input_path_name(input_path)?;

    // Prepare output file path
    let mut output_path: PathBuf;
    match &args.output_path {
        Some(x) => {
            output_path = x.to_path_buf();
        },
        None => {
            output_path = PathBuf::from(input_path);
            output_path.set_extension("html");
        },
    };

    // Translate Markdown to HTML
    let input_as_markdown = ::std::fs::read_to_string(&input_path)
        .chain_err(|| format!("input path must be readable; path: {:?}", input_path))?;
    let content_as_html = markdown_to_html(&input_as_markdown);

    // Create variables
    let vars = Vars {
        title: Some("my title".into()),
        content: Some(content_as_html),
    };

    // Render Tera template that has {{ content }} slot for HTML string
    let context = ::tera::Context::from_serialize(&vars)
        .chain_err(|| "create context")?;
    let output_as_html = tera.render(template, &context)
        .chain_err(|| "render template")?;
    ::std::fs::write(&output_path, output_as_html)
        .chain_err(|| "write output")?;
    if args.verbose > 0 {
        info!("do path → success → input:{:?} output:{:?}", input_path, output_path);
    }
    Ok(())
}

fn vet_input_path_file(input_path: &PathBuf) -> Result<()> {
    if !input_path.exists() {
        bail!("input path must exist. path: {:?}", input_path)
    }
    let metadata = ::std::fs::metadata(input_path)
        .chain_err(|| format!("input path must have metadata. path: {:?}", input_path))?;
    if !metadata.is_file() {
        bail!("input path must be a file. path: {:?}", input_path);
    }
    Ok(())
}

fn vet_input_path_name(input_path: &PathBuf) -> Result<()> {
    // Check if we want to process the input path or skip it
    let os_md = OsStr::new("md");
    let extension = input_path.extension();
    if extension != Some(&os_md) {
        bail!("input path must have extension \".md\". path: {:?}", input_path);
    }
    Ok(())
}

fn markdown_to_html(input_as_markdown: &str) -> String {
    let parser = crate::markdown::markdown_parser::parser(&*input_as_markdown);
    let mut content_as_html = String::new();
    pulldown_cmark::html::push_html(&mut content_as_html, parser);
    content_as_html
}
