//! Markdown matter using TOML front matter.

use std::any::Any;
use once_cell::sync::Lazy;
use regex::Regex;
use crate::matter::matter_parser_trait::MatterParserTrait;

/// State alias is for this file's generic implementation.
type State = crate::state::state_with_toml::StateWithTOML;

#[derive(Debug)]
pub struct MatterParserWithTOML {
}

impl MatterParserTrait<State, Error> for MatterParserWithTOML {

    fn as_any(&self) -> &dyn Any {
        self
    }

    /// Example:
    ///
    /// ```
    /// # use ::indoc::indoc;
    /// let mix_text = indoc!{r#"
    ///     +++
    ///     alfa = "bravo"
    ///     charlie = "delta"
    ///     +++
    ///     echo
    ///     foxtrot
    /// "#};
    /// let content_text, matter_text = parse_mix_text_to_content_text_and_matter_text(mix_text).unwrap();
    /// assert_eq!(content_text, indoc!{r#"
    ///     echo
    ///     foxtrot
    /// "#};
    /// assert_eq!(matter_text, indoc!{r#"
    ///     alfa = "bravo"
    ///     charlie = "delta"
    /// "#};
    /// ```
    #[allow(dead_code)]
    fn parse_mix_text_to_content_text_and_matter_text(&self, mix_text: &str) -> Result<(String, String), Error> {
        trace!("{} ➡ parse_mix_text_to_content_text_and_matter_text", file!());
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
    /// let matter_text = indoc!{r#"
    ///     alfa = "bravo"
    ///     charlie = "delta"
    /// "#};
    /// let state = parse_matter_text_to_state(mix_text).unwrap();
    /// assert_eq!(state.get("alfa"), String::from("bravo"));
    /// assert_eq!(state.get("charlie"), String::from("delta"));
    /// ```
    #[allow(dead_code)]
    fn parse_matter_text_to_state(&self, matter_text: &str) -> Result<State, Error> {
        trace!("{} ➡ parse_matter_text_to_state", file!());
        match matter_text.parse::<::toml::Value>() {
            Ok(toml) => {
                match toml {
                    ::toml::Value::Table(table) => Ok(table),
                    value => Err(Error::ParseMatterTextToStateMustBeMap(value))
                }
            },
            Err(e) => {
                Err(Error::ParseMatterTextToState(e))
            }
        }
    }

}

pub static REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?m)(?s)\A\+\+\+\n(?P<matter>.*?\n)\+\+\+\n(?P<content>.*)\z").unwrap()
});

#[derive(thiserror::Error, Debug)]
pub enum Error {

    #[error("ParseMixTextToContentTextAndMatterText ➡ mix_text: {mix_text}")]
    ParseMixTextToContentTextAndMatterText {
        mix_text: String,
    },

    #[error("ParseMatterTextToState ➡ {0:?}")]
    ParseMatterTextToState(toml::de::Error),

    #[error("ParseMatterTextToStateMustBeMap ➡ {0:?}")]
    ParseMatterTextToStateMustBeMap(toml::Value),

}

#[cfg(test)]
mod tests {
    use super::*;
    use ::indoc::indoc;

    type MatterParserX = MatterParserWithTOML;

    const MIX_TEXT: &str = indoc!{r#"
        +++
        alfa = "bravo"
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
        alfa = "bravo"
        charlie = "delta"
    "#};

    fn expect_state() -> State {
        toml::from_str(indoc!{r#"
            alfa = "bravo"
            charlie = "delta"
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
