//! Markdown matter using JSON front matter.

use once_cell::sync::Lazy;
use regex::Regex;
use crate::matter::matter_parser::MatterParser;

pub struct MatterParserWithJSON {
}

impl MatterParser for MatterParserWithJSON {

    /// Parse a block of text to a markdown content and matter text.
    #[allow(dead_code)]
    fn parse(text: &str) -> Option<(&str, &str)> {
        if let Some(captures) = REGEX.captures(text) {
            Some((
                captures.name("markdown").unwrap().as_str(),
                captures.name("matter").unwrap().as_str(),
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
    /// let text = indoc!{r#"
    ///     {
    ///         "alpha": "bravo",
    ///         "charlie": "delta"
    ///     }
    /// "#};
    /// let state: crate::matter::state::State = parse_to_state(&text);
    /// assert_eq!(state["alpha"], "bravo");
    /// assert_eq!(state["charlie"], "delta");
    /// ```
    ///
    #[allow(dead_code)]
    fn parse_to_matter_state<S: AsRef<str> + Sized>(text: S) -> crate::matter::state::State {
        match parse_to_vars(text) {
            Ok(x) => crate::matter::state::State::JSON(x),
            _ => crate::matter::state::State::None,
        }
    }

}

// #[allow(dead_code)]
// pub fn blank() -> ::serde_json::Value {
//     ::serde_json::from_str("").unwrap()
// }

pub static REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?m)(?s)\A(?P<matter>\{.*?\n\}\n)(?P<markdown>.*)\z").unwrap()
});


/// Parse a block of text to a JSON value.
///
/// Example:
///
/// ```
/// let tex = indoc!{r#"
///     {
///         "alpha": "bravo",
///         "charlie": "delta"
///     }
/// "#};
/// let x: ::serde_json::Value = parse(&text).unwrap();
/// assert_eq!(x["alpha"], "bravo");
/// assert_eq!(x["charlie"], "delta");
/// ```
///
#[allow(dead_code)]
pub fn parse_to_vars<S: AsRef<str> + Sized>(text: S) -> Result<::serde_json::Value, ::serde_json::Error> {
    ::serde_json::from_str(text.as_ref())
}


#[cfg(test)]
mod tests {
    use super::*;
    use ::indoc::indoc;

    type MatterParserX = MatterParserWithJSON;

    #[test]
    fn test_parse_x_present() {
        let input_markdown = indoc!{r#"
            {
                "alpha": "bravo",
                "charlie": "delta"
            }
            echo
            foxtrot
        "#};
        let expect_matter = indoc!{r#"
            {
                "alpha": "bravo",
                "charlie": "delta"
            }
        "#};
        let expect_markdown = indoc!{r#"
            echo
            foxtrot
        "#};
        let option = MatterParserX::parse(input_markdown);
        assert!(option.is_some());
        let (actual_markdown, actual_matter) = option.unwrap();
        assert_eq!(actual_markdown, expect_markdown);
        assert_eq!(actual_matter, expect_matter);
    }

    #[test]
    fn test_parse_x_absent() {
        let input_markdown = indoc!{r#"
            echo
            foxtrot
        "#};
        let option = MatterParserX::parse(input_markdown);
        assert!(option.is_none());
    }

    #[test]
    fn test_parse_to_vars() {
        let text = indoc!{r#"
            {
                "alpha": "bravo",
                "charlie": "delta"
            }
        "#};
        let matter_result = parse_to_vars(&text);
        assert!(matter_result.is_ok());
        let matter = matter_result.unwrap();
        assert_eq!(matter["alpha"].as_str().unwrap(), "bravo");
        assert_eq!(matter["charlie"].as_str().unwrap(), "delta");
    }

    #[test]
    fn test_parse_to_matter_state() {
        let text = indoc!{r#"
            {
                "alpha": "bravo",
                "charlie": "delta"
            }
        "#};
        let _matter_state = MatterParserX::parse_to_matter_state(&text);
        //TODO
    }

}
