//! Markdown matter using HTML, JSON, TOML, YAML.

/// Extract from markdown input to markdown output and matter variables.
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
/// let (output_markdown: &str, matter: Option<Matter>) = extract(input_markdown);
/// ```
///
#[allow(dead_code)]
pub fn extract(input: &str) -> (&str, crate::markdown::matter::state::State) {
    match input.chars().next() {
        Some('<') => {
            let (markdown, matter) = crate::markdown::matter::kinds::html::extract(input);
            if let Some(matter) = matter {
                (markdown, crate::markdown::matter::state::State::HTML(matter))
            } else {
                (input, crate::markdown::matter::state::State::None)
            }
        },
        Some('{') => {
            let (markdown, matter) = crate::markdown::matter::kinds::json::extract(input);
            if let Some(Ok(matter)) = matter {
                (markdown, crate::markdown::matter::state::State::JSON(matter))
            } else {
                (input, crate::markdown::matter::state::State::None)
            }
        },
        Some('+') => {
            let (markdown, matter) = crate::markdown::matter::kinds::toml::extract(input);
            if let Some(Ok(matter)) = matter {
                (markdown, crate::markdown::matter::state::State::TOML(matter))
            } else {
                (input, crate::markdown::matter::state::State::None)
            }
        }
        Some('-') => {
            let (markdown, matter) = crate::markdown::matter::kinds::yaml::extract(input);
            if let Some(Ok(matter)) = matter {
                (markdown, crate::markdown::matter::state::State::YAML(matter))
            } else {
                (input, crate::markdown::matter::state::State::None)
            }
        },
        _ => (input, crate::markdown::matter::state::State::None),
    }
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
    fn test_html() {
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
        let (output_markdown, matter) = extract(input_markdown);
        assert_eq!(output_markdown, expect_markdown);
        if let crate::markdown::matter::state::State::HTML(matter) = matter {
            assert_eq!(matter["alpha"].as_str(), "bravo");
            assert_eq!(matter["charlie"].as_str(), "delta");
        } else {
            panic!("HTML(matter)")
        };
    }

    #[test]
    fn test_json() {
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
        let (output_markdown, matter) = extract(input_markdown);
        assert_eq!(output_markdown, expect_markdown);
        if let crate::markdown::matter::state::State::JSON(matter) = matter {
            assert_eq!(matter["alpha"].as_str().unwrap(), "bravo");
            assert_eq!(matter["charlie"].as_str().unwrap(), "delta");
        } else {
            panic!("HTML(matter)")
        };
    }

    #[test]
    fn test_toml() {
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
        let (output_markdown, matter) = extract(input_markdown);
        assert_eq!(output_markdown, expect_markdown);
        if let crate::markdown::matter::state::State::TOML(matter) = matter {
            assert_eq!(matter["alpha"].as_str().unwrap(), "bravo");
            assert_eq!(matter["charlie"].as_str().unwrap(), "delta");
        } else {
            panic!("HTML(matter)")
        };
    }

    #[test]
    fn test_yaml() {
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
        let (output_markdown, matter) = extract(input_markdown);
        assert_eq!(output_markdown, expect_markdown);
        if let crate::markdown::matter::state::State::YAML(matter) = matter {
            assert_eq!(matter["alpha"].as_str().unwrap(), "bravo");
            assert_eq!(matter["charlie"].as_str().unwrap(), "delta");
        } else {
            panic!("HTML(matter)")
        };
    }

}
