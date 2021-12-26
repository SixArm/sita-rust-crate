//! Templater with Handlebars

use handlebars::Handlebars;
use serde::Serialize;
use std::path::PathBuf;
use crate::app::args::Args;
use crate::errors::*;
use crate::types::*;
use crate::state::state_enum::StateEnum;
use crate::templater::templater_trait::TemplaterTrait;

#[derive(Debug, Default)]
pub struct TemplaterWithHandlebars<'templater> {
    pub handlebars: ::handlebars::Handlebars<'templater>,
}

impl<'templater> TemplaterTrait for TemplaterWithHandlebars<'templater> {

    fn new() -> Self {
        trace!("templater_with_handlebars.rs new");
        let mut handlebars = Handlebars::default();
        TemplaterWithHandlebars {
            handlebars: handlebars,
        }
    }

    fn new_with_args(_args: &Args) -> Self {
        trace!("templater_with_handlebars.rs new_with_args");
        let mut handlebars = Handlebars::default();
        handlebars.set_strict_mode(true);
        TemplaterWithHandlebars {
            handlebars: handlebars,
        }
    }

    fn template_name_default(&self) -> String {
        trace!("templater_with_handlebars.rs template_name_default");
        String::from("default")
    }

    fn template_content_text_default(&self) -> String {
        trace!("templater_with_handlebars.rs template_content_text_default");
        String::from("{{ content }}")
    }

    fn add_template_via_name_and_content_text(&mut self, name: &str, content_text: &str) -> Result<()> {
        trace!("templater_with_handlebars.rs add_template_via_name_and_content_text");
        self.handlebars.register_template_string(&name, &content_text)
        .chain_err(|| "add_template_via_name_and_content_text")
    }

    fn add_template_via_name_and_content_file(&mut self, name: &str, content_file: &PathBuf) -> Result<()> {
        trace!("templater_with_handlebars.rs add_template_via_name_and_content_file");
        let content_text = ::std::fs::read_to_string(content_file)
        .chain_err(|| "add_template_via_name_and_content_file read_to_string")?;
        self.add_template_via_name_and_content_text(&name, &content_text)
    }

    fn contains_any_template(&self) -> bool {
        trace!("templater_with_handlebars.rs contains_any_template");
        !self.handlebars.get_templates().is_empty()
    }

    fn contains_template_name(&self, name: &str) -> bool {
        trace!("templater_with_handlebars.rs contains_template_name");
        self.handlebars.get_template(&name).is_some()
    }

    fn template_names_as_set_str(&self) -> Set<&str> {
        trace!("templater_with_handlebars.rs template_names_as_set_str");
        let mut names: Set<&str> = Set::new();
        for key in self.handlebars.get_templates().keys() {
            names.insert(key);
        }
        names
    }

