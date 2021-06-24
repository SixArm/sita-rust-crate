//! Markdown parser.

use pulldown_cmark::{Options, Parser};

pub fn parser(text: &str) -> Parser  {
    Parser::new_ext(text, Options::all())
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_parser() {
        let markdown = "# alpha\n\nbravo *charlie* ~~delta~~ foxtrot\n";
        let parser = super::parser(markdown);
        let expect =  "<h1>alpha</h1>\n<p>bravo <em>charlie</em> <del>delta</del> foxtrot</p>\n";
        let mut actual = String::new();
        pulldown_cmark::html::push_html(&mut actual, parser);
        assert_eq!(actual, expect);
    }

}
