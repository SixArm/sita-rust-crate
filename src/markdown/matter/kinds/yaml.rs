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

/// Parse a block of text to a YAML value.
///
/// Example:
///
/// ```
/// let text = indoc!{r#"
///     alpha: bravo
///     charlie: delta
/// "#};
/// let x: ::serde_yaml::Value = parse(&text).unwrap();
/// assert_eq!(x["alpha"], "bravo");
/// assert_eq!(x["charlie"], "delta");
/// ```
///
#[allow(dead_code)]
pub fn parse<S: AsRef<str> + Sized>(text: S) -> Result<::serde_yaml::Value, ::serde_yaml::Error> {
    serde_yaml::from_str(text.as_ref())
}

/// Parse a block of text to a matter state struct YAML enum.
///
/// Example:
///
/// ```
/// let text = indoc!{r#"
///     alpha: bravo
///     charlie: delta
/// "#};
/// let state: crate::markdown::matter::state::State = parse_to_state(&text);
/// assert_eq!(state["alpha"], "bravo");
/// assert_eq!(state["charlie"], "delta");
/// ```
///
#[allow(dead_code)]
pub fn parse_to_state<S: AsRef<str> + Sized>(text: S) -> crate::markdown::matter::state::State {
    match parse(text) {
        Ok(x) => crate::markdown::matter::state::State::YAML(x),
        _ => crate::markdown::matter::state::State::None,
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use ::indoc::indoc;

    #[test]
    fn test_extract_x_present() {
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
        let expect_matter = indoc!{r#"
            alpha: bravo
            charlie: delta
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
        let text = indoc!{r#"
            alpha: bravo
            charlie: delta
        "#};
        let matter_result = parse(&text);
        assert!(matter_result.is_ok());
        let matter = matter_result.unwrap();
        assert_eq!(matter["alpha"].as_str().unwrap(), "bravo");
        assert_eq!(matter["charlie"].as_str().unwrap(), "delta");
    }

    #[test]
    fn test_parse_to_state() {
        let text = indoc!{r#"
            alpha: bravo
            charlie: delta
        "#};
        let _state = parse_to_state(&text);
        //TODO
    }

}
