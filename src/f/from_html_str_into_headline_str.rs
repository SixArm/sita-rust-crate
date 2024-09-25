use once_cell::sync::Lazy;
use regex::Regex;
use crate::types::html::*;

pub static REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)(?m)(?s)<h1( .*?)?>(?P<headline>.*?)\s*</h1>").unwrap()
});

/// Convert from HtmlStr into a headline str.
///
/// This finds the first <h1> tag and returns the inner text.
///
/// A typical use case is scanning a web page to find a viable title.
///
/// Example:
//
/// ```rust
/// let from: "<h1>Alpha Bravo</h1>";
/// let to: &str = from_html_str_into_headline_str(&from);
/// //=> "Alpha Bravo"
/// ```
///
#[allow(dead_code)]
pub fn from_html_str_into_headline_str(html_str: &HtmlStr) -> Option<&str> {
    if let Some(captures) = (*REGEX).captures(html_str) {
        if let Some(x) = captures.name("headline") {
            return Some(x.as_str())
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use assertables::*;
    use ::indoc::indoc;

    #[test]
    fn test_from_html_str_into_headline_str_x_present_x_h1_lowercase_and_plain() {
        let html_str = indoc!{r#"
            lorem ipsum
            <h1>alfa bravo</h1>
            lorem ipsum
         "#};
        assert_some_eq!(from_html_str_into_headline_str(&html_str), Some("alfa bravo"));
    }

    #[test]
    fn test_from_html_str_into_headline_str_x_present_x_tag_has_uppercase_and_extra() {
        let html_str = indoc!{r#"
            lorem ipsum
            <H1 class="foo">alfa bravo</H1>
            lorem ipsum
         "#};
        assert_some_eq!(from_html_str_into_headline_str(&html_str), Some("alfa bravo"));
    }

    #[test]
    fn test_from_html_str_into_headline_str_x_absent() {
        let html_str = indoc!{r#"
            lorem ipsum
            alfa bravo
            lorem ipsum
        "#};
        assert_none!(from_html_str_into_headline_str(&html_str));
    }

}
