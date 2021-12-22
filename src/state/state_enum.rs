//! State management.
//!
//! This can use HTML, JSON, TOML, YAML.
//! This can be expanded for potential future formats.

#[derive(Debug)]
pub enum StateEnum {
    StateWithHTML(crate::state::state_with_html::StateWithHTML),
    StateWithJSON(crate::state::state_with_json::StateWithJSON),
    StateWithTOML(crate::state::state_with_toml::StateWithTOML),
    StateWithYAML(crate::state::state_with_yaml::StateWithYAML),
}
