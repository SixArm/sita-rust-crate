//! Markdown matter using TOML front matter.

use once_cell::sync::Lazy;
use regex::Regex;

// #[allow(dead_code)]
// pub fn blank() -> ::toml::Value {
//     "".parse::<::toml::Value>().unwrap()
// }

pub static REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?m)(?s)\A\+\+\+\n(?P<matter>.*?)\n\+\+\+\n(?P<markdown>.*)\z").unwrap()
});

#[allow(dead_code)]
pub fn extract(input: &str) -> (&str, Option<Result<::toml::Value, ::toml::de::Error>>) {
    if let Some(captures) = REGEX.captures(input) {
        if let Some(matter) = captures.name("matter") {
            if let Some(markdown) = captures.name("markdown") {
                return (
                    markdown.as_str(),
                    Some(matter.as_str().parse::<::toml::Value>()),
                )
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
    //     let actual: ::toml::Value = super::blank();
    //     let expect: ::toml::Value = "".parse::<::toml::Value>().unwrap();
    //     assert_eq!(actual, expect);
    // }

    #[test]
    fn test_present() {
        let input_markdown = indoc!{r#"
            +++
            alpha = "bravo"
            charlie = "delta"
            +++
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
