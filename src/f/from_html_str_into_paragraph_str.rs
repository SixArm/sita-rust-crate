use once_cell::sync::Lazy;
use regex::Regex;
use crate::types::html::*;

pub static REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)(?m)(?s)<p( .*?)?>(?P<paragraph>.*?)\s*</p>").unwrap()
});

/// Convert from HtmlStr into a paragraph str.
///
/// This finds the first <p> tag and returns the inner text.
///
/// A typical use case is scanning a web page to find a viable description.
///
/// Example:
//
/// ```rust
/// let from: "<h1>Alpha</h1><p>Bravo</p><p>Charlie</p>";
/// let to: &str = from_html_str_into_paragraph_str(&from);
/// //=> "Bravo"
/// ```
///
#[allow(dead_code)]
pub fn from_html_str_into_paragraph_str(html_str: &HtmlStr) -> Option<&str> {
    if let Some(captures) = (*REGEX).captures(html_str) {
        if let Some(x) = captures.name("paragraph") {
            return Some(x.as_str())
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::indoc::indoc;

    #[test]
    fn test_from_html_str_into_paragraph_str_x_present_x_h1_lowercase_and_plain() {
        let html_str = indoc!{r#"
            lorem ipsum
            <h1>alfa bravo</h1>
            <p>charlie delta</p>
            <p>echo foxtrot</p>
            lorem ipsum
         "#};
        let option = from_html_str_into_paragraph_str(&html_str);
        assert!(option.is_some());
        let headline_str = option.unwrap();
        assert_eq!(headline_str, "charlie delta");
    }

    #[test]
    fn test_from_html_str_into_paragraph_str_x_present_x_tag_has_uppercase_and_extra() {
        let html_str = indoc!{r#"
            lorem ipsum
            <h1>alfa bravo</h1>
            <P class="foo">charlie delta</P>
            <p>echo foxtrot</p>
            lorem ipsum
         "#};
        let option = from_html_str_into_paragraph_str(&html_str);
        assert!(option.is_some());
        let headline_str = option.unwrap();
        assert_eq!(headline_str, "charlie delta");
    }

    #[test]
    fn test_from_html_str_into_paragraph_str_x_absent() {
        let html_str = indoc!{r#"
            lorem ipsum
            alfa bravo
            charlie delta
            echo foxtrot
            lorem ipsum
        "#};
        let option = from_html_str_into_paragraph_str(&html_str);
        assert!(option.is_none());
    }

}