    fn render_template_with_state_enum(&self, template_name: &str, state_enum: &StateEnum) -> Result<HtmlString> {
        trace!("templater_with_handlebars.rs render_template_with_state_enum");
        //TODO make generic
        match state_enum {
            StateEnum::StateWithMap(x) =>  self.handlebars.render(template_name, x),
            StateEnum::StateWithJSON(x) => self.handlebars.render(template_name, x),
            StateEnum::StateWithTOML(x) => self.handlebars.render(template_name, x),
            StateEnum::StateWithYAML(x) => self.handlebars.render(template_name, x),
        }.chain_err(|| "render_template_with_state_enum")
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use lazy_static::*;
    use crate::app::args::Args;
    use crate::matter::matter_parser_trait::MatterParserTrait;
    use crate::state::state_enum::StateEnum;
    use crate::state::state_with_json::StateWithJSON;
    use crate::state::state_with_map::StateWithMap;
    use crate::state::state_with_toml::StateWithTOML;
    use crate::state::state_with_yaml::StateWithYAML;

    lazy_static! {
        pub static ref TESTS_DIR: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests"].iter().collect::<PathBuf>();
    }
    
    const FAB_OUTPUT_HTML: &str = "my content";

    type TemplaterX<'templater> = TemplaterWithHandlebars<'templater>;

    #[test]
    fn test_new() {
        let templater = TemplaterX::new();
        assert!(!templater.contains_any_template());
    }

    #[test]
    fn test_new_with_args() {
        let args = Args::default();
        let templater = TemplaterX::new_with_args(&args);
        assert!(!templater.contains_any_template());
    }

    #[test]
    fn test_templater_name_default() {
        let templater = TemplaterX::new();
        assert_eq!(templater.template_name_default(), "default");
    }

    #[test]
    fn test_templater_content_text_default() {
        let templater = TemplaterX::new();
        assert_eq!(templater.template_content_text_default(), "{{ content }}");
    }

    #[test]
    fn test_add_template_via_name_and_content_text() {
        let mut templater = TemplaterX::new();
        let name = "alpha";
        let content_text = "{{ bravo }}";
        assert_eq!(templater.contains_template_name("alpha"), false);
        assert_eq!(templater.contains_template_name("charlie"), false);
        templater.add_template_via_name_and_content_text(&name, &content_text);
        assert_eq!(templater.contains_template_name("alpha"), true);
        assert_eq!(templater.contains_template_name("charlie"), false);
    }

    #[test]
    fn test_add_template_via_name_and_content_file() {
        let mut templater = TemplaterX::new();
        let name = "alpha";
        let content_file = TESTS_DIR.join("function").join("add_template_via_name_and_content_file").join("template.html");
        assert!(content_file.exists());
        assert_eq!(templater.contains_template_name("alpha"), false);
        assert_eq!(templater.contains_template_name("charlie"), false);
        templater.add_template_via_name_and_content_file(&name, &content_file).expect("add_template_via_name_and_content_file");
        assert_eq!(templater.contains_template_name("alpha"), true);
        assert_eq!(templater.contains_template_name("charlie"), false);
    }
        
    #[test]
    fn test_contains_any_template() {
        let mut templater  = TemplaterX::new();
        assert_eq!(templater.contains_any_template(), false);
        templater.add_template_via_default();
        assert_eq!(templater.contains_any_template(), true);
    }

    #[test]
    fn test_contains_template_name() {
        let mut templater  = TemplaterX::new();
        assert_eq!(templater.contains_template_name("default"), false);
        templater.add_template_via_default();
        assert_eq!(templater.contains_template_name("default"), true);
    }

    #[test]
    fn test_template_names_as_set_string() {
        let mut templater = TemplaterX::new();
        let name_0: &str = "my-name-0";
        let name_1: &str = "my-name-1";
        templater.add_template_via_name_and_content_text(&name_0, "my text 0").expect("add_template_via_name_and_content_text");
        templater.add_template_via_name_and_content_text(&name_1, "my text 1").expect("add_template_via_name_and_content_text");
        let actual: Set<&str> = templater.template_names_as_set_str();
        let expect: Set<&str> = set!(name_0, name_1);
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_render_template_with_state_enum_x_map() {
        let mut templater = TemplaterX::new();
        templater.add_template_via_default().expect("default");
        let matter_text = indoc!{r#"
            <!--
                title: my title
                content: my content
            -->
        "#};
        let name = templater.template_name_default();
        let state  =  crate::matter::matter_parser_with_map::MatterParserWithMap{}.parse_matter_text_to_state(matter_text).expect("parse_matter_text_to_state");
        let state_enum = StateEnum::StateWithMap(state);
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
        let name = templater.template_name_default();
        let state: StateWithJSON = crate::matter::matter_parser_with_json::MatterParserWithJSON{}.parse_matter_text_to_state(matter_text).expect("parse_matter_text_to_state");
        let state_enum = StateEnum::StateWithJSON(state);
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
        let name = templater.template_name_default();
        let state: StateWithTOML = crate::matter::matter_parser_with_toml::MatterParserWithTOML{}.parse_matter_text_to_state(matter_text).expect("parse_matter_text_to_state");
        let state_enum = StateEnum::StateWithTOML(state);
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
        let name = templater.template_name_default();
        let state: StateWithYAML = crate::matter::matter_parser_with_yaml::MatterParserWithYAML{}.parse_matter_text_to_state(matter_text).expect("parse_matter_text_to_state");
        let state_enum = StateEnum::StateWithYAML(state);
        let actual = templater.render_template_with_state_enum(&name, &state_enum).expect("render_template_with_state");
        assert_eq!(actual, FAB_OUTPUT_HTML);
    }

}
