use crate::types::*;

#[derive(Debug)]
pub enum State {
    HTML(Map<String, String>),
    JSON(::serde_json::Value),
    TOML(::toml::Value),
    YAML(::serde_yaml::Value),
    None
}
