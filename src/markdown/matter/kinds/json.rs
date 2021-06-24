//! Markdown matter using JSON front matter.

use once_cell::sync::Lazy;
use regex::Regex;

// #[allow(dead_code)]
// pub fn blank() -> ::serde_json::Value {
//     ::serde_json::from_str("").unwrap()
// }

pub static REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?m)(?s)\A(?P<matter>\{.*?\n\})\n(?P<markdown>.*)\z").unwrap()
});

#[allow(dead_code)]
pub fn extract(input: &str) -> (&str, Option<Result<::serde_json::Value, ::serde_json::Error>>) {
    if let Some(captures) = REGEX.captures(input) {
        if let Some(matter) = captures.name("matter") {
            if let Some(markdown) = captures.name("markdown") {
                return (
                    markdown.as_str(),
                    Some(::serde_json::from_str(matter.as_str()))
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
    //     let actual: ::serde_json::Value = super::blank();
    //     let expect: ::serde_json::Value = ::serde_json::from_str("").unwrap();
    //     assert_eq!(actual, expect);
    // }

    #[test]
    fn test_present() {
        let input_markdown = indoc!{r#"
            {
                "alpha": "bravo",
                "charlie": "delta"
            }
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
        let (output_markdown, json_option) = extract(input_markdown);
        assert_eq!(output_markdown, input_markdown);
        assert!(json_option.is_none());
    }

}
