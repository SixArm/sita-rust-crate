// Extract from markdown input to markdown output and matter variables.
//
// Example:
//
// ```
// let input_markdown = indoc!{r#"
// <!--
//   alpha: bravo
//   charlie: delta
// -->
// echo
// foxtrot
// "#};
// let (output_markdown: &str, matter: Option<Matter>) = extract(input_markdown);
// ```
//
#[allow(dead_code)]
pub fn extract(input: &str) -> (&str, Matter) {
    match input.chars().next() {
        Some('<') => {
            let (markdown, matter) = crate::markdown::matter::kinds::html::extract(input);
            if let Some(matter) = matter {
                (markdown, Matter::HTML(front))
            } else {
                (input, crate::markdown::matter::state::State::None)
            }
        },
        Some('{') => {
            let (markdown, matter) = crate::markdown::matter::kinds::json::extract(input);
            if let Some(Ok(matter)) = matter {
                (markdown, Matter::JSON(front))
            } else {
                (input, crate::markdown::matter::state::State::None)
            }
        },
        Some('+') => {
            let (markdown, matter) = crate::markdown::matter::kinds::toml::extract(input);
            if let Some(Ok(matter)) = matter {
                (markdown, Matter::TOML(front))
            } else {
                (input, crate::markdown::matter::state::State::None)
            }
        }
        Some('-') => {
            let (markdown, matter) = crate::markdown::matter::kinds::yaml::extract(input);
            if let Some(Ok(matter)) = matter {
                (markdown, Matter::YAML(front))
            } else {
                (input, crate::markdown::matter::state::State::None)
            }
        },
        _ => (input, crate::markdown::matter::state::State::None),
    }
}

#[cfg(test)]
mod tests {
    use assertables;
    use indoc::indoc;
    use super::*;

    fn assert_discriminant_eq<T>(a: &T, b: &T) {
        assert_eq!(
            ::std::mem::discriminant(a), 
            ::std::mem::discriminant(b),
        ); 
    }

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
        assert_discriminant_eq(matter, Matter::HTML);
        // assert_eq!(map["alpha"], "bravo");
        // assert_eq!(map["charlie"], "delta");
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
        assert_discriminant_eq(&matter, &Matter::JSON);
        // assert_eq!(matter["alpha"].as_str().unwrap(), "bravo");
        // assert_eq!(matter["charlie"].as_str().unwrap(), "delta");
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
        assert_discriminant_eq(&matter, &Matter::TOML);
        // assert_eq!(matter["alpha"].as_str().unwrap(), "bravo");
        // assert_eq!(matter["charlie"].as_str().unwrap(), "delta");
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
        assert_discriminant_eq(&matter, &Matter::YAML);
        // assert_eq!(matter["alpha"].as_str().unwrap(), "bravo");
        // assert_eq!(matter["charlie"].as_str().unwrap(), "delta");
    }

}
