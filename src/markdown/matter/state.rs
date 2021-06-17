#[derive(Debug)]
pub enum State {
    HTML(::std::collections::HashMap<String, String>),
    JSON(::serde_json::Value),
    TOML(::toml::Value),
    YAML(::yaml_rust::yaml::Yaml),
    None
}
