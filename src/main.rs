//! Main
//!
//!   * `args` - Our arguments struct, set via `clap`.
//!   * `config` - Our configuration struct, set via `confy`.
//!   * `confy` - Tests for `confy` loading and parsing.
//!   * `tera` - Tera template loading and parsing.
//!   * `vars` - Our variables for Tera template context.

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
    pub(crate) mod tera;
    pub(crate) mod vars;
}

use ::tera::Tera;
use crate::app::args::Args;
use crate::app::config::Config;
use crate::templating::vars::Vars;

//fn main() -> Result<(), ::std::io::Error> {
fn main() -> Result<(), ::confy::ConfyError> {
    let _config: Config = ::confy::load("sita")?;
    let args: Args = crate::app::clap::args();
    let tera: Tera = crate::templating::tera::init(&*args.templates_glob);
    let vars: Vars = Vars { title: "Example".into() };
    let html = tera.render(
        "example.html", 
        &::tera::Context::from_serialize(&vars).unwrap()
    ).unwrap();
    print!("{}", html);
    Ok(())
}