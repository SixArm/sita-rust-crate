use crate::types::{html::*, url::*};

/// Create a "html" tag with default settings.
///
/// Example:
///
/// ```
/// let x = html_tag_with_default();
/// assert_eq!(x, "<html>");
/// ```
///
#[allow(dead_code)]
fn html_tag_with_default() -> HtmlString {
    String::from("<html>")
}

/// Create a "html" tag with a language setting.
///
/// Example:
///
/// ```
/// let lang = "en";
/// let x = html_tag_with_lang(lang);
/// assert_eq!(x, "<html lang=\"en\">");
/// ```
///
#[allow(dead_code)]
fn html_tag_with_lang<S: AsRef<str> + Sized>(lang: S) -> HtmlString {
    format!("<html lang=\"{}\">", lang.as_ref())
}

/// Create a "link" tag with a rel and href URL.
///
/// ```
/// let rel = "stylesheet";
/// let href = "example.css";
/// let html = link_tagger_with_rel_and_href(rel, href);
/// assert_eq!(x, "<link rel=\"stylesheet\" href=\"example.css\">");
/// ```
///
#[allow(dead_code)]
fn link_tagger_with_rel_and_href<S: AsRef<str> + Sized>(rel: S, href: S) -> HtmlString {
    format!("<link rel=\"{}\" href=\"{}\">", rel.as_ref(), href.as_ref())
}

/// Create a "title" tag pair with a title setting.
///
/// Example:
///
/// ```
/// let title = "Welcome";
/// let x = title_tagger_with_title(title);
/// assert_eq!(x, "<title>Welcome</title>");
/// ```
///
#[allow(dead_code)]
fn title_tagger_with_title<S: AsRef<str> + Sized>(title: S) -> HtmlString {
    format!("<title>{}</title>", title.as_ref())
}

/// Create a "script" tag string with a URL.
///
/// ```
/// let url: &UrlStr = "my.js";
/// let x = script_tagger_with_src(&url);
/// assert_eq!(x, "<script src=\"my.js\"></script>");
/// ```
///
#[allow(dead_code)]
fn script_tagger_with_src(url: &UrlStr) -> HtmlString {
    format!("<script src=\"{}\"></script>", url)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_html_tag_with_default() {
        let actual = super::html_tag_with_default();
        let expect = "<html>";
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_html_tag_with_lang() {
        let lang = "en";
        let actual = super::html_tag_with_lang(&lang);
        let expect = "<html lang=\"en\">";
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_link_tagger_with_rel_and_href() {
        let rel = "alfa";
        let href: &UrlStr = "bravo";
        let actual = super::link_tagger_with_rel_and_href(&rel, &href);
        let expect = "<link rel=\"alfa\" href=\"bravo\">";
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_title_tagger_with_title() {
        let title = "alfa";
        let actual = super::title_tagger_with_title(&title);
        let expect = "<title>alfa</title>";
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_script_tagger_with_src() {
        let url: &UrlStr = "alfa";
        let actual = super::script_tagger_with_src(&url);
        let expect = "<script src=\"alfa\"></script>";
        assert_eq!(actual, expect);
    }

}
