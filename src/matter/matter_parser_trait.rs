//! Markdown matter using HTML, JSON, TOML, YAML.

use std::any::Any;

pub trait MatterParserTrait<STATE, ERROR> {

    /// Convert from the specific type to any type.
    #[allow(dead_code)]
    fn as_any(&self) -> &dyn Any;

    /// Parse mix text to content text and matter text.
    fn parse_mix_text_to_content_text_and_matter_text(&self, mix_text: &str) -> Result<(String, String), ERROR>;

    /// Parse mix text to content text and state.
    fn parse_mix_text_to_content_text_and_state(&self, mix_text: &str) -> Result<(String, STATE), ERROR>;

    /// Parse matter text to state.
    fn parse_matter_text_to_state(&self, matter_text: &str) -> Result<STATE, ERROR>;

}
