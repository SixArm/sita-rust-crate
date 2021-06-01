use once_cell::sync::Lazy;
use regex::Regex;

pub static BLANK: Lazy<::toml::Value> = Lazy::new(|| {
    "".parse::<::toml::Value>().unwrap()
});

pub static REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?m)(?s)\A<!--\n(?P<front>.*?)\n-->\n(?P<markdown>.*)\z").unwrap()
});

//TODO warn dead code
#[allow(dead_code)]
pub fn extract(input: &str) -> (&str, Option<Result<::toml::Value, ::toml::de::Error>>) {
    if let Some(captures) = REGEX.captures(input) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use ::indoc::indoc;

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
        let (output_markdown, front_option) = extract(input_markdown);
        assert_eq!(output_markdown, expect_markdown);
        assert!(front_option.is_some());
        let front_result = front_option.unwrap();
        assert!(front_result.is_ok());
        let front = front_result.unwrap();
        assert_eq!(front["alpha"].as_str().unwrap(), "bravo");
        assert_eq!(front["charlie"].as_str().unwrap(), "delta");
    }

    #[test]
    fn test_absent() {
        let input_markdown = indoc!{r#"
            echo
            foxtrot
        "#};
        let (output_markdown, front_option) = extract(input_markdown);
        assert_eq!(output_markdown, input_markdown);
        assert!(front_option.is_none());
    }

}
