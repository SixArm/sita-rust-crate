//! Markdown matter using JSON front matter.

use std::any::Any;
use once_cell::sync::Lazy;
use regex::Regex;
use crate::matter::matter_parser_trait::MatterParserTrait;

/// State alias is for this file's generic implementation.
type State = crate::state::state_with_json::StateWithJSON;

#[derive(Debug)]
pub struct MatterParserWithJSON {
}

impl MatterParserTrait<State, Error> for MatterParserWithJSON {

    fn as_any(&self) -> &dyn Any {
        self
    }

    /// Example:
    ///
    /// ```
    /// # use ::indoc::indoc;
    /// let mix_text = indoc!{r#"
    ///     {
    ///         "alfa": "bravo",
    ///         "charlie": "delta",
    ///     }
    ///     echo
    ///     foxtrot
    /// "#};
    /// let content_text, matter_text = parse_mix_text_to_content_text_and_matter_text(mix_text).unwrap();
    /// assert_eq!(content_text, indoc!{r#"
    ///     echo
    ///     foxtrot
    /// "#};
    /// assert_eq!(matter_text, indoc!{r#"
    ///     {
    ///         "alfa": "bravo",
    ///         "charlie": "delta"
    ///     }
    /// "#};
    /// ```
    #[allow(dead_code)]
    fn parse_mix_text_to_content_text_and_matter_text(&self, mix_text: &str) -> Result<(String, String), Error> {
        trace!("parse_mix_text_to_content_text_and_matter_text");
        match REGEX.captures(mix_text) {
            Some(captures) => Ok((
                String::from(captures.name("content").unwrap().as_str()),
                String::from(captures.name("matter").unwrap().as_str()),
            )),
            None => Err(
                Error::ParseMixTextToContentTextAndMatterText {
                    mix_text: mix_text.to_owned()
                }
            )
        }
    }

    /// This function chains:
    ///
    /// * `parse_mix_text_to_content_text_and_matter_text`
    /// * `parse_matter_text_to_state`
    ///
    fn parse_mix_text_to_content_text_and_state(&self, mix_text: &str) -> Result<(String, State), Error> {
        let (content_text, matter_text) = self.parse_mix_text_to_content_text_and_matter_text(mix_text)?;
        let state = self.parse_matter_text_to_state(&matter_text)?;
        Ok((content_text, state))
    }

    /// Example:
    ///
    /// ```
    /// # use ::indoc::indoc;
    /// let mix_text = indoc!{r#"
    ///     {
    ///         "alfa": "bravo",
    ///         "charlie": "delta",
    ///     }
    ///     echo
    ///     foxtrot
    /// "#};
    /// let state = parse_matter_text_to_state(mix_text).unwrap();
    /// assert_eq!(state.get("alfa"), String::from("bravo"));
    /// assert_eq!(state.get("charlie"), String::from("delta"));
    /// ```
    #[allow(dead_code)]
    fn parse_matter_text_to_state(&self, matter_text: &str) -> Result<State, Error> {
        trace!("MatterParserWithJSON::parse_matter_text_to_state");
        match ::serde_json::from_str(matter_text) {
            Ok(o) => Ok(o),
            Err(e) =>  Err(Error::ParseMatterTextToState(e))
        }
    }

}

pub static REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?m)(?s)\A(?P<matter>\{.*?\n\}\n)(?P<content>.*)\z").unwrap()
});

#[derive(thiserror::Error, Debug)]
pub enum Error {

    #[error("ParseMixTextToContentTextAndMatterText ➡ mix_text: {mix_text:?}")]
    ParseMixTextToContentTextAndMatterText {
        mix_text: String,
    },

    #[error("ParseMatterTextToState ➡ {0:?}")]
    ParseMatterTextToState(serde_json::Error)

}

#[cfg(test)]
mod tests {
    use super::*;
    use ::indoc::indoc;

    type MatterParserX = MatterParserWithJSON;

    const MIX_TEXT: &str = indoc!{r#"
        {
            "alfa": "bravo",
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
            "alfa": "bravo",
            "charlie": "delta"
        }
    "#};

    fn expect_state() -> State {
        serde_json::from_str(indoc!{r#"
            {
                "alfa": "bravo",
                "charlie": "delta"
            }
        "#}).unwrap()
    }

    #[test]
    fn test_parse_mix_text_to_content_text_and_matter_text_x_present() {
        let actual = MatterParserX{}.parse_mix_text_to_content_text_and_matter_text(MIX_TEXT);
        let (content_text, matter_text) = actual.unwrap();
        assert_eq!(content_text, CONTENT_TEXT);
        assert_eq!(matter_text, MATTER_TEXT);
    }

    #[test]
    fn test_parse_mix_text_to_content_text_and_matter_text_x_absent() {
        let actual = MatterParserX{}.parse_mix_text_to_content_text_and_matter_text(CONTENT_TEXT);
        assert!(actual.is_err());
    }

    #[test]
    fn test_parse_mix_text_to_content_text_and_state_x_present() {
        let actual = MatterParserX{}.parse_mix_text_to_content_text_and_state(MIX_TEXT);
        let (content_text, state) = actual.unwrap();
        assert_eq!(content_text, CONTENT_TEXT);
        assert_eq!(state, expect_state());
    }

    #[test]
    fn test_parse_mix_text_to_content_text_and_state_x_absent() {
        let actual = MatterParserX{}.parse_mix_text_to_content_text_and_state(CONTENT_TEXT);
        assert!(actual.is_err());
    }

    #[test]
    fn test_parse_matter_text_to_state() {
        let actual = MatterParserX{}.parse_matter_text_to_state(MATTER_TEXT);
        let state = actual.unwrap();
        assert_eq!(state, expect_state());
    }

}
