//! Templater with Tera

use std::path::PathBuf;
use serde::Serialize;
use crate::app::args::Args;
use crate::errors::*;
use crate::types::*;
use crate::matter::matter_parser::MatterParser;
use crate::state::state::State;
use crate::state::state_enum::StateEnum;
use crate::templating::templater::Templater;

pub struct TemplaterWithTera {
    pub tera: ::tera::Tera,
}

impl Templater for TemplaterWithTera {

    // Create a new templater.
    //
    // Example:
    //
    // ```
    // let mut templater = TemplaterWithTera::new();
    // ```
    //
    fn new() -> Self {
        trace!("templater_with_tera::new");
        let my_tera = ::tera::Tera::default();
        TemplaterWithTera {
            tera: my_tera,
        }
    }

    // Create a new templater with args.
    //
    // Example:
    //
    // ```
    // let args = Args::default();
    // let mut templater = TemplaterWithTera::new_with_args(&args);
    // ```
    //
    fn new_with_args(_args: &Args) -> Self {
        trace!("templater_with_tera::new_with_args");
        let mut my_tera = ::tera::Tera::default();
        my_tera.autoescape_on(vec![]); // disable autoescaping completely
        TemplaterWithTera {
            tera: my_tera,
        }        
    }

    // Add a template via name and text.
    //
    // Example:
    //
    // ```
    // let mut templater = TemplaterWithTera::new();
    // let name = "alpha";
    // let text = "<p>{{ bravo }}</p>";
    // add_template_via_name_and_text(&name, &text);
    // ```
    //
    fn add_template_via_name_and_text(&mut self, name: &str, text: &str) -> Result<()> {
        trace!("templater_with_tera::add_template_via_name_and_file: name={} text=…", &name);
        self.tera.add_raw_template(&name, &text)
        .chain_err(|| "add_template_via_name_and_text")
    }

    // Add a template via name and file.
    //
    // Example:
    //
    // ```
    // let mut templater = TemplaterWithTera::new();
    // let name = "alpha";
    // let file = PathBuf::from("template.html")
    // add_template_via_name_and_file(&name, &file);
    // ```
    //
    fn add_template_via_name_and_file(&mut self, name: &str, file: &PathBuf) -> Result<()> {
        trace!("templater_with_tera::add_template_via_name_and_file: name={} file: {:?}", &name, &file);
        self.tera.add_template_file(&file, Some(&name))
        .chain_err(|| "add_template_via_name_and_file")
    }

    // Does the templater have any templates?
    //
    // Example:
    //
    // ```
    // let mut templater = TemplaterWithTera::new();
    // let flag = tera_has_template(templater);
    // assert_eq!(flag, false);
    // ```
    //
    // ```
    // let mut templater = TemplaterWithTera::new();
    // templater.add_template_via_name_and_text("my-template", "{{ my-content }}");
    // let flag = has_template(templater);
    // assert_eq!(flag, true);
    // ```
    //
    fn has_template(&self) -> bool {
        trace!("templater_with_tera::has_template");
        self.tera.get_template_names().nth(0).is_some()
    }

    // Get the template names.
    //
    // Example:
    //
    // ```
    // let mut templater = TemplaterWithTera::new();
    // add_template_via_name_and_text("alpha", "alpha text".into());
    // add_template_via_name_and_text("bravo", "bravo text".into());
    // let template_names: Set<&str> = templater.template_names_as_set_str();
    // assert_eq!(template_names, set!["alpha", "bravo"]);
    // ```
    //
    fn template_names_as_set_str(&self) -> Set<&str> {
        trace!("templater_with_tera::template_names_as_set_str");
        self.tera.get_template_names().collect::<_>()
    }

    // Render a template name with the given variables as JSON.
    //
    // ```
    // let templater: Templater = TemplaterWithTera::new();
    // templater.add_default_template();
    // let name = template_default_name();
    // let vars: ::serde_json::Value = ::serde_json::from_str(indoc!{r#"{"content": "alpha"}"#}).unwrap();
    // let html = templater.render(&name, &vars);
    // assert_eq!(html, "alpha");
    // ```
    //
    fn render_template_with_state_enum(&self, template_name: &str, state_enum: &StateEnum) -> Result<HtmlString> {
        trace!("templater_with_tera::render_template_with_state_enum");
        let context = from_state_enum_to_tera_context(&state_enum)
        .chain_err(|| "create tera context")?;
        debug!("context: {:?}", &context);
        let html = self.tera.render(&template_name, &context)
        .chain_err(|| "render template with tera context")?;
        Ok(html)
    }

}

