//! Markdown matter using HTML front matter.

use std::any::Any;
use once_cell::sync::Lazy;
use regex::Regex;
use crate::types::*;
use crate::matter::matter_parser::MatterParser;
use crate::state::state_with_html::StateWithHTML;

pub struct MatterParserWithHTML {
}

impl MatterParser<StateWithHTML> for MatterParserWithHTML {

    /// Reflection
    fn as_any(&self) -> &dyn Any {
        self
    }

    /// Parse mix text to content text and matter text.
    #[allow(dead_code)]
    fn parse_mix_text_to_content_text_and_matter_text<S: Into<String>>(&self, mix_text: S) -> Option<(String, String)> {
        debug!("MatterParserWithHTML parse_mix_text_to_content_text_and_matter_text");
        if let Some(captures) = REGEX.captures(mix_text.into().as_ref()) {
            Some((
                String::from(captures.name("content").unwrap().as_str()),
                String::from(captures.name("matter").unwrap().as_str()),
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
    /// let matter_text = indoc!{r#"
    ///     alpha: bravo
    ///     charlie: delta
    /// "#};
    /// let state: StateWithHTML = parse_matter_text_to_state(&matter_text).unwrap();
    /// assert_eq!(state.data["alpha"], "bravo");
    /// assert_eq!(state.data["charlie"], "delta");
    /// ```
    ///
    #[allow(dead_code)]
    fn parse_matter_text_to_state<S: Into<String>>(&self, matter_text: S) -> Option<StateWithHTML> {
        debug!("MatterParserWithHTML parse_matter_text_to_state");
        let vars = parse_matter_text_to_vars(matter_text.into());
        match vars.is_empty() {
            false => Some(vars),
            _ => None,
        }
    }

}

pub static REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?m)(?s)\A<!--\n(?P<matter>.*?\n)-->\n(?P<content>.*)\z").unwrap()
});

pub static PARSE_LINE_TO_KEY_VALUE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\A\s*(?P<key>\w+?):\s*(?P<value>.*?)\s*\z").unwrap()
});

/// Parse matter text to variables implemented as a Map.
///
/// Example:
///
/// ```
/// let matter_text = indoc!{r#"
///     alpha: bravo
///     charlie: delta
/// "#};
/// let vars: Map<String, String> = parse_matter_text_to_vars(&matter_text);
/// assert_eq!(vars["alpha"], "bravo");
/// assert_eq!(vars["charlie"], "delta");
/// ```
///
#[allow(dead_code)]
pub fn parse_matter_text_to_vars<S: AsRef<str> + Sized>(matter_text: S) -> Map<String, String> {
    let mut map: Map<String, String> = Map::new();
    for line in matter_text.as_ref().split("\n") {
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

    const MIX_TEXT: &str = indoc!{r#"
        <!--
        alpha: bravo
        charlie: delta
        -->
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

    fn expect_vars() -> Map<String, String> {
        map!(
            String::from("alpha") => String::from("bravo"), 
            String::from("charlie") => String::from("delta")
        )
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
        let vars: Map<String, String> = parse_matter_text_to_vars(MATTER_TEXT);
        assert_eq!(vars, expect_vars());
    }

    #[test]
    fn test_parse_matter_text_to_state() {
        let option = MatterParserX{}.parse_matter_text_to_state(MATTER_TEXT);
        assert!(option.is_some());
        let state = option.unwrap();
        assert_eq!(state, expect_vars());
    }

}
