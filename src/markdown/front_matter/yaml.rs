use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REGEX: Regex
    = Regex::new(r"(?m)(?s)\A---\n(?P<front>.*?)\n---\n(?P<markdown>.*)\z").unwrap();
}

//TODO warn dead code
#[allow(dead_code)]
fn extract(input: &str) -> (&str, Option<Result<::yaml_rust::yaml::Yaml, ::yaml_rust::ScanError>>) {
    if let Some(captures) = REGEX.captures(input) {
        if let Some(front) = captures.name("front") {
            if let Some(markdown) = captures.name("markdown") {
                match ::yaml_rust::YamlLoader::load_from_str(front.as_str()) {
                    Ok(vec) => {
                        return (
                            markdown.as_str(),
                            Some(Ok(vec[0].to_owned())),
                        )
                    },
                    Err(e) => {
                        return (
                            markdown.as_str(),
                            Some(Err(e)),
                        )
                    }
                }
            }
        }
    }
    (input, None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::indoc::indoc;
    //use crate::vars::Vars;

    #[test]
    fn test_present() {
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
        let (output_markdown, front_option) = extract(input_markdown);
        assert_eq!(output_markdown, expect_markdown);
        assert!(front_option.is_some());
        let front_result = front_option.unwrap();
        assert!(front_result.is_ok());
        let front: ::yaml_rust::yaml::Yaml = front_result.unwrap();
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
