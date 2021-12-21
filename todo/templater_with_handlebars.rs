//! Templater with Handlebars

use handlebars::Handlebars;
use indoc::indoc;
use std::path::PathBuf;
use crate::app::args::Args;
use crate::errors::*;
use crate::types::*;
use crate::matter::matter_parser::MatterParser;
use crate::templating::templater::Templater;

pub struct TemplaterWithHandlebars<'a> {
    //pub type Templater<'a> = handlebars::Handlebars<'a>;
    pub handlebars<'a>: ::handlebars::Handlebars<'a>,
}

impl Templater for TemplaterWithHandlebars {

    // Create a default templater.
    //
    // Example:
    //
    // ```
    // let mut templater = TemplaterWithHandlebars::new();
    // ```
    //
    fn default() -> Self {
        let my_handlebars = Handlebars::default();
        TemplaterWithHandlebars {
            handlebars: my_handlebars,
        }
    }

    // Create a default templater with args.
    //
    // Example:
    //
    // ```
    // let args = Args::default();
    // let mut templater = TemplaterWithHandlebars::new_with_args(&args);
    // ```
    //
    fn default_with_args(_args: &Args) -> Self {
        let my_handlebars = Handlebars::default();
        my_handlebars.set_strict_mode(true);
        TemplaterWithHandlebars {
            handlebars: my_handlebars,
        }
    }

    // Add a template via name and text.
    //
    // Example:
    //
    // ```
    // let mut templater = TemplaterWithHandlebars::new();
    // let name = "alpha";
    // let text = "<p>{{ bravo }}</p>";
    // add_template_via_name_and_text(&name, &text);
    // ```
    //
    fn add_template_via_name_and_text(&mut self, name: &str, text: &str) -> Result<()> {
        self.handlebars.register_template_string(&name, &text)
        .chain_err(|| "add_template_via_name_and_text")
    }

    // Add a template via name and file.
    //
    // Example:
    //
    // ```
    // let mut templater = TemplaterWithHandlebars::new();
    // let name = "alpha";
    // let file = PathBuf::from("template.html")
    // add_template_via_name_and_file(&name, &file);
    // ```
    //
    fn add_template_via_name_and_file(&mut self, name: &str, file: &PathBuf) -> Result<()> {
        let text = ::std::fs::read_to_string(file)
        .chain_err(|| "add_template_via_name_and_file read_to_string")?;
        self.add_template_via_name_and_text(&name, &s)
    }

    // Does the templater have any templates?
    //
    // Example:
    //
    // ```
    // let mut templater = TemplaterWithHandlebars::new();
    // let flag = has_template(templater);
    // assert_eq!(flag, false);
    // ```
    //
    // ```
    // let mut templater = TemplaterWithHandlebars::new();
    // templater.add_template_via_name_and_text("my-template", "{{ my-content }}");
    // let flag = has_template(templater);
    // assert_eq!(flag, true);
    // ```
    //
    fn has_template(&self) -> bool {
        !self.handlebars.get_templates().is_empty()
    }

    // Get the template names.
    //
    // Example:
    //
    // ```
    // let mut templater = TemplaterWithHandlebars::new();
    // add_template_via_name_and_text("alpha".into(), "alpha text".into());
    // add_template_via_name_and_text("bravo".into(), "bravo text".into());
    // let template_names: Set<&String> = templater.template_names_as_set_string(&templater);
    // assert_eq!(template_names, set![&"alpha".into(), &"bravo".into()]);
    // ```
    //
    fn template_names_as_set_string<'a>(&self) -> Set<&'a String> {
        self.handlebars.get_templates().keys().collect::<Set<&'a String>>()
    }

