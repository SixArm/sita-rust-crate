//! State management.
//!
//! This is a state manager that can handle a variety of formats:
//!
//! * Map: Map struct (for state via matter parser HTML or Markdown comments)
//! * JSON: JavaScript Object Notation
//! * TOML: Tom's Obvious Markup Language
//! * YAML: Yet Anther Markup Language
//!
//! This can be expanded for potential future formats.

#[derive(Debug)]
pub enum StateEnum {
    StateWithMap(crate::state::state_with_map::StateWithMap),
    StateWithJSON(crate::state::state_with_json::StateWithJSON),
    StateWithTOML(crate::state::state_with_toml::StateWithTOML),
    StateWithYAML(crate::state::state_with_yaml::StateWithYAML),
}
