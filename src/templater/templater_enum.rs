//! Templater management.
//!
//! This can use Handlebars, Liquid, Tera.
//! 
//! This can be expanded for potential future formats.

#[derive(Debug)]
pub enum TemplaterEnum<'templater> {
    #[allow(dead_code)] TemplaterWithHandlebars(crate::templater::templater_with_handlebars::TemplaterWithHandlebars<'templater>),
    // #[allow(dead_code)] TemplaterWithLiquid(crate::templater::templater_with_liquid::TemplaterWithLiquid),
    #[allow(dead_code)] TemplaterWithTera(crate::templater::templater_with_tera::TemplaterWithTera),
}
