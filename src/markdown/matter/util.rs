//! Markdown matter using HTML, JSON, TOML, YAML.

use crate::errors::*;

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
    if let Some((markdown, matter)) = crate::markdown::matter::kinds::html::extract(input) {
        return (markdown, crate::markdown::matter::kinds::html::parse_to_state(matter));
    };
    if let Some((markdown, matter)) = crate::markdown::matter::kinds::json::extract(input) {
        return (markdown, crate::markdown::matter::kinds::json::parse_to_state(matter));
    };
    if let Some((markdown, matter)) = crate::markdown::matter::kinds::toml::extract(input) {
        return (markdown, crate::markdown::matter::kinds::toml::parse_to_state(matter));
    }
    if let Some((markdown, matter)) = crate::markdown::matter::kinds::yaml::extract(input) {
        return (markdown, crate::markdown::matter::kinds::yaml::parse_to_state(matter));
    }
    return (input, crate::markdown::matter::state::State::None)
}

pub fn from_state_to_tera_context(state: &crate::markdown::matter::state::State) -> Result<::tera::Context> {
    let mut context = match state {
        crate::markdown::matter::state::State::HTML(x) => {
            ::tera::Context::from_serialize(&x)
            .chain_err(|| "matter HTML to Tera context")?
        }
        crate::markdown::matter::state::State::JSON(x) =>  {
            ::tera::Context::from_serialize(&x)
            .chain_err(|| "matter JSON to Tera context")?
        }
        crate::markdown::matter::state::State::TOML(x) => {
            ::tera::Context::from_serialize(&x)
            .chain_err(|| "matter TOML to Tera context")?
        }            
        crate::markdown::matter::state::State::YAML(x) => {
            ::tera::Context::from_serialize(&x)
            .chain_err(|| "matter YAML to Tera context")?
        }, 
        crate::markdown::matter::state::State::None => {
            ::tera::Context::new()
        }
    };
    Ok(context)
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
