use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref MARKDOWN_FRONT_MATTER_REGEX_AS_JSON: Regex
    = Regex::new(r"(?m)(?s)\A(?P<front>\{\n.*?\n\}\n)(?P<markdown>.*)\z").unwrap();
}

lazy_static! {
    static ref MARKDOWN_FRONT_MATTER_REGEX_AS_TOML: Regex
    = Regex::new(r"(?m)(?s)\A\+\+\+\n(?P<front>.*?)\n\+\+\+\n(?P<markdown>.*)\z").unwrap();
}

lazy_static! {
    static ref MARKDOWN_FRONT_MATTER_REGEX_AS_YAML: Regex
    = Regex::new(r"(?m)(?s)(?P<front>---\n.*?\n---)\n(?P<markdown>.*)\z").unwrap();
}

fn extract_front_matter_as_json(input: &str) -> (&str, Option<Result<::serde_json::Value, ::serde_json::Error>>) {
    if let Some(captures) = MARKDOWN_FRONT_MATTER_REGEX_AS_JSON.captures(input) {
        if let Some(front) = captures.name("front") {
            if let Some(markdown) = captures.name("markdown") {
                return (
                    markdown.as_str(),
                    Some(::serde_json::from_str(front.as_str()))
                )
            }
        }
    }
    (input, None)
}

fn extract_front_matter_as_toml(input: &str) -> (&str, Option<Result<::toml::Value, ::toml::de::Error>>) {
    if let Some(captures) = MARKDOWN_FRONT_MATTER_REGEX_AS_TOML.captures(input) {
        if let Some(front) = captures.name("front") {
            if let Some(markdown) = captures.name("markdown") {
                return (
                    markdown.as_str(),
                    Some(front.as_str().parse::<::toml::Value>()),
                )
            }
        }
    }
    (input, None)
}

fn extract_front_matter_as_yaml(input: &str) -> (&str, Option<Result<::yaml_rust::yaml::Yaml, ::yaml_rust::ScanError>>) {
    if let Some(captures) = MARKDOWN_FRONT_MATTER_REGEX_AS_YAML.captures(input) {
        if let Some(front) = captures.name("front") {
            if let Some(markdown) = captures.name("markdown") {
                match ::yaml_rust::YamlLoader::load_from_str(front.as_str()) {
                    Ok(vec) => {
                        return (
                            markdown.as_str(),
                            Some(Ok(vec[0].to_owned())),
                        )
                    },
                    Err(e) => {
                        return (
                            markdown.as_str(),
                            Some(Err(e)),
                        )
                    }
                }
            }
        }
    }
    (input, None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::indoc::indoc;
    //use crate::vars::Vars;

    #[test]
    fn test_with_json() {
        let input_markdown = indoc! {r#"
            {
                "alpha": "bravo",
                "charlie": "delta"
            }
            echo
            foxtrot
        "#};
        let expect_markdown = indoc! {r#"
            echo
            foxtrot
        "#};
        let (output_markdown, front_option) = extract_front_matter_as_json(input_markdown);
        assert_eq!(output_markdown, expect_markdown);
        assert!(front_option.is_some());
        let front_result = front_option.unwrap();
        assert!(front_result.is_ok());
        let front = front_result.unwrap();
        assert_eq!(front["alpha"].as_str().unwrap(), "bravo");
        assert_eq!(front["charlie"].as_str().unwrap(), "delta");
    }

    #[test]
    fn test_sans_json() {
        let input_markdown = indoc! {r#"
            echo
            foxtrot
        "#};
        let (output_markdown, json_option) = extract_front_matter_as_json(input_markdown);
        assert_eq!(output_markdown, input_markdown);
        assert!(json_option.is_none());
    }

    #[test]
    fn test_with_toml() {
        let input_markdown = indoc! {r#"
            +++
            alpha = "bravo"
            charlie = "delta"
            +++
            echo
            foxtrot
        "#};
        let expect_markdown = indoc! {r#"
            echo
            foxtrot
        "#};
        let (output_markdown, front_option) = extract_front_matter_as_toml(input_markdown);
        assert_eq!(output_markdown, expect_markdown);
        assert!(front_option.is_some());
        let front_result = front_option.unwrap();
        assert!(front_result.is_ok());
        let front = front_result.unwrap();
        assert_eq!(front["alpha"].as_str().unwrap(), "bravo");
        assert_eq!(front["charlie"].as_str().unwrap(), "delta");
    }

    #[test]
    fn test_sans_toml() {
        let input_markdown = indoc! {r#"
            echo
            foxtrot
        "#};
        let (output_markdown, front_option) = extract_front_matter_as_toml(input_markdown);
        assert_eq!(output_markdown, input_markdown);
        assert!(front_option.is_none());
    }

    #[test]
    fn test_with_yaml() {
        let input_markdown = indoc! {r#"
            ---
            alpha: bravo
            charlie: delta
            ---
            echo
            foxtrot
        "#};
        let expect_markdown = indoc! {r#"
            echo
            foxtrot
        "#};
        let (output_markdown, front_option) = extract_front_matter_as_yaml(input_markdown);
        assert_eq!(output_markdown, expect_markdown);
        assert!(front_option.is_some());
        let front_result = front_option.unwrap();
        assert!(front_result.is_ok());
        let front: ::yaml_rust::yaml::Yaml = front_result.unwrap();
        assert_eq!(front["alpha"].as_str().unwrap(), "bravo");
        assert_eq!(front["charlie"].as_str().unwrap(), "delta");
    }

    #[test]
    fn test_sans_yaml() {
        let input_markdown = indoc! {r#"
            echo
            foxtrot
        "#};
        let (output_markdown, front_option) = extract_front_matter_as_yaml(input_markdown);
        assert_eq!(output_markdown, input_markdown);
        assert!(front_option.is_none());
    }

}
