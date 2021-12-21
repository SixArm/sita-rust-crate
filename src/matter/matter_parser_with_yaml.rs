//! Markdown matter using YAML front matter.

use once_cell::sync::Lazy;
use regex::Regex;
use serde_yaml;
use crate::matter::matter_parser::MatterParser;

pub struct MatterParserWithYAML {
}

impl MatterParser for MatterParserWithYAML {

    /// Parse a block of mix text to content text and matter text.
    #[allow(dead_code)]
    fn parse_mix_text_to_content_text_and_matter_text(mix_text: &str) -> Option<(&str, &str)> {
        if let Some(captures) = REGEX.captures(mix_text) {
            Some((
                captures.name("content").unwrap().as_str(),
                captures.name("matter").unwrap().as_str(),
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
    /// let text = indoc!{r#"
    ///     alpha: bravo
    ///     charlie: delta
    /// "#};
    /// let state: crate::matter::state::State = parse_to_state(&text);
    /// assert_eq!(state["alpha"], "bravo");
    /// assert_eq!(state["charlie"], "delta");
    /// ```
    ///
    #[allow(dead_code)]
    fn parse_matter_text_to_matter_state<S: AsRef<str> + Sized>(matter_text: S) -> crate::matter::state::State {
        match parse_matter_text_to_vars(matter_text) {
            Ok(x) => crate::matter::state::State::YAML(x),
            _ => crate::matter::state::State::None,
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
        let option = MatterParserX::parse_mix_text_to_content_text_and_matter_text(MIX_TEXT);
        assert!(option.is_some());
        let (content_text, matter_text) = option.unwrap();
        assert_eq!(content_text, CONTENT_TEXT);
        assert_eq!(matter_text, MATTER_TEXT);
    }

    #[test]
    fn test_parse_mix_text_to_content_text_and_matter_text_x_absent() {
        let option = MatterParserX::parse_mix_text_to_content_text_and_matter_text(CONTENT_TEXT);
        assert!(option.is_none());
    }

    #[test]
    fn test_parse_matter_text_to_vars() {
        let result = parse_matter_text_to_vars(MATTER_TEXT);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expect_vars());
    }

    #[test]
    fn test_parse_matter_text_to_matter_state() {
        if let crate::matter::state::State::YAML(vars) = MatterParserX::parse_matter_text_to_matter_state(MATTER_TEXT) {
            assert_eq!(vars, expect_vars());
        } else {
            panic!("State vars");
        };
    }

    #[test]
    fn test_parse_mix_text_to_content_text_and_matter_state() {
        let option = MatterParserX::parse_mix_text_to_content_text_and_matter_state(MIX_TEXT);
        assert!(option.is_some());
        let (content_text, matter_state) = option.unwrap();
        assert_eq!(content_text, CONTENT_TEXT);
        if let crate::matter::state::State::YAML(vars) = matter_state {
            assert_eq!(vars, expect_vars());
        } else {
            panic!("State vars");
        };
    }

}
