//! Templater management.
//!
//! This can use Handlebars, Liquid, Tera.
//! 
//! This can be expanded for potential future formats.

#[derive(Debug)]
pub enum TemplaterEnum<'templater> {
    TemplaterWithHandlebars(crate::templater::templater_with_handlebars::TemplaterWithHandlebars<'templater>),
    //TemplaterWithLiquid(crate::templater::templater_with_liquid::TemplaterWithLiquid),
    TemplaterWithTera(crate::templater::templater_with_tera::TemplaterWithTera),
}
