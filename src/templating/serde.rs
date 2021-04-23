//! serde verification
//!
//! These tests verify that the `serde` crate and its helpers
//! are working the way we expect, using the types we expect.
//!
//! We test 3 types: JSON, TOML, YAML.
//!
//! We test each type by using a primary-level number, string, and boolean,
//! and also a secondary-level number, string, and boolean.

#[cfg(test)]
mod tests {

    #[test]
    fn test_x_json() {
        let json_str = r#"
        {
          "alpha": 123,
          "bravo": "charlie",
          "delta": true,
          "echo": {
            "foxtrot": 456,
            "golf": "hotel",
            "india": true
          }
        }
        "#;
        let result: Result<::serde_json::Value, ::serde_json::Error> = ::serde_json::from_str(json_str);
        assert!(result.is_ok());
        let x: ::serde_json::Value = result.unwrap();
        assert_eq!(x["bravo"].as_str().unwrap(), "charlie");
    }

    #[test]
    fn test_x_toml() {
        let toml_str = r#"
        alpha = 7
        bravo = "charlie"
        delta = true

        [echo]
        foxtrot = 456
        golf = "hotel"
        india = true
        "#;
        let result: Result<::toml::Value, ::toml::de::Error> = ::toml::from_str(toml_str);
        assert!(result.is_ok());
        let x: ::toml::Value = result.unwrap();
        assert_eq!(x["bravo"].as_str().unwrap(), "charlie");
    }

    #[test]
    fn test_x_yaml() {
        let yaml_str = r#"
        alpha: 123
        bravo: "charlie"
        delta: true
        echo:
            foxtrot: 456
            golf: "hotel"
            india: true
        "#;
        let result: Result<::serde_yaml::Value, ::serde_yaml::Error> = ::serde_yaml::from_str(yaml_str);
        assert!(result.is_ok());
        let x: ::serde_yaml::Value = result.unwrap();
        assert_eq!(x["bravo"].as_str().unwrap(), "charlie");
    }

}
