//! Markdown matter using YAML front matter.

use std::any::Any;
use once_cell::sync::Lazy;
use regex::Regex;
use serde_yaml;
use crate::errors::*;
use crate::matter::matter_parser_trait::MatterParserTrait;
use crate::state::state_with_yaml::StateWithYAML;

#[derive(Debug)]
pub struct MatterParserWithYAML {
}

impl MatterParserTrait<StateWithYAML> for MatterParserWithYAML {

    fn as_any(&self) -> &dyn Any {
        self
    }

    /// Example:
    /// 
    /// ```
    /// # use ::indoc::indoc;
    /// let mix_text = indoc!{r#"
    ///     ---
    ///     alpha: bravo
    ///     charlie: delta
    ///     ---
    ///     echo
    ///     foxtrot
    /// "#};
    /// let content_text, matter_text = parse_mix_text_to_content_text_and_matter_text(mix_text).unwrap();
    /// assert_eq!(content_text, indoc!{r#"
    ///     echo
    ///     foxtrot
    /// "#};
    /// assert_eq!(matter_text, indoc!{r#"
    ///     alpha: bravo
    ///     charlie: delta
    /// "#};
    /// ```
    #[allow(dead_code)]
    fn parse_mix_text_to_content_text_and_matter_text(&self, mix_text: &str) -> Result<(String, String)> {
        trace!("MatterParserWithYAML::parse_mix_text_to_content_text_and_matter_text");
        let captures = REGEX.captures(mix_text)
        .chain_err(|| "captures")?;
        Ok((
            String::from(captures.name("content").unwrap().as_str()),
            String::from(captures.name("matter").unwrap().as_str()),
        ))
    }

    /// Example:
    /// 
    /// ```
    /// # use ::indoc::indoc;
    /// let matter_text = indoc!{r#"
    ///     alpha: bravo
    ///     charlie: delta
    /// "#};
    /// let state = parse_matter_text_to_state(mix_text).unwrap();
    /// assert_eq!(state.get("alpha"), String::from("bravo"));
    /// assert_eq!(state.get("charlie"), String::from("delta"));
    /// ```
    #[allow(dead_code)]
    fn parse_matter_text_to_state(&self, matter_text: &str) -> Result<StateWithYAML> {
        trace!("MatterParserWithYAML::parse_matter_text_to_state");
        serde_yaml::from_str(matter_text)
        .chain_err(|| "::serde_yaml::from_str")
    }

}

pub static REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?m)(?s)\A---\n(?P<matter>.*?\n)---\n(?P<content>.*)\z").unwrap()
});

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

    fn expect_state() -> StateWithYAML {
        serde_yaml::from_str(indoc!{r#"
            alpha: bravo
            charlie: delta
        "#}).unwrap()
    }

    #[test]
    fn test_parse_mix_text_to_content_text_and_matter_text_x_present() {
        let result = MatterParserX{}.parse_mix_text_to_content_text_and_matter_text(MIX_TEXT);
        assert!(result.is_ok());
        let (content_text, matter_text) = result.unwrap();
        assert_eq!(content_text, CONTENT_TEXT);
        assert_eq!(matter_text, MATTER_TEXT);
    }

    #[test]
    fn test_parse_mix_text_to_content_text_and_matter_text_x_absent() {
        let result = MatterParserX{}.parse_mix_text_to_content_text_and_matter_text(CONTENT_TEXT);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_matter_text_to_state() {
        let result = MatterParserX{}.parse_matter_text_to_state(MATTER_TEXT);
        assert!(result.is_ok());
        let state = result.unwrap();
        assert_eq!(state, expect_state());
    }

}
