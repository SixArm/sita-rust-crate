//! Templater with Liquid

use std::path::PathBuf;
use serde::Serialize;
use crate::app::args::Args;
use crate::errors::*;
use crate::types::*;
use crate::matter::matter_parser_trait::MatterParserTrait;
use crate::state::state_trait::StateTrait;
use crate::state::state_enum::StateEnum;
use crate::templater::templater_trait::TemplaterTrait;

#[derive(Debug, Default)]
pub struct TemplaterWithLiquid {
}

impl TemplaterTrait for TemplaterWithLiquid {

    //TODO
    fn new() -> Self {
        trace!("templater_with_liquid.rs new");
        TemplaterWithLiquid {
        }
    }

    //TODO
    fn new_with_args(_args: &Args) -> Self {
        trace!("templater_with_liquid.rs new_with_args");
        TemplaterWithLiquid {
        }
    }

    //TODO
    fn template_name_default(&self) -> String {
        trace!("templater_with_liquid.rs template_name_default");
        String::from("default")
    }

    //TODO
    fn template_content_text_default(&self) -> String {
        trace!("templater_with_liquid.rs template_content_text_default");
        String::from("{{ content }}")
    }


    //TODO
    fn register_template_via_name_and_content_text(&mut self, name: &str, content_text: &str) -> Result<()> {
        trace!("templater_with_liquid.rs register_template_via_name_and_content_text");
        Ok(())
    }

    //TODO
    fn register_template_via_name_and_content_file(&mut self, name: &str, content_file: &PathBuf) -> Result<()> {
        trace!("templater_with_liquid.rs register_template_via_name_and_content_file");
        Ok(())
    }

    //TODO
    fn contains_any_template(&self) -> bool {
        trace!("templater_with_liquid.rs contains_any_template");
        false
    }

    //TODO
    fn contains_template_name(&self, name: &str) -> bool {
        trace!("templater_with_liquid.rs contains_template_name");
        false
    }

    //TODO
    fn template_names_as_set_str(&self) -> Set<&str> {
        trace!("templater_with_liquid.rs template_names_as_set_str");
        Set::new()
    }

