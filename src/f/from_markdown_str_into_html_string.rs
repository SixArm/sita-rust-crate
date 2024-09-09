use pulldown_cmark;

/// From Markdown str into HTML String.
/// 
/// This implementation uses the pulldown_cmark tool.
/// 
/// This implementation creates a pulldown parser with our preferred options,
/// which are defined in `crate::markdown::markdown_parser::parser`.`
///
pub fn from_markdown_str_into_html_string(markdown_str: &str) -> String {
    let parser = crate::markdown::markdown_parser::parser(markdown_str);
    let mut html_string = String::new();
    pulldown_cmark::html::push_html(&mut html_string, parser);
    html_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let markdown_str = "# alfa\nbravo\n";
        let html_string = from_markdown_str_into_html_string(markdown_str);
        assert_eq!("<h1>alfa</h1>\n<p>bravo</p>\n", html_string);
    }

}
