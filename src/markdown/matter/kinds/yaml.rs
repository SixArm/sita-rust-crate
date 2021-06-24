//! Markdown matter using YAML front matter.

use once_cell::sync::Lazy;
use regex::Regex;
use serde_yaml;

// TODO replace these older chunks...
//
//   ::yaml_rust::yaml::Yaml -> serde_yaml::Value
//   ::yaml_rust::ScanError -> serde_yaml::Error
//   ::yaml_rust::YamlLoader::load_from_str -> serde_yaml::from_str

// #[allow(dead_code)]
// pub fn blank() -> ::yaml_rust::yaml::Yaml {
//     let docs = ::yaml_rust::YamlLoader::load_from_str("").unwrap();
//     let doc = &docs[0];
//     doc.clone()
// }

pub static REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?m)(?s)\A---\n(?P<matter>.*?)\n---\n(?P<markdown>.*)\z").unwrap()
});

#[allow(dead_code)]
pub fn extract(input: &str) -> (&str, Option<Result<serde_yaml::Value, serde_yaml::Error>>) {
    if let Some(captures) = REGEX.captures(input) {
        if let Some(matter) = captures.name("matter") {
            if let Some(markdown) = captures.name("markdown") {
                match serde_yaml::from_str(matter.as_str()) {
                    Ok(x) => {
                        return (
                            markdown.as_str(),
                            Some(Ok(x)),
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

    // #[test]
    // fn test_blank() {
    //     let actual: ::yaml_rust::yaml::Yaml = super::blank();
    //     let expect: ::yaml_rust::yaml::Yaml = (&(::yaml_rust::YamlLoader::load_from_str("").unwrap()))[0].clone();
    //     assert_eq!(actual, expect);
    // }

    #[test]
    fn test_present() {
        let input_markdown = indoc!{r#"
            ---
            alpha: bravo
            charlie: delta
            ---
            echo
            foxtrot
        "#};
        let expect_markdown = indoc!{r#"
            echo
            foxtrot
        "#};
        let (output_markdown, matter_option) = extract(input_markdown);
        assert_eq!(output_markdown, expect_markdown);
        assert!(matter_option.is_some());
        let matter_result = matter_option.unwrap();
        assert!(matter_result.is_ok());
        let matter = matter_result.unwrap();
        assert_eq!(matter["alpha"].as_str().unwrap(), "bravo");
        assert_eq!(matter["charlie"].as_str().unwrap(), "delta");
    }

    #[test]
    fn test_absent() {
        let input_markdown = indoc!{r#"
            echo
            foxtrot
        "#};
        let (output_markdown, matter_option) = extract(input_markdown);
        assert_eq!(output_markdown, input_markdown);
        assert!(matter_option.is_none());
    }

}
