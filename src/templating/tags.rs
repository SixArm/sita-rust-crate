use crate::types;

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
fn html_tag_with_lang(lang: &str) -> HtmlString {
    format!("<html lang=\"{}\">", lang)
}

/// Create a "title" tag with default settings, 
/// which is always blank.
///
/// Example:
///
/// ```
/// let x = title_tag_with_default();
/// assert_eq!(x, "");
/// ```
///
#[allow(dead_code)]
fn title_tag_with_default() -> HtmlString {
    String::from("<title></title>")
}

/// Create a "title" tag with a title setting.
///
/// Example:
///
/// ```
/// let title = "Welcome";
/// let x = title_tag_with_title(title);
/// assert_eq!(x, "<title>Welcome</title>");
/// ```
///
#[allow(dead_code)]
fn title_tag_with_title(title: &str) -> HtmlString {
    format!("<title>{}</title>", title)
}

/// Create a "script" tag string with a URL.
///
/// ```
/// let url: &UrlStr = "my.js";
/// let x = script_tag_with_url(&url);
/// assert_eq!(x, "<script src=\"my.js\"></script>");
/// ```
///
#[allow(dead_code)]
fn script_tag_with_url(url: &UrlStr) -> HtmlString {
    format!("<script src=\"{}\"></script>", url)
}

/// Create a stylesheet "link" tag with a URL.
///
/// ```
/// let url: &UrlStr = "my.css";
/// let x = stylesheet_tag_with_url(url);
/// assert_eq!(x, "<link rel=\"stylesheet\" href=\"my.css\">");
/// ```
///
#[allow(dead_code)]
fn stylesheet_tag_with_url(url: &UrlStr) -> HtmlString {
    format!("<link rel=\"stylesheet\" href=\"{}\">", url)
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
    fn test_title_tag_with_default() {
        let actual = super::title_tag_with_default();
        let expect = "<title></title>";
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_title_tag_with_title() {
        let title = "Welcome";
        let actual = super::title_tag_with_title(&title);
        let expect = "<title>Welcome</title>";
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_script_tag_with_url() {
        let url: &UrlStr = "alpha";
        let actual = super::script_tag_with_url(&url);
        let expect = "<script src=\"alpha\"></script>";
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_stylesheet_tag_with_url() {
        let url: &UrlStr = "alpha";
        let actual = super::stylesheet_tag_with_url(&url);
        let expect = "<link rel=\"stylesheet\" href=\"alpha\">";
        assert_eq!(actual, expect);
    }

}
