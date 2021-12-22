//! Markdown matter using HTML, JSON, TOML, YAML.

use serde::Serialize;
use crate::matter::matter_parser::MatterParser;
use crate::matter::matter_parser_with_html::MatterParserWithHTML;
use crate::matter::matter_parser_with_json::MatterParserWithJSON;
use crate::matter::matter_parser_with_toml::MatterParserWithTOML;
use crate::matter::matter_parser_with_yaml::MatterParserWithYAML;
use crate::state::state::State;

/// Parse from mix text to content text and state.
///
/// Example:
///
/// ```
/// let mix_text = indoc!{r#"
/// <!--
///   alpha: bravo
///   charlie: delta
/// -->
/// echo
/// foxtrot
/// "#};
/// let (content_text, dyn_state) = parse_mix_text_to_content_text_and_state(mix_text).unwrap();
/// ```
///
#[allow(dead_code)]
pub fn parse_mix_text_to_content_text_and_state(mix_text: &str) -> Option<(String, Box<dyn State>)> {
    if let Some((s, state)) = (MatterParserWithHTML{}.parse_mix_text_to_content_text_and_state(mix_text)) { return Some((s, Box::new(state))); }
    if let Some((s, state)) = (MatterParserWithJSON{}.parse_mix_text_to_content_text_and_state(mix_text)) { return Some((s, Box::new(state))); }
    if let Some((s, state)) = (MatterParserWithTOML{}.parse_mix_text_to_content_text_and_state(mix_text)) { return Some((s, Box::new(state))); }
    if let Some((s, state)) = (MatterParserWithYAML{}.parse_mix_text_to_content_text_and_state(mix_text)) { return Some((s, Box::new(state))); }
    None
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use indoc::indoc;
}
