//! Markdown matter using HTML, JSON, TOML, YAML.

use crate::matter::matter_parser::MatterParser;
use crate::matter::matter_parser_with_html::MatterParserWithHTML;
use crate::matter::matter_parser_with_json::MatterParserWithJSON;
use crate::matter::matter_parser_with_toml::MatterParserWithTOML;
use crate::matter::matter_parser_with_yaml::MatterParserWithYAML;

/// Parse from input text (typically front matter and markdown content) 
/// to markdown content and matter state variables.
///
/// Example:
///
/// ```
/// let input_markdown = indoc!{r#"
/// <!--
///   alpha: bravo
///   charlie: delta
/// -->
/// echo
/// foxtrot
/// "#};
/// let (output_markdown: &str, matter: Option<Matter>) = parse(input_markdown);
/// ```
///
#[allow(dead_code)]
pub fn parse_mix_text_to_content_text_and_matter_state(mix_text: &str) -> (&str, crate::matter::state::State) {
    if let Some(x) = MatterParserWithHTML::parse_mix_text_to_content_text_and_matter_state(mix_text) { return x; };
    if let Some(x) = MatterParserWithJSON::parse_mix_text_to_content_text_and_matter_state(mix_text) { return x; };
    if let Some(x) = MatterParserWithTOML::parse_mix_text_to_content_text_and_matter_state(mix_text) { return x; };
    if let Some(x) = MatterParserWithYAML::parse_mix_text_to_content_text_and_matter_state(mix_text) { return x; };
    (mix_text, crate::matter::state::State::None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    // fn assert_discriminant_eq<T>(a: &T, b: &T) {
    //     assert_eq!(
    //         ::std::mem::discriminant(a), 
    //         ::std::mem::discriminant(b),
    //     ); 
    // }

    #[test]
    fn test_parse_mix_text_to_content_text_and_matter_text_x_html() {
        let input_markdown = indoc!{r#"
            <!--
            alpha: bravo
            charlie: delta
            -->
            echo
            foxtrot
        "#};
        let expect_content_text = indoc!{r#"
            echo
            foxtrot
        "#};
        let (content_text, matter_state) = parse_mix_text_to_content_text_and_matter_state(input_markdown);
        assert_eq!(content_text, expect_content_text);
        if let crate::matter::state::State::HTML(matter) = matter_state {
            assert_eq!(matter["alpha"].as_str(), "bravo");
            assert_eq!(matter["charlie"].as_str(), "delta");
        } else {
            panic!("HTML(matter)")
        };
    }

    #[test]
    fn test_parse_mix_text_to_content_text_and_matter_text_x_json() {
        let input_markdown = indoc!{r#"
            {
                "alpha": "bravo",
                "charlie": "delta"
            }
            echo
            foxtrot
        "#};
        let expect_content_text = indoc!{r#"
            echo
            foxtrot
        "#};
        let (content_text, matter_state) = parse_mix_text_to_content_text_and_matter_state(input_markdown);
        assert_eq!(content_text, expect_content_text);
        if let crate::matter::state::State::JSON(matter) = matter_state {
            assert_eq!(matter["alpha"].as_str().unwrap(), "bravo");
            assert_eq!(matter["charlie"].as_str().unwrap(), "delta");
        } else {
            panic!("JSON(matter)")
        };
    }

    #[test]
    fn test_parse_mix_text_to_content_text_and_matter_text_x_toml() {
        let input_markdown = indoc!{r#"
            +++
            alpha = "bravo"
            charlie = "delta"
            +++
            echo
            foxtrot
        "#};
        let expect_content_text = indoc!{r#"
            echo
            foxtrot
        "#};
        let (content_text, matter_state) = parse_mix_text_to_content_text_and_matter_state(input_markdown);
        assert_eq!(content_text, expect_content_text);
        if let crate::matter::state::State::TOML(matter) = matter_state {
            assert_eq!(matter["alpha"].as_str().unwrap(), "bravo");
            assert_eq!(matter["charlie"].as_str().unwrap(), "delta");
        } else {
            panic!("TOML(matter)")
        };
    }

    #[test]
    fn test_parse_mix_text_to_content_text_and_matter_text_x_yaml() {
        let input_markdown = indoc!{r#"
            ---
            alpha: bravo
            charlie: delta
            ---
            echo
            foxtrot
        "#};
        let expect_content_text = indoc!{r#"
            echo
            foxtrot
        "#};
        let (content_text, matter_state) = parse_mix_text_to_content_text_and_matter_state(input_markdown);
        assert_eq!(content_text, expect_content_text);
        if let crate::matter::state::State::YAML(matter) = matter_state {
            assert_eq!(matter["alpha"].as_str().unwrap(), "bravo");
            assert_eq!(matter["charlie"].as_str().unwrap(), "delta");
        } else {
            panic!("YAML(matter)")
        };
    }

}
