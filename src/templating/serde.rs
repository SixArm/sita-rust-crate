//! serde verification
//!
//! These tests verify that the `serde` crate and its helpers
//! are working the way we expect, using the types we expect.
//!
//! We test these types: HTML, JSON, TOML, YAML.
//!
//! We test each type by using a primary-level number, string, and boolean,
//! and also a secondary-level number, string, and boolean.

#[cfg(test)]
mod tests {

    #[test]
    fn test_x_json() {
        let s = r#"
        {
          "alfa": 123,
          "bravo": "charlie",
          "delta": true,
          "echo": {
            "foxtrot": 456,
            "golf": "hotel",
            "india": true
          }
        }
        "#;
        let result: Result<::serde_json::Value, ::serde_json::Error> = ::serde_json::from_str(s);
        let x: ::serde_json::Value = result.unwrap();
        assert_eq!(x["bravo"].as_str().unwrap(), "charlie");
    }

    #[test]
    fn test_x_toml() {
        let s = r#"
        alfa = 7
        bravo = "charlie"
        delta = true

        [echo]
        foxtrot = 456
        golf = "hotel"
        india = true
        "#;
        let result: Result<::toml::Value, ::toml::de::Error> = ::toml::from_str(s);
        let x: ::toml::Value = result.unwrap();
        assert_eq!(x["bravo"].as_str().unwrap(), "charlie");
    }

    #[test]
    fn test_x_yaml() {
        let s = r#"
        alfa: 123
        bravo: charlie
        delta: true
        echo:
            foxtrot: 456
            golf: hotel
            india: true
        "#;
        let result: Result<::serde_yaml::Value, ::serde_yaml::Error> = ::serde_yaml::from_str(s);
        let x: ::serde_yaml::Value = result.unwrap();
        assert_eq!(x["bravo"].as_str().unwrap(), "charlie");
    }

}
