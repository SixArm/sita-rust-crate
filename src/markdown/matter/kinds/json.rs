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
pub fn extract(input: &str) -> Option<(&str, &str)> {
    if let Some(captures) = REGEX.captures(input) {
        Some((
            captures.name("markdown").unwrap().as_str(),
            captures.name("matter").unwrap().as_str(),
        ))
    } else {
        None
    }
}

/// Parse a block of text to a JSON value.
///
/// Example:
///
/// ```
/// let block = indoc!{r#"
///     {
///         "alpha": "bravo",
///         "charlie": "delta"
///     }
/// "#};
/// let x: ::serde_json::Value = parse(&block).unwrap();
/// assert_eq!(x["alpha"], "bravo");
/// assert_eq!(x["charlie"], "delta");
/// ```
///
#[allow(dead_code)]
pub fn parse<S: AsRef<str> + Sized>(text: S) -> Result<::serde_json::Value, ::serde_json::Error> {
    ::serde_json::from_str(text.as_ref())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::indoc::indoc;

    #[test]
    fn test_extract_x_present() {
        let input_markdown = indoc!{r#"
            {
                "alpha": "bravo",
                "charlie": "delta"
            }
            echo
            foxtrot
        "#};
        let expect_matter = indoc!{r#"
            {
                "alpha": "bravo",
                "charlie": "delta"
            }
        "#};
        let expect_markdown = indoc!{r#"
            echo
            foxtrot
        "#};
        let option = extract(input_markdown);
        assert!(option.is_some());
        let (actual_markdown, actual_matter) = option.unwrap();
        assert_eq!(actual_markdown, expect_markdown);
        assert_eq!(actual_matter, expect_matter);
    }

    #[test]
    fn test_extract_x_absent() {
        let input_markdown = indoc!{r#"
            echo
            foxtrot
        "#};
        let option = extract(input_markdown);
        assert!(option.is_none());
    }


    #[test]
    fn test_parse() {
        let s = indoc!{r#"
            {
                "alpha": "bravo",
                "charlie": "delta"
            }
        "#};
        let matter_result = parse(s);
        assert!(matter_result.is_ok());
        let matter = matter_result.unwrap();
        assert_eq!(matter["alpha"].as_str().unwrap(), "bravo");
        assert_eq!(matter["charlie"].as_str().unwrap(), "delta");
    }

}
