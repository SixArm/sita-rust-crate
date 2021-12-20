//! Markdown matter using HTML front matter.

use once_cell::sync::Lazy;
use regex::Regex;
use crate::types::*;

pub static REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?m)(?s)\A<!--\n(?P<matter>.*?)\n-->\n(?P<markdown>.*)\z").unwrap()
});

pub static PARSE_LINE_TO_KEY_VALUE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\A\s*(?P<key>\w+?):\s*(?P<value>.*?)\s*\z").unwrap()
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

/// Parse a block of text to a map.
///
/// Example:
///
/// ```
/// let block = indoc!{r#"
///     alpha: bravo
///     charlie: delta
/// "#};
/// let x: Map<String, String> = parse(&block);
/// assert_eq!(x["alpha"], "bravo");
/// assert_eq!(x["charlie"], "delta");
/// ```
///
#[allow(dead_code)]
pub fn parse<S: AsRef<str> + Sized>(text: S) -> Map<String, String> {
    let mut map: Map<String, String> = Map::new();
    for line in text.as_ref().split("\n") {
        if let Some(captures) = (*PARSE_LINE_TO_KEY_VALUE_REGEX).captures(line) {
            if let Some(key) = captures.name("key") {
                if let Some(value) = captures.name("value") {
                    map.insert(String::from(key.as_str()), String::from(value.as_str()));
                }
            }
        }
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::indoc::indoc;

    #[test]
    fn test_extract_x_present() {
        let input_markdown = indoc!{r#"
            <!--
            alpha: bravo
            charlie: delta
            -->
            echo
            foxtrot
        "#};
        let expect_markdown = indoc!{r#"
            echo
            foxtrot
        "#};
        let expect_matter = indoc!{r#"
            alpha: bravo
            charlie: delta
        "#};
        let option = extract(input_markdown);
        assert!(option.is_some());        
        let (actual_markdown, actual_matter) = option.unwrap();
        assert_eq!(actual_markdown, expect_markdown);
        assert_eq!(actual_matter, expect_matter);
        let matter = parse(actual_matter);
        assert_eq!(matter["alpha"], "bravo");
        assert_eq!(matter["charlie"], "delta");
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
        let block = indoc!{r#"
            alpha: bravo
            charlie: delta
        "#};
        let actual: Map<String, String> = parse_block_to_map(&block);
        assert_eq!(actual["alpha"], "bravo");
        assert_eq!(actual["charlie"], "delta");
    }

}