        // Render a template name with the given variables as JSON.
    //
    // ```
    // let templater: Templater = TemplaterWithHandlebars::new();
    // templater.add_default_template();
    // let name = template_default_name();
    // let vars: ::serde_json::Value = ::serde_json::from_str(indoc!{r#"{"content": "alpha"}"#}).unwrap();
    // let html = templater.render(&name, &vars);
    // assert_eq!(html, "alpha");
    // ```
    //
    fn render_template_with_vars_as_json<S: AsRef<str> + Sized>(&self, template_name: S, vars: &::serde_json::Value) -> Result<HtmlString> {
        Ok(String::from("TODO"))
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use lazy_static::*;
    use crate::app::args::Args;

    lazy_static! {
        pub static ref TESTS_DIR: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests"].iter().collect::<PathBuf>();
    }
    
    const FAB_OUTPUT_HTML: &str = "my content";

    type TemplaterX: TemplaterWithHandlebars;

    #[test]
    fn test_default() {
        let _templater = TemplaterX::new();
        //TODO
    }

    #[test]
    fn test_new_with_args() {
        let args = Args::default();
        let _templater = TemplaterX::new_with_args(&args);
        //TODO
    }

    #[test]
    fn test_add_template_via_name_and_text() {
        let mut templater = TemplaterX::new();
        let name = "alpha";
        let text = "{{ bravo }}";
        templater.add_template_via_name_and_text(&name, &text);
        assert!(templater.has_template());
    }

    #[test]
    fn test_add_template_via_name_and_file() {
        let mut templater = TemplaterX::new();
        let name = "alpha";
        let file = TESTS_DIR.join("function").join("add_template_via_name_and_file").join("template.html");
        templater.add_template_via_name_and_file(&name, &file);
        assert!(templater.has_template());
    }
        
    #[test]
    fn test_has_template_x_true() {
        let mut templater  = TemplaterX::new();
        templater.register_template_string("my-name", "my-content").unwrap();
        let flag = templater.has_template();
        assert_eq!(flag, true);
    }

    #[test]
    fn test_has_template_x_false() {
        let templater = TemplaterX::new();
        let flag = templater.has_template();
        assert_eq!(flag, false);
    }

    #[test]
    fn test_template_names_as_set_string() {
        let mut templater = TemplaterX::new();
        let name_0: String = "my-name-0".into();
        let name_1: String = "my-name-1".into();
        templater.add_template_via_name_and_text(&name_0, "my text 0").expect("add_template_via_name_and_text");
        templater.add_template_via_name_and_text(&name_1, "my text 1").expect("add_template_via_name_and_text");
        let actual: Set<&str> = templater.template_names_as_set_str();
        let expect: Set<&String> = set!(&name_0, &name_1);
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_render_template_with_vars_x_html() {
        let mut templater = TemplaterX::new();
        templater.add_template_default().expect("default");
        let matter = indoc!{r#"
            <!--
                title: my title
                content: my content
            -->
        "#};
        let _name = templater.template_default_name();
        let _vars =  crate::matter::matter_parser_with_html::MatterParserWithHTML::parse_to_matter_state(&matter);
        // let result = templater.render_template_with_vars(&name, &vars);
        // assert!(result.is_ok());
        // let actual = result.unwrap();
        // assert_eq!(actual, FAB_OUTPUT_HTML);
    }

    #[test]
    fn test_render_template_with_vars_x_json() {
        let mut templater = TemplaterX::new();
        templater.add_template_default().expect("default");
        let matter = indoc!{r#"
            {
                "title": "my title",
                "content": "my content"
            }
        "#};
        let name = templater.template_default_name();
        let vars = crate::matter::matter_parser_with_json::MatterParserWithJSON::parse_to_matter_state(&matter);
        let result = templater.render_template_with_vars(&name, &vars);
        assert!(result.is_ok());
        let actual = result.unwrap();
        assert_eq!(actual, FAB_OUTPUT_HTML);
    }

    #[test]
    fn test_render_template_with_vars_x_toml() {
        let mut templater = TemplaterX::new();
        templater.add_template_default().expect("default");
        let matter = indoc!{r#"
            title = "my title"
            content = "my content"
        "#};
        let name = templater.template_default_name();
        let vars = crate::matter::matter_parser_with_toml::MatterParserWithTOML::parse_to_matter_state(&matter);
        let result = templater.render_template_with_vars(&name, &vars);
        assert!(result.is_ok());
        let actual = result.unwrap();
        assert_eq!(actual, FAB_OUTPUT_HTML);
    }

    #[test]
    fn test_render_template_with_vars_x_yaml() {
        let mut templater = TemplaterX::new();
        templater.add_template_default().expect("default");
        let matter = indoc!{r#"
            title: "my title"
            content: "my content"
        "#};
        let name = templater.template_default_name();
        let vars = crate::matter::matter_parser_with_yaml::MatterParserWithYAML::parse_to_matter_state(&matter);
        let result = templater.render_template_with_vars(&name, &vars);
        assert!(result.is_ok());
        let actual = result.unwrap();
        assert_eq!(actual, FAB_OUTPUT_HTML);
    }

}
