//! Markdown matter using JSON front matter.

use std::any::Any;
use once_cell::sync::Lazy;
use regex::Regex;
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
    fn parse_mix_text_to_content_text_and_matter_text<S: Into<String>>(&self, mix_text: S) -> Option<(String, String)> {
        debug!("MatterParserWithJSON parse_mix_text_to_content_text_and_matter_text");
        if let Some(captures) = REGEX.captures(mix_text.into().as_ref()) {
            Some((
                String::from(captures.name("content").unwrap().as_str()),
                String::from(captures.name("matter").unwrap().as_str()),
            ))
        } else {
            None
        }
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
    fn parse_matter_text_to_state<S: Into<String>>(&self, matter_text: S) -> Option<StateWithJSON> {
        debug!("MatterParserWithJSON parse_matter_text_to_state");
        match parse_matter_text_to_vars(matter_text.into()) {
            Ok(x) => Some(x),
            _ => None,
        }
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
pub fn parse_matter_text_to_vars<S: AsRef<str> + Sized>(matter_text: S) -> Result<::serde_json::Value, ::serde_json::Error> {
    ::serde_json::from_str(matter_text.as_ref())
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

    fn expect_vars() -> ::serde_json::Value {
        serde_json::from_str(indoc!{r#"
            {
                "alpha": "bravo",
                "charlie": "delta"
            }
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
