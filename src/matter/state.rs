//! Markdown matter state management.
//!
//! This can use HTML, JSON, TOML, YAML, or None.

use crate::errors::*;
use crate::types::*;

#[derive(Debug)]
pub enum State {
    HTML(Map<String, String>),
    JSON(::serde_json::Value),
    TOML(::toml::Value),
    YAML(::serde_yaml::Value),
    None
}

pub fn insert(state: &mut State, key: String, value: String) -> () {
    match state {
        crate::matter::state::State::HTML(x) => crate::matter::state_html::insert(x, key, value),
        crate::matter::state::State::JSON(x) => crate::matter::state_json::insert(x, key, value),
        crate::matter::state::State::TOML(x) => crate::matter::state_toml::insert(x, key, value),
        crate::matter::state::State::YAML(x) => crate::matter::state_yaml::insert(x, key, value),
        crate::matter::state::State::None => { panic!("State::None cannot insert"); }
    }
}
