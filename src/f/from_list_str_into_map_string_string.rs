use crate::types::{list::*, map::*};

/// Convert from &Vec<&str> into Map<String, String>.
///
/// Example:
//
/// ```rust
/// let from: Vec<&str> = vec!["alfa", "bravo", "charlie", "delta"];
/// let int: Map<String, String> = from_list_str_into_map_string_string(&from);
/// //=> ["alfa" => "bravo", "charlie" => "delta"]
/// ```
///
#[allow(dead_code)]
pub fn from_list_str_into_map_string_string(list_str: &List<&str>) -> Map<String, String> {
    trace!("from_list_str_into_map_string_string ➡ list_str: {:?}", list_str);
    let mut map: Map<String, String> = Map::new();
    for i in (0..list_str.len()-1).step_by(2) {
        let k = String::from(list_str[i]);
        let v = String::from(list_str[i+1]);
        map.insert(k, v);
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_list_str_into_map_string_string() {
        let from: Vec<&str> = vec!["alfa", "bravo", "charlie", "delta"];
        let actual: Map<String, String> = from_list_str_into_map_string_string(&from);
        let mut expect: Map<String, String> = Map::new();
        expect.insert("alfa".into(), "bravo".into());
        expect.insert("charlie".into(), "delta".into());
        assert_eq!(actual, expect);
    }

}
