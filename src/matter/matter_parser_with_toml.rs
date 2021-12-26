//! Markdown matter using TOML front matter.

use std::any::Any;
use once_cell::sync::Lazy;
use regex::Regex;
use crate::errors::*;
use crate::matter::matter_parser_trait::MatterParserTrait;
use crate::state::state_with_toml::StateWithTOML;

#[derive(Debug)]
pub struct MatterParserWithTOML {
}

impl MatterParserTrait<StateWithTOML> for MatterParserWithTOML {
    
    fn as_any(&self) -> &dyn Any {
        self
    }

    #[allow(dead_code)]
    fn parse_mix_text_to_content_text_and_matter_text(&self, mix_text: &str) -> Result<(String, String)> {
        trace!("MatterParserWithTOML::parse_mix_text_to_content_text_and_matter_text");
        let captures = REGEX.captures(mix_text)
        .chain_err(|| "captures")?;
        Ok((
            String::from(captures.name("content").unwrap().as_str()),
            String::from(captures.name("matter").unwrap().as_str()),
        ))
    }
    
    #[allow(dead_code)]
    fn parse_matter_text_to_state(&self, matter_text: &str) -> Result<StateWithTOML> {
        trace!("MatterParserWithTOML::parse_matter_text_to_state");
        let toml = matter_text.parse::<::toml::Value>()
        .chain_err(|| "matter_text.parse::<::toml::Value>()")?;
        match toml {
            ::toml::Value::Table(table) => Ok(table),
            _ => bail!("TOML isn't a map"),
        }
    }

}

pub static REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?m)(?s)\A\+\+\+\n(?P<matter>.*?\n)\+\+\+\n(?P<content>.*)\z").unwrap()
});

#[cfg(test)]
mod tests {
    use super::*;
    use ::indoc::indoc;

    type MatterParserX = MatterParserWithTOML;

    const MIX_TEXT: &str = indoc!{r#"
        +++
        alpha = "bravo"
        charlie = "delta"
        +++
        echo
        foxtrot
    "#};

    const CONTENT_TEXT: &str = indoc!{r#"
        echo
        foxtrot
    "#};

    const MATTER_TEXT: &str = indoc!{r#"
        alpha = "bravo"
        charlie = "delta"
    "#};

    fn expect_state() -> StateWithTOML {
        toml::from_str(indoc!{r#"
            alpha = "bravo"
            charlie = "delta"
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
