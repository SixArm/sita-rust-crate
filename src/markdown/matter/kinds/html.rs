use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn blank() -> HashMap<String, String> {
    HashMap::new()
}

pub static REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?m)(?s)\A<!--\n(?P<matter>.*?)\n-->\n(?P<markdown>.*)\z").unwrap()
});

#[allow(dead_code)]
pub fn extract(input: &str) -> (&str, Option<HashMap<String, String>>) {
    if let Some(captures) = REGEX.captures(input) {
        if let Some(matter) = captures.name("matter") {
            if let Some(markdown) = captures.name("markdown") {
                return (markdown.as_str(), Some(parse_block_to_map(matter.as_str())))
            }
        }
    }
    (input, None)
}

pub static PARSE_LINE_TO_KEY_VALUE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\A\s*(?P<key>\w+?):\s*(?P<value>.*?)\s*\z").unwrap()
});

/// Parse a block of text to a map.
///
/// Example:
///
/// ```
/// let block = indoc!{r#"
///     alpha: bravo
///     charlie: delta
/// "#};
/// let x: HashMap<String, String> = parse_block_to_map(&block);
/// assert_eq!(x["alpha"], "bravo");
/// assert_eq!(x["charlie"], "delta");
/// ```
///
#[allow(dead_code)]
pub fn parse_block_to_map(text: &str) -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();
    for line in text.split("\n") {
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
    use std::collections::HashMap;

    use super::*;
    use ::indoc::indoc;

    #[test]
    fn test_blank() {
        let actual = super::blank();
        let expect: HashMap<String, String> = HashMap::new();
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_present() {
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
        let (output_markdown, matter_option) = extract(input_markdown);
        assert_eq!(output_markdown, expect_markdown);
        assert!(matter_option.is_some());
        let map: HashMap<String, String> = matter_option.unwrap();
        assert_eq!(map["alpha"], "bravo");
        assert_eq!(map["charlie"], "delta");
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

    #[test]
    fn test_parse_block_to_map() {
        let block = indoc!{r#"
            alpha: bravo
            charlie: delta
        "#};
        let actual: HashMap<String, String> = parse_block_to_map(&block);
        assert_eq!(actual["alpha"], "bravo");
        assert_eq!(actual["charlie"], "delta");
    }
}
