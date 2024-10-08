//! Templater management.
//!
//! This can use Handlebars, Liquid, Tera.
//!
//! This can be expanded for potential future formats.

#[allow(dead_code)]
#[derive(Debug)]
pub enum TemplaterEnum<'templater> {
    #[allow(dead_code)] TemplaterWithHandlebars(crate::templater::templater_with_handlebars::TemplaterWithHandlebars<'templater>),
    // #[allow(dead_code)] TemplaterWithAskama(crate::templater::templater_with_askama::TemplaterWithAskama),
    // #[allow(dead_code)] TemplaterWithLiquid(crate::templater::templater_with_liquid::TemplaterWithLiquid),
    // #[allow(dead_code)] TemplaterWithTera(crate::templater::templater_with_tera::TemplaterWithTera),
}