pub fn from_state_enum_to_tera_context(state_enum: &crate::state::state_enum::StateEnum) -> ::tera::Result<::tera::Context> {
    match state_enum {
        crate::state::state_enum::StateEnum::StateWithHTML(x) => ::tera::Context::from_serialize(x),
        crate::state::state_enum::StateEnum::StateWithJSON(x) => ::tera::Context::from_serialize(x),
        crate::state::state_enum::StateEnum::StateWithTOML(x) => ::tera::Context::from_serialize(x),
        crate::state::state_enum::StateEnum::StateWithYAML(x) => ::tera::Context::from_serialize(x),
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

    type TemplaterX = TemplaterWithTera;

    #[test]
    fn test_default() {
        let _templater = TemplaterX::new();
        //TODO
    }

    #[test]
    fn test_default_with_args() {
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
        templater.add_template_via_name_and_text("my-name", "my-content").unwrap();
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
    fn test_template_names_as_set_str() {
        let mut templater = TemplaterX::new();
        let name_0: &str = "my-name-0";
        let name_1: &str = "my-name-1";
        templater.add_template_via_name_and_text(&name_0, "my text 0").expect("add_template_via_name_and_text");
        templater.add_template_via_name_and_text(&name_1, "my text 1").expect("add_template_via_name_and_text");
        let actual: Set<&str> = templater.template_names_as_set_str();
        let expect: Set<&str> = set!(name_0, name_1);
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_render_template_with_state_enum_x_html() {
        let mut templater = TemplaterX::new();
        templater.add_template_via_default().expect("default");
        let matter_text = indoc!{r#"
            <!--
                title: my title
                content: my content
            -->
        "#};
        let name = templater.template_default_name();
        let state = crate::matter::matter_parser_with_html::MatterParserWithHTML{}.parse_matter_text_to_state(matter_text).expect("parse_matter_text_to_state");
        let state_enum = state.to_state_enum();
        let actual = templater.render_template_with_state_enum(&name, &state_enum).expect("render_template_with_state");
        assert_eq!(actual, FAB_OUTPUT_HTML);
    }

    #[test]
    fn test_render_template_with_state_enum_x_json() {
        let mut templater = TemplaterX::new();
        templater.add_template_via_default().expect("default");
        let matter_text = indoc!{r#"
            {
                "title": "my title",
                "content": "my content"
            }
        "#};
        let name = templater.template_default_name();
        let state = crate::matter::matter_parser_with_json::MatterParserWithJSON{}.parse_matter_text_to_state(matter_text).expect("parse_matter_text_to_state");
        let state_enum = state.to_state_enum();
        let actual = templater.render_template_with_state_enum(&name, &state_enum).expect("render_template_with_state");
        assert_eq!(actual, FAB_OUTPUT_HTML);
    }

    #[test]
    fn test_render_template_with_state_enum_x_toml() {
        let mut templater = TemplaterX::new();
        templater.add_template_via_default().expect("default");
        let matter_text = indoc!{r#"
            title = "my title"
            content = "my content"
        "#};
        let name = templater.template_default_name();
        let state = crate::matter::matter_parser_with_toml::MatterParserWithTOML{}.parse_matter_text_to_state(matter_text).expect("parse_matter_text_to_state");
        let state_enum = state.to_state_enum();
        let actual = templater.render_template_with_state_enum(&name, &state_enum).expect("render_template_with_state");
        assert_eq!(actual, FAB_OUTPUT_HTML);
    }

    #[test]
    fn test_render_template_with_state_enum_x_yaml() {
        let mut templater = TemplaterX::new();
        templater.add_template_via_default().expect("default");
        let matter_text = indoc!{r#"
            title: "my title"
            content: "my content"
        "#};
        let name = templater.template_default_name();
        let state = crate::matter::matter_parser_with_yaml::MatterParserWithYAML{}.parse_matter_text_to_state(matter_text).expect("parse_matter_text_to_state");
        let state_enum = state.to_state_enum();
        let actual = templater.render_template_with_state_enum(&name, &state_enum).expect("render_template_with_state");
        assert_eq!(actual, FAB_OUTPUT_HTML);
    }

}
