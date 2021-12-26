//! Markdown matter using HTML, JSON, TOML, YAML.

use std::any::Any;
use crate::errors::*;

pub trait MatterParserTrait<STATE> {
    fn as_any(&self) -> &dyn Any;
    fn parse_mix_text_to_content_text_and_matter_text(&self, mix_text: &str) -> Result<(String, String)>;
    fn parse_mix_text_to_content_text_and_state(&self, mix_text: &str) -> Result<(String, STATE)> {
        if let Ok((content_text, matter_text)) = self.parse_mix_text_to_content_text_and_matter_text(mix_text) {
            if let Ok(state) = self.parse_matter_text_to_state(&matter_text) {
                return Ok((content_text, state));
            }
        }
        Err("Failed to parse mix text to content text and state.".into())
    }
    fn parse_matter_text_to_state(&self, matter_text: &str) -> Result<STATE>;
}
