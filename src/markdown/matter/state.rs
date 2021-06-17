#[derive(Debug)]
pub enum State {
    HTML(::std::collections::BTreeMap<String, String>),
    JSON(::serde_json::Value),
    TOML(::toml::Value),
    YAML(::serde_yaml::Value),
    None
}