    //TODO
    fn render_template_with_state_enum(&self, template_name: &str, state_enum: &StateEnum) -> Result<HtmlString> {
        trace!("templater_with_liquid.rs render_template_with_state_enum");
        Ok(HtmlString::from(""))
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use lazy_static::*;
    use crate::app::args::Args;
    use crate::matter::matter_parser_trait::MatterParserTrait;
    use crate::matter::matter_parser_with_btms::MatterParserWithBTMS;
    use crate::matter::matter_parser_with_json::MatterParserWithJSON;
    use crate::matter::matter_parser_with_toml::MatterParserWithTOML;
    use crate::matter::matter_parser_with_yaml::MatterParserWithYAML;
    use crate::state::state_enum::StateEnum;
    use crate::state::state_with_btms::StateWithBTMS;
    use crate::state::state_with_json::StateWithJSON;
    use crate::state::state_with_toml::StateWithTOML;
    use crate::state::state_with_yaml::StateWithYAML;

    const FAB_OUTPUT_HTML: &str = "my content";

    type TemplaterX = TemplaterWithLiquid;

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
    fn test_register_template_via_name_and_content_text() {
        let mut templater = TemplaterX::new();
        let name = "alpha";
        let content_text = "{{ bravo }}";
        assert_eq!(templater.contains_template_name("alpha"), false);
        assert_eq!(templater.contains_template_name("charlie"), false);
        let result = templater.register_template_via_name_and_content_text(&name, &content_text);
        assert!(result.is_ok());
        assert_eq!(templater.contains_template_name("alpha"), true);
        assert_eq!(templater.contains_template_name("charlie"), false);
    }

    #[test]
    fn test_register_template_via_name_and_content_file() {
        let mut templater = TemplaterX::new();
        let name = "alpha";
        let content_file = TESTS_DIR
            .join("src")
            .join("f")
            .join("register_template_via_name_and_content_file")
            .join("template.html");
        assert!(content_file.exists());
        assert_eq!(templater.contains_template_name("alpha"), false);
        assert_eq!(templater.contains_template_name("charlie"), false);
        let result = templater.register_template_via_name_and_content_file(&name, &content_file);
        assert!(result.is_ok());
        assert_eq!(templater.contains_template_name("alpha"), true);
        assert_eq!(templater.contains_template_name("charlie"), false);
    }

    #[test]
    fn test_contains_any_template() {
        let mut templater  = TemplaterX::new();
        assert_eq!(templater.contains_any_template(), false);
        templater.register_template_via_default().expect("register_template_via_default");
        assert_eq!(templater.contains_any_template(), true);
    }

    #[test]
    fn test_contains_template_name() {
        let mut templater  = TemplaterX::new();
        assert_eq!(templater.contains_template_name("default"), false);
        templater.register_template_via_default().expect("register_template_via_default");
        assert_eq!(templater.contains_template_name("default"), true);
    }

    #[test]
    fn test_template_names_as_set_str() {
        let mut templater = TemplaterX::new();
        let name_0: &str = "my-name-0";
        let name_1: &str = "my-name-1";
        templater.register_template_via_name_and_content_text(&name_0, "my text 0").expect("register_template_via_name_and_content_text");
        templater.register_template_via_name_and_content_text(&name_1, "my text 1").expect("register_template_via_name_and_content_text");
        let actual: Set<&str> = templater.template_names_as_set_str();
        let expect: Set<&str> = set!(name_0, name_1);
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_render_template_with_state_enum_x_btms() {
        let mut templater = TemplaterX::new();
        templater.register_template_via_default().expect("default");
        let matter_text = indoc!{r#"
            <!--
                title: my title
                content: my content
            -->
        "#};
        let name = templater.template_name_default();
        let state: StateWithBTMS = MatterParserWithBTMS{}.parse_matter_text_to_state(matter_text).expect("parse_matter_text_to_state");
        let state_enum = StateEnum::StateWithBTMS(state);
        let actual = templater.render_template_with_state_enum(&name, &state_enum).expect("render_template_with_state");
        assert_eq!(actual, FAB_OUTPUT_HTML);
    }

    #[test]
    fn test_render_template_with_state_enum_x_json() {
        let mut templater = TemplaterX::new();
        templater.register_template_via_default().expect("default");
        let matter_text = indoc!{r#"
            {
                "title": "my title",
                "content": "my content"
            }
        "#};
        let name = templater.template_name_default();
        let state: StateWithJSON = MatterParserWithJSON{}.parse_matter_text_to_state(matter_text).expect("parse_matter_text_to_state");
        let state_enum = StateEnum::StateWithJSON(state);
        let actual = templater.render_template_with_state_enum(&name, &state_enum).expect("render_template_with_state");
        assert_eq!(actual, FAB_OUTPUT_HTML);
    }

    #[test]
    fn test_render_template_with_state_enum_x_toml() {
        let mut templater = TemplaterX::new();
        templater.register_template_via_default().expect("default");
        let matter_text = indoc!{r#"
            title = "my title"
            content = "my content"
        "#};
        let name = templater.template_name_default();
        let state: StateWithTOML = MatterParserWithTOML{}.parse_matter_text_to_state(matter_text).expect("parse_matter_text_to_state");
        let state_enum = StateEnum::StateWithTOML(state);
        let actual = templater.render_template_with_state_enum(&name, &state_enum).expect("render_template_with_state");
        assert_eq!(actual, FAB_OUTPUT_HTML);
    }

    #[test]
    fn test_render_template_with_state_enum_x_yaml() {
        let mut templater = TemplaterX::new();
        templater.register_template_via_default().expect("default");
        let matter_text = indoc!{r#"
            title: "my title"
            content: "my content"
        "#};
        let name = templater.template_name_default();
        let state: StateWithYAML = MatterParserWithYAML{}.parse_matter_text_to_state(matter_text).expect("parse_matter_text_to_state");
        let state_enum = StateEnum::StateWithYAML(state);
        let actual = templater.render_template_with_state_enum(&name, &state_enum).expect("render_template_with_state");
        assert_eq!(actual, FAB_OUTPUT_HTML);
    }

}
