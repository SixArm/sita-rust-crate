use once_cell::sync::Lazy;
use regex::Regex;
use crate::types::*;

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
    use ::indoc::indoc;

    #[test]
    fn test_from_html_str_into_headline_str_x_present_x_h1_lowercase_and_plain() {
        let html_str = indoc!{r#"
            lorem ipsum
            <h1>alpha bravo</h1>
            lorem ipsum
         "#};
        let option = from_html_str_into_headline_str(&html_str);
        assert!(option.is_some());
        let headline_str = option.unwrap();
        assert_eq!(headline_str, "alpha bravo");
    }

    #[test]
    fn test_from_html_str_into_headline_str_x_present_x_h1_uppercase_and_extra() {
        let html_str = indoc!{r#"
            lorem ipsum
            <H1 class="foo">alpha bravo</H1>
            lorem ipsum
         "#};
        let option = from_html_str_into_headline_str(&html_str);
        assert!(option.is_some());
        let headline_str = option.unwrap();
        assert_eq!(headline_str, "alpha bravo");
    }

    #[test]
    fn test_from_html_str_into_headline_str_x_absent() {
        let html_str = indoc!{r#"
            lorem ipsum
            alpha bravo
            lorem ipsum
        "#};
        let option = from_html_str_into_headline_str(&html_str);
        assert!(option.is_none());
    }

}
