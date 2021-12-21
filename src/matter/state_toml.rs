pub fn insert(x: &mut ::toml::Value, key: String, value: String) -> () {
    match x {
        ::toml::Value::Table(table) => {
            table.insert(key, ::toml::Value::String(value));
        },
        _ => {
            panic!("state_toml cannot insert");
        }
    }
}
