pub fn insert(x: &mut ::serde_json::Value, key: String, value: String) -> () {
    match x {
        ::serde_json::Value::Object(map) => {
            map.insert(key, ::serde_json::Value::String(value));
        },
        _ => {
            panic!("state_json cannot insert");
        }
    }
}
