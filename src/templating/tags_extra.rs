/// Create a "html" tag with optional settings.
///
/// Example:
///
/// ```
/// let lang = "en";
/// let x = html_tag_with_with_option(Some(lang));
/// assert_eq!(x, "<html lang=\"en\">");
/// ```
///
#[allow(dead_code)]
fn html_tag_with_option(lang: Option<String>) -> HtmlString {
    match lang {
        Some(x) => html_tag_with_lang(&x),
        None => html_tag_with_default(),
    }
}

/// Create a "title" tag with optional settings.
///
/// ```
/// let title = "Welcome";
/// let x = title_tag_with_option(Some(title));
/// assert_eq!(x, "<title>Welcome</title>");
/// ```
///
#[allow(dead_code)]
fn title_tag_with_option(title: Option<String>) -> HtmlString {
    match title {
        Some(x) => title_tag_with_title(&x),
        None => title_tag_with_default(),
    }
}

/// Create "script" tags with optional settings.
///
/// TODO
///
#[allow(dead_code)]
fn script_tags_with_option(script_url_list: Option<Vec<UrlString>>) -> HtmlString {
    match script_url_list {
        Some(urls) => script_tags_with_urls(&urls),
        None => String::from(""),
    }
}

/// Create "script" tags with URLs.
///
/// TODO
///
#[allow(dead_code)]
fn script_tags_with_urls(urls: &Vec<UrlString>) -> HtmlString {
    urls.iter().map(|url| 
        script_tag_with_url(&url)
    ).collect::<String>()
}

/// Create stylesheet "link" tags with optional settings.
///
/// TODO
///
#[allow(dead_code)]
fn stylesheet_tags_with_option(stylesheet_url_list: Option<Vec<UrlString>>) -> HtmlString {
    match stylesheet_url_list {
        Some(urls) => stylesheet_tags_with_urls(&urls),
        None => String::from(""),
    }
}

/// Create stylesheet "link" tags with URLs.
///
/// TODO
///
#[allow(dead_code)]
fn stylesheet_tags_with_urls(urls: &Vec<UrlString>) -> HtmlString {
    urls.iter().map(|url| 
        stylesheet_tag_with_url(&url)
    ).collect::<String>()
}

#[test]
fn test_html_tag_with_option_x_some() {
    let lang = "en";
    let option = Some(lang);
    let actual = super::html_tag_with_option(option);
    let expect = super::html_tag_with_lang(&lang);
    assert_eq!(actual, expect);
}

#[test]
fn test_html_tag_with_option_x_none() {
    let option: Option<String> = None;
    let actual = super::html_tag_with_option(option);
    let expect = super::html_tag_with_default();
    assert_eq!(actual, expect);
}

#[test]
fn test_title_tag_with_option_x_some() {
    let title = "Welcome";
    let option = Some(title);
    let actual = super::title_tag_with_option(option);
    let expect = super::title_tag_with_title(&title;
    assert_eq!(actual, expect);
}

#[test]
fn test_title_tag_with_option_x_none() {
    let option: Option<String> = None;
    let actual = super::title_tag_with_option(option);
    let expect = super::title_tag_with_default();;
    assert_eq!(actual, expect);
}

#[test]
fn test_script_tags_with_option_x_some() {
    let url_0: UrlStr = "alpha";
    let url_1: UrlStr = "bravo";
    let urls: Vec<UrlString> = vec![
        url_0.clone(),
        url_1.clone(),
    ];
    let option = Some(urls.clone());
    let actual = super::script_tags_with_option(option);
    let expect = super::script_tags_with_urls(&urls);
    assert_eq!(actual, expect);
}

#[test]
fn test_script_tags_with_option_x_none() {
    let option = None;
    let actual = super::script_tags_with_option(option);
    let expect = "";
    assert_eq!(actual, expect);
}

#[test]
fn test_script_tags_with_urls() {
    let url_0: UrlStr = "alpha";
    let url_1: UrlStr = "bravo";
    let urls = vec![
        url_0.clone(),
        url_1.clone(),
    ];
    let actual = super::script_tags_with_urls(&urls);
    let expect = format!("{}{}", 
        script_tag_with_url(&url_0),
        script_tag_with_url(&url_1)
    );
    assert_eq!(actual, expect);
}

#[test]
fn test_stylesheet_tags_with_option_x_some() {
    let url_0: UrlStr = "alpha";
    let url_1: UrlStr = "bravo";
    let urls = vec![
        url_0.clone(),
        url_1.clone(),
    ];
    let option = Some(urls.clone());
    let actual = super::stylesheet_tags_with_option(option);
    let expect = super::stylesheet_tags_with_urls(&urls);
    assert_eq!(actual, expect);
}

#[test]
fn test_stylesheet_tags_with_option_x_none() {
    let option = None;
    let actual = super::stylesheet_tags_with_option(option);
    let expect = "";
    assert_eq!(actual, expect);
}

#[test]
fn test_stylesheet_tags_with_urls() {
    let url_0: UrlStr = "alpha";
    let url_1: UrlStr = "bravo";
    let urls = vec![
        url_0.clone(),
        url_1.clone(),
    ];
    let actual = super::stylesheet_tags_with_urls(&urls);
    let expect = format!("{}{}", 
        stylesheet_tag_with_url(&url_0),
        stylesheet_tag_with_url(&url_1),
    );
    assert_eq!(actual, expect);
}

