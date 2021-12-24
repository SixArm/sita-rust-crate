//! Markdown matter using JSON front matter.

use std::any::Any;
use once_cell::sync::Lazy;
use regex::Regex;
use crate::errors::*;
use crate::matter::matter_parser::MatterParser;
use crate::state::state_with_json::StateWithJSON;
pub struct MatterParserWithJSON {
}

impl MatterParser<StateWithJSON> for MatterParserWithJSON {

    /// Reflection.
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    /// Parse a block of mix text to content text and matter text.
    #[allow(dead_code)]
    fn parse_mix_text_to_content_text_and_matter_text(&self, mix_text: &str) -> Result<(String, String)> {
        trace!("MatterParserWithJSON::parse_mix_text_to_content_text_and_matter_text");
        let captures = REGEX.captures(mix_text)
        .chain_err(|| "captures")?;
        Ok((
            String::from(captures.name("content").unwrap().as_str()),
            String::from(captures.name("matter").unwrap().as_str()),
        ))
    }

    /// Parse a block of text to a matter state struct JSON enum.
    ///
    /// Example:
    ///
    /// ```
    /// let matter_text = indoc!{r#"
    ///     {
    ///         "alpha": "bravo",
    ///         "charlie": "delta"
    ///     }
    /// "#};
    /// let state: StateWithJSON = parse_matter_text_to_state(&matter_text);
    /// assert_eq!(state.data["alpha"], "bravo");
    /// assert_eq!(state.data["charlie"], "delta");
    /// ```
    ///
    #[allow(dead_code)]
    fn parse_matter_text_to_state(&self, matter_text: &str) -> Result<StateWithJSON> {
        trace!("MatterParserWithJSON::parse_matter_text_to_state");
        parse_matter_text_to_vars(&matter_text)
    }

}

// #[allow(dead_code)]
// pub fn blank() -> ::serde_json::Value {
//     ::serde_json::from_str("").unwrap()
// }

pub static REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?m)(?s)\A(?P<matter>\{.*?\n\}\n)(?P<content>.*)\z").unwrap()
});


/// Parse matter text to variables implemented as JSON.
///
/// Example:
///
/// ```
/// let matter_text = indoc!{r#"
///     {
///         "alpha": "bravo",
///         "charlie": "delta"
///     }
/// "#};
/// let vars: ::serde_json::Value = parse_matter_text_to_vars(&matter_text).unwrap();
/// assert_eq!(vars["alpha"], "bravo");
/// assert_eq!(vars["charlie"], "delta");
/// ```
///
#[allow(dead_code)]
pub fn parse_matter_text_to_vars(matter_text: &str) -> Result<StateWithJSON> {
    ::serde_json::from_str(matter_text)
    .chain_err(|| "::serde_json::from_str")
}


#[cfg(test)]
mod tests {
    use super::*;
    use ::indoc::indoc;

    type MatterParserX = MatterParserWithJSON;

    const MIX_TEXT: &str = indoc!{r#"
        {
            "alpha": "bravo",
            "charlie": "delta"
        }
        echo
        foxtrot
    "#};

    const CONTENT_TEXT: &str = indoc!{r#"
        echo
        foxtrot
    "#};

    const MATTER_TEXT: &str = indoc!{r#"
        {
            "alpha": "bravo",
            "charlie": "delta"
        }
    "#};

    fn expect_vars() -> StateWithJSON {
        serde_json::from_str(indoc!{r#"
            {
                "alpha": "bravo",
                "charlie": "delta"
            }
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
    fn test_parse_matter_text_to_vars() {
        let result = parse_matter_text_to_vars(MATTER_TEXT);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expect_vars());
    }

    #[test]
    fn test_parse_matter_text_to_state() {
        let result = MatterParserX{}.parse_matter_text_to_state(MATTER_TEXT);
        assert!(result.is_ok());
        let state = result.unwrap();
        assert_eq!(state, expect_vars());
    }

}
