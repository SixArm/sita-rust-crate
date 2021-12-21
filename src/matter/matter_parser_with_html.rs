//! Markdown matter using HTML front matter.

use once_cell::sync::Lazy;
use regex::Regex;
use crate::types::*;
use crate::matter::matter_parser::MatterParser;

pub struct MatterParserWithHTML {
}

impl MatterParser for MatterParserWithHTML {

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

    /// Parse a block of text to variables as a matter state struct HTML enum.
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
    fn parse_to_matter_state<S: AsRef<str> + Sized>(text: S) -> crate::matter::state::State {
        let vars = parse_to_vars(text);
        match vars.is_empty() {
            false => crate::matter::state::State::HTML(vars),
            _ => crate::matter::state::State::None,
        }
    }

}

pub static REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?m)(?s)\A<!--\n(?P<matter>.*?\n)-->\n(?P<markdown>.*)\z").unwrap()
});

pub static PARSE_LINE_TO_KEY_VALUE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\A\s*(?P<key>\w+?):\s*(?P<value>.*?)\s*\z").unwrap()
});

/// Parse a block of text to variables as a map of string key to string value.
///
/// Example:
///
/// ```
/// let text = indoc!{r#"
///     alpha: bravo
///     charlie: delta
/// "#};
/// let x: Map<String, String> = parse_to_vars(&text);
/// assert_eq!(x["alpha"], "bravo");
/// assert_eq!(x["charlie"], "delta");
/// ```
///
#[allow(dead_code)]
pub fn parse_to_vars<S: AsRef<str> + Sized>(text: S) -> Map<String, String> {
    let mut map: Map<String, String> = Map::new();
    for line in text.as_ref().split("\n") {
        if let Some(captures) = (*PARSE_LINE_TO_KEY_VALUE_REGEX).captures(line) {
            if let Some(key) = captures.name("key") {
                if let Some(value) = captures.name("value") {
                    map.insert(String::from(key.as_str()), String::from(value.as_str()));
                }
            }
        }
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::indoc::indoc;

    type MatterParserX = MatterParserWithHTML;

    #[test]
    fn test_parse_x_present() {
        let input_markdown = indoc!{r#"
            <!--
            alpha: bravo
            charlie: delta
            -->
            echo
            foxtrot
        "#};
        let expect_markdown = indoc!{r#"
            echo
            foxtrot
        "#};
        let expect_matter = indoc!{r#"
            alpha: bravo
            charlie: delta
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
            alpha: bravo
            charlie: delta
        "#};
        let actual: Map<String, String> = parse_to_vars(&text);
        assert_eq!(actual["alpha"], "bravo");
        assert_eq!(actual["charlie"], "delta");
    }

    #[test]
    fn test_parse_to_matter_state() {
        let text = indoc!{r#"
            alpha: bravo
            charlie: delta
        "#};
        let _matter_state = MatterParserX::parse_to_matter_state(&text);
        //TODO
    }

}
