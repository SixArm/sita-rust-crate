//! Markdown parser.

use pulldown_cmark::{Options, Parser};

pub fn parser(text: &str) -> Parser  {
    Parser::new_ext(text, options())
}

/// Create pulldown cmark options.
/// 
/// This implementation adds all options;
/// this should be equivalent to Options::all()
/// 
/// TODO: make the options choosable via CLAP
/// 
fn options() -> Options {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_TASKLISTS);
    options
}

#[cfg(test)]
mod tests {
    use ::indoc::indoc;

    #[test]
    fn test_parser() {
        let markdown = indoc!{r#"
            # alpha
            bravo *charlie* delta
        "#};
        let parser = super::parser(markdown);
        let expect =  indoc!{r#"
            <h1>alpha</h1>
            <p>bravo <em>charlie</em> delta</p>
        "#};
        let mut actual = String::new();
        pulldown_cmark::html::push_html(&mut actual, parser);
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_option_footnotes() {
        let markdown = indoc!{r#"
            alpha[^1]

            [^1]: bravo
        "#};
        let parser = super::parser(markdown);
        let expect =  indoc!{r##"
            <p>alpha<sup class="footnote-reference"><a href="#1">1</a></sup></p>
            <div class="footnote-definition" id="1"><sup class="footnote-definition-label">1</sup>
            <p>bravo</p>
            </div>
        "##};
        let mut actual = String::new();
        pulldown_cmark::html::push_html(&mut actual, parser);
        assert_eq!(actual, expect);
    }

 
    #[test]
    fn test_option_heading_attributes() {
        let markdown = indoc!{r#"
            # alpha
        "#};
        let parser = super::parser(markdown);
        let expect = indoc!{r#"
            <h1>alpha</h1>
        "#};
        let mut actual = String::new();
        pulldown_cmark::html::push_html(&mut actual, parser);
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_option_smart_punctuation() {
        let markdown = indoc!{r#"
            "alpha"
            'bravo'
        "#};
        let parser = super::parser(markdown);
        let expect = indoc!{r#"
            <p>“alpha”
            ‘bravo’</p>
        "#};
        let mut actual = String::new();
        pulldown_cmark::html::push_html(&mut actual, parser);
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_option_strikethrough() {
        let markdown = indoc!{r#"
            ~~alpha~~
        "#};
        let parser = super::parser(markdown);
        let expect = indoc!{r#"
            <p><del>alpha</del></p>
        "#};
        let mut actual = String::new();
        pulldown_cmark::html::push_html(&mut actual, parser);
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_option_tables() {
        let markdown = indoc!{r#"
            | alpha | bravo |
            |-|-|
            | charlie | delta |
        "#};
        let parser = super::parser(markdown);
        let expect = indoc!{r#"
            <table><thead><tr><th>alpha</th><th>bravo</th></tr></thead><tbody>
            <tr><td>charlie</td><td>delta</td></tr>
            </tbody></table>
        "#};
        let mut actual = String::new();
        pulldown_cmark::html::push_html(&mut actual, parser);
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_option_tasklists() {
        let markdown = indoc!{r#"
            - [ ] alpha
            - [x] bravo
        "#};
        let parser = super::parser(markdown);
        let expect = indoc!{r#"
            <ul>
            <li><input disabled="" type="checkbox"/>
            alpha</li>
            <li><input disabled="" type="checkbox" checked=""/>
            bravo</li>
            </ul>
        "#};
        let mut actual = String::new();
        pulldown_cmark::html::push_html(&mut actual, parser);
        assert_eq!(actual, expect);
    }

}
