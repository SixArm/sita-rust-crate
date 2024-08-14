//! Matter parser mutex
//!
//! This is a Markdown front matter parser that can handle a variety of formats:
//!
//! * BTML: BTreeMap struct
//! * JSON: JavaScript Object Notation
//! * MDCC: Markdown comment code
//! * TOML: Tom's Obvious Markup Language
//! * YAML: Yet Anther Markup Language

use crate::matter::matter_parser_trait::MatterParserTrait;
use crate::matter::matter_parser_with_html::MatterParserWithHTML;
use crate::matter::matter_parser_with_json::MatterParserWithJSON;
use crate::matter::matter_parser_with_markdown_comments::MatterParserWithMarkdownComments;
use crate::matter::matter_parser_with_toml::MatterParserWithTOML;
use crate::matter::matter_parser_with_yaml::MatterParserWithYAML;
use crate::state::state_trait::StateTrait;

/// Parse from mix text to content text and state.
///
/// Example BTMS:
///
/// ```
/// # use ::indoc::indoc;
/// let mix_text = indoc!{r#"
/// <!--
/// alfa: bravo
/// charlie: delta
/// -->
/// echo
/// foxtrot
/// "#};
/// let (content_text, box_dyn_state_trait) = parse_mix_text_to_content_text_and_state(mix_text).unwrap();
/// ```
///
/// Example JSON:
///
/// ```
/// # use ::indoc::indoc;
/// let mix_text = indoc!{r#"
///     {
///         "alfa": "bravo",
///         "charlie": "delta",
///     }
///     echo
///     foxtrot
/// "#};
/// let (content_text, box_dyn_state_trait) = parse_mix_text_to_content_text_and_state(mix_text).unwrap();
/// ```
///
/// Example MDCC:
///
/// ```
/// # use ::indoc::indoc;
/// let mix_text = indoc!{r#"
///     [//]: # (alfa: bravo)
///     [//]: # (charlie: delta)
///     echo
///     foxtrot
/// "#};
/// let (content_text, box_dyn_state_trait) = parse_mix_text_to_content_text_and_state(mix_text).unwrap();
/// ```
///
/// Example TOML:
///
/// ```
/// # use ::indoc::indoc;
/// let mix_text = indoc!{r#"
///     +++
///     alfa = "bravo"
///     charlie = "delta"
///     +++
///     echo
///     foxtrot
/// "#};
/// let (content_text, box_dyn_state_trait) = parse_mix_text_to_content_text_and_state(mix_text).unwrap();
/// ```
///
/// Example YAML:
///
/// ```
/// # use ::indoc::indoc;
/// let mix_text = indoc!{r#"
///     ---
///     alfa: bravo
///     charlie: delta
///     ---
///     echo
///     foxtrot
/// "#};
/// let (content_text, box_dyn_state_trait) = parse_mix_text_to_content_text_and_state(mix_text).unwrap();
/// ```

#[allow(dead_code)]
pub fn parse_mix_text_to_content_text_and_state(mix_text: &str) -> Result<(String, Box<dyn StateTrait>), Error> {
    trace!("parse_mix_text_to_content_text_and_state");
    if let Ok((s, state)) = (MatterParserWithHTML{}.parse_mix_text_to_content_text_and_state(mix_text)) { return Ok((s, Box::new(state))); }
    if let Ok((s, state)) = (MatterParserWithJSON{}.parse_mix_text_to_content_text_and_state(mix_text)) { return Ok((s, Box::new(state))); }
    if let Ok((s, state)) = (MatterParserWithMarkdownComments{}.parse_mix_text_to_content_text_and_state(mix_text)) { return Ok((s, Box::new(state))); }
    if let Ok((s, state)) = (MatterParserWithTOML{}.parse_mix_text_to_content_text_and_state(mix_text)) { return Ok((s, Box::new(state))); }
    if let Ok((s, state)) = (MatterParserWithYAML{}.parse_mix_text_to_content_text_and_state(mix_text)) { return Ok((s, Box::new(state))); }
    Err(Error::ParseMixTextToContentTextAndMatterText { mix_text: mix_text.to_owned() })
}

#[derive(thiserror::Error, Debug)]
pub enum Error {

    #[error("parse mix text to content text and state ➡ mix_text: {mix_text}")]
    ParseMixTextToContentTextAndMatterText {
        mix_text: String,
    }

}

#[cfg(test)]
mod tests {
    // use super::*;
    // use indoc::indoc;
}
