use ::lol_html::{element, rewrite_str, RewriteStrSettings}; // , HtmlRewriter, Settings
use lol_html::html_content::Element;

pub fn rewrite_href_from_http_to_https(e: &mut Element) {
    if let Some(mut x) = e.get_attribute("href") {
        if x.starts_with("http:") {
            x.replace_range(0..5, "https:");
            e.set_attribute("href", &x).expect("set_attribute");
        }
    }
}

pub fn rewrite_href_from_md_to_html(e: &mut Element) {
    if let Some(mut x) = e.get_attribute("href") {
        if x.ends_with(".md") {
            x.replace_range((x.len()-3).., ".html");
            e.set_attribute("href", &x).expect("set_attribute");
        }
    }
}

pub fn rewrite(s: &str) -> String {
    let element_content_handlers = vec![
        element!("a[href]", |e| {
            crate::rewriting::lol::rewrite_href_from_http_to_https(e);
            crate::rewriting::lol::rewrite_href_from_md_to_html(e);
             Ok(())
        })
    ];
    rewrite_str(s,
        RewriteStrSettings {
            element_content_handlers,
            ..RewriteStrSettings::default()
        }
    ).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rewrite_from_md_to_html_x_success() {
        let input = r#"<a href="foo.md">"#;
        let expect =  r#"<a href="foo.html">"#;
        let actual = rewrite(input);
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_rewrite_from_md_to_html_x_skip_because_not_ends_with() {
        let input = r#"<a href="foo.md/goo">"#;
        let actual = rewrite(input);
        assert_eq!(actual, input);
    }

    #[test]
    fn test_rewrite_from_http_to_https_x_success() {
        let input = r#"<a href="http://example.com">"#;
        let expect =  r#"<a href="https://example.com">"#;
        let actual = rewrite(input);
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_rewrite_from_http_to_https_x_skip_because_not_starts_with() {
        let input = r#"<a href="foo/http:/goo">"#;
        let actual = rewrite(input);
        assert_eq!(actual, input);
    }

}