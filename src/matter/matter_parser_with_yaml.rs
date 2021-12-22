//! Markdown matter using YAML front matter.

use std::any::Any;
use once_cell::sync::Lazy;
use regex::Regex;
use serde_yaml;
use crate::matter::matter_parser::MatterParser;
use crate::state::state_with_yaml::StateWithYAML;

pub struct MatterParserWithYAML {
}

impl MatterParser<StateWithYAML> for MatterParserWithYAML {

    /// Reflection
    fn as_any(&self) -> &dyn Any {
        self
    }

    /// Parse a block of mix text to content text and matter text.
    #[allow(dead_code)]
    fn parse_mix_text_to_content_text_and_matter_text<S: Into<String>>(&self, mix_text: S) -> Option<(String, String)> {
        if let Some(captures) = REGEX.captures(mix_text.into().as_ref()) {
            Some((
                String::from(captures.name("content").unwrap().as_str()),
                String::from(captures.name("matter").unwrap().as_str()),
            ))
        } else {
            None
        }
    }

    /// Parse a block of matter text to matter variables as a matter state struct YAML enum.
    ///
    /// Example:
    ///
    /// ```
    /// let matter_text = indoc!{r#"
    ///     alpha: bravo
    ///     charlie: delta
    /// "#};
    /// let state: StateWithYAML = parse_matter_text_to_state(&matter_text);
    /// assert_eq!(state.data["alpha"], "bravo");
    /// assert_eq!(state.data["charlie"], "delta");
    /// ```
    ///
    #[allow(dead_code)]
    fn parse_matter_text_to_state<S: Into<String>>(&self, matter_text: S) -> Option<StateWithYAML> {
        match parse_matter_text_to_vars(matter_text.into()) {
            Ok(x) => Some(x),
            _ => None,
        }
    }

}

// TODO replace these older chunks...
//
//   ::yaml_rust::yaml::Yaml -> serde_yaml::Value
//   ::yaml_rust::ScanError -> serde_yaml::Error
//   ::yaml_rust::YamlLoader::load_from_str -> serde_yaml::from_str

// #[allow(dead_code)]
// pub fn blank() -> ::yaml_rust::yaml::Yaml {
//     let docs = ::yaml_rust::YamlLoader::load_from_str("").unwrap();
//     let doc = &docs[0];
//     doc.clone()
// }

pub static REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?m)(?s)\A---\n(?P<matter>.*?\n)---\n(?P<content>.*)\z").unwrap()
});

/// Parse matter text to variables implemented as YAML.
///
/// Example:
///
/// ```
/// let matter_text = indoc!{r#"
///     alpha: bravo
///     charlie: delta
/// "#};
/// let vars: ::serde_yaml::Value = parse_matter_text_to_vars(&matter_text).unwrap();
/// assert_eq!(vars["alpha"], "bravo");
/// assert_eq!(vars["charlie"], "delta");
/// ```
///
#[allow(dead_code)]
pub fn parse_matter_text_to_vars<S: AsRef<str> + Sized>(matter_text: S) -> Result<::serde_yaml::Value, ::serde_yaml::Error> {
    serde_yaml::from_str(matter_text.as_ref())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::indoc::indoc;

    type MatterParserX = MatterParserWithYAML;

    const MIX_TEXT: &str = indoc!{r#"
        ---
        alpha: bravo
        charlie: delta
        ---
        echo
        foxtrot
    "#};

    const CONTENT_TEXT: &str = indoc!{r#"
        echo
        foxtrot
    "#};

    const MATTER_TEXT: &str = indoc!{r#"
        alpha: bravo
        charlie: delta
    "#};

    fn expect_vars() -> serde_yaml::Value {
        serde_yaml::from_str(indoc!{r#"
            alpha: bravo
            charlie: delta
        "#}).unwrap()
    }
    
    #[test]
    fn test_parse_mix_text_to_content_text_and_matter_text_x_present() {
        let option = MatterParserX{}.parse_mix_text_to_content_text_and_matter_text(MIX_TEXT);
        assert!(option.is_some());
        let (content_text, matter_text) = option.unwrap();
        assert_eq!(content_text, CONTENT_TEXT);
        assert_eq!(matter_text, MATTER_TEXT);
    }

    #[test]
    fn test_parse_mix_text_to_content_text_and_matter_text_x_absent() {
        let option = MatterParserX{}.parse_mix_text_to_content_text_and_matter_text(CONTENT_TEXT);
        assert!(option.is_none());
    }

    #[test]
    fn test_parse_matter_text_to_vars() {
        let result = parse_matter_text_to_vars(MATTER_TEXT);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expect_vars());
    }

    #[test]
    fn test_parse_matter_text_to_state() {
        let option = MatterParserX{}.parse_matter_text_to_state(MATTER_TEXT);
        assert!(option.is_some());
        let state = option.unwrap();
        assert_eq!(state, expect_vars());
    }

}
