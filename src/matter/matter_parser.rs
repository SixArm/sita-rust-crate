//! Markdown matter using HTML, JSON, TOML, YAML.

use std::any::Any;

pub trait MatterParser<STATE> {
    fn as_any(&self) -> &dyn Any;
    fn parse_mix_text_to_content_text_and_matter_text<S: Into<String>>(&self, mix_text: S) -> Option<(String, String)>;
    fn parse_mix_text_to_content_text_and_state<S: Into<String>>(&self, mix_text: S) -> Option<(String, STATE)> {
        if let Some((content_text, matter_text)) = self.parse_mix_text_to_content_text_and_matter_text(mix_text) {
            if let Some(state) = self.parse_matter_text_to_state(matter_text) {
                return Some((content_text, state));
            }
        }
        None
    }
    fn parse_matter_text_to_state<S: Into<String>>(&self, matter_text: S) -> Option<STATE>;
}
