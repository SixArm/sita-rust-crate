pub fn insert(x: &mut ::serde_yaml::Value, key: String, value: String) -> () {
    match x {
        ::serde_yaml::Value::Mapping(map) => {
            map.insert(::serde_yaml::Value::String(key), ::serde_yaml::Value::String(value));
        },
        _ => {
            panic!("state_yaml cannot insert");
        }
    }
}
