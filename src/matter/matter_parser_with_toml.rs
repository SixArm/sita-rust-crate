//! Markdown matter using TOML front matter.

use std::any::Any;
use once_cell::sync::Lazy;
use regex::Regex;
use crate::matter::matter_parser::MatterParser;
use crate::state::state_with_toml::StateWithTOML;

pub struct MatterParserWithTOML {
}

impl MatterParser<StateWithTOML> for MatterParserWithTOML {
    
    /// Reflection
    fn as_any(&self) -> &dyn Any {
        self
    }

    /// Parse a block of mix text to content text and matter text.
    #[allow(dead_code)]
    fn parse_mix_text_to_content_text_and_matter_text<S: Into<String>>(&self, mix_text: S) -> Option<(String, String)> {
        debug!("MatterParserWithTOML parse_mix_text_to_content_text_and_matter_text");
        if let Some(captures) = REGEX.captures(mix_text.into().as_ref()) {
            Some((
                String::from(captures.name("content").unwrap().as_str()),
                String::from(captures.name("matter").unwrap().as_str()),
            ))
        } else {
            None
        }
    }
    
    /// Parse a block of text to a matter state struct TOML enum.
    ///
    /// Example:
    ///
    /// ```
    /// let matter_text = indoc!{r#"
    ///     alpha = "bravo"
    ///     charlie = "delta"
    /// "#};
    /// let state: StateWithTOML = parse_matter_text_to_state(&matter_text);
    /// assert_eq!(state.data["alpha"], "bravo");
    /// assert_eq!(state.data["charlie"], "delta");
    /// ```
    ///
    #[allow(dead_code)]
    fn parse_matter_text_to_state<S: Into<String>>(&self, matter_text: S) -> Option<StateWithTOML> {
        debug!("MatterParserWithTOML parse_matter_text_to_state");
        match parse_matter_text_to_vars(matter_text.into()) {
            Ok(x) => Some(x),
            _ => None,
        }
    }

}

// #[allow(dead_code)]
// pub fn blank() -> ::toml::Value {
//     "".parse::<::toml::Value>().unwrap()
// }

pub static REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?m)(?s)\A\+\+\+\n(?P<matter>.*?\n)\+\+\+\n(?P<content>.*)\z").unwrap()
});


/// Parse matter text to variables implemented as TOML.
///
/// Example:
///
/// ```
/// let matter_text = indoc!{r#"
///     alpha = "bravo"
///     charlie = "delta"
/// "#};
/// let vars: ::toml::Value = parse_matter_text_to_vars(&matter_text).unwrap();
/// assert_eq!(vars["alpha"], "bravo");
/// assert_eq!(vars["charlie"], "delta");
/// ```
///
#[allow(dead_code)]
pub fn parse_matter_text_to_vars<S: AsRef<str> + Sized>(matter_text: S) -> Result<::toml::Value, ::toml::de::Error> {
    matter_text.as_ref().parse::<::toml::Value>()
}

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

    fn expect_vars() -> toml::Value {
        let s = indoc!{r#"
            alpha = "bravo"
            charlie = "delta"
        "#};
        s.parse::<::toml::Value>().unwrap()
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
