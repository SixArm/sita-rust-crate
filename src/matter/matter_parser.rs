//! Markdown matter using HTML, JSON, TOML, YAML.

pub trait MatterParser {
    fn parse_mix_text_to_content_text_and_matter_text(mix_text: &str) -> Option<(&str, &str)>;
    fn parse_mix_text_to_content_text_and_matter_state(mix_text: &str) -> Option<(&str, crate::matter::state::State)> {
        if let Some((content_text, matter_text)) = Self::parse_mix_text_to_content_text_and_matter_text(mix_text) {
            Some((
                content_text,
                Self::parse_matter_text_to_matter_state(matter_text),
            ))
        } else {
            None
        }
    }
    fn parse_matter_text_to_matter_state<S: AsRef<str> + Sized>(matter_text: S) -> crate::matter::state::State;
}
