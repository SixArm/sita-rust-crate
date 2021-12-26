//! State management.
//!
//! This can use HTML, JSON, TOML, YAML.
//! This can be expanded for potential future formats.

#[derive(Debug)]
pub enum StateEnum {
    StateWithMap(crate::state::state_with_map::StateWithMap),
    StateWithJSON(crate::state::state_with_json::StateWithJSON),
    StateWithTOML(crate::state::state_with_toml::StateWithTOML),
    StateWithYAML(crate::state::state_with_yaml::StateWithYAML),
}
