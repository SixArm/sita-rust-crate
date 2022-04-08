//! Templater with Tera

use std::path::PathBuf;
use crate::app::args::Args;
use crate::errors::*;
use crate::types::*;
use crate::state::state_enum::StateEnum;
use crate::templater::templater_trait::TemplaterTrait;

#[derive(Debug, Default)]
pub struct TemplaterWithTera {
    pub tera: ::tera::Tera,
}

impl TemplaterTrait for TemplaterWithTera {

    fn new() -> Self {
        trace!("templater_with_tera.rs new");
        let my_tera = ::tera::Tera::default();
        TemplaterWithTera {
            tera: my_tera,
        }
    }

    fn new_with_args(_args: &Args) -> Self {
        trace!("templater_with_tera.rs new_with_args");
        let mut my_tera = ::tera::Tera::default();
        my_tera.autoescape_on(vec![]); // disable autoescaping completely
        TemplaterWithTera {
            tera: my_tera,
        }
    }

    fn template_name_default(&self) -> String {
        trace!("templater_with_tera.rs template_name_default");
        String::from("default")
    }

    fn template_content_text_default(&self) -> String {
        trace!("templater_with_tera.rs template_name_default");
        String::from("{{ content }}")
    }

    fn register_template_via_name_and_content_text(&mut self, name: &str, content_text: &str) -> Result<()> {
        trace!("templater_with_tera.rs register_template_via_name_and_content_file: name: {} content_text: …", &name);
        self.tera.add_raw_template(&name, &content_text)
        .chain_err(|| "register_template_via_name_and_content_text")
    }

    fn register_template_via_name_and_content_file(&mut self, name: &str, content_file: &PathBuf) -> Result<()> {
        trace!("templater_with_tera.rs register_template_via_name_and_content_file: name: {} content_file: {:?}", &name, &content_file);
        self.tera.add_template_file(&content_file, Some(name))
        .chain_err(|| "register_template_via_name_and_content_file")
    }

    fn contains_any_template(&self) -> bool {
        trace!("templater_with_tera.rs contains_any_template");
        self.tera.get_template_names().nth(0).is_some()
    }

    fn contains_template_name(&self, name: &str) -> bool {
        trace!("templater_with_tera.rs contains_template_name");
        self.tera.get_template_names().any(|x| x == name)
    }

    fn template_names_as_set_str(&self) -> Set<&str> {
        trace!("templater_with_tera.rs template_names_as_set_str");
        self.tera.get_template_names().collect::<_>()
    }

    fn register_helper_via_name_and_content_text(&mut self, name: &str, content_text: &str) -> Result<()> {
        trace!("templater_with_tera.rs register_helper_via_name_and_content_file: name: {} content_text: {}", &name, &content_text);
        panic!("todo");
    }

    fn register_helper_via_name_and_content_file(&mut self, name: &str, content_file: &PathBuf) -> Result<()> {
        trace!("templater_with_tera.rs register_helper_via_name_and_content_file: name: {} content_file: {:?}", &name, &content_file);
        panic!("todo");
    }

    fn render_template_with_state_enum(&self, template_name: &str, state_enum: &StateEnum) -> Result<HtmlString> {
        trace!("templater_with_tera.rs render_template_with_state_enum");
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
        crate::state::state_enum::StateEnum::StateWithBTMS(x) => ::tera::Context::from_serialize(x),
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

    lazy_static! {
        pub static ref TESTS_DIR: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests"].iter().collect::<PathBuf>();
        pub static ref TESTY_DIR: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests", "src", "templater", "templater_with_tera"].iter().collect::<PathBuf>();
    }

    const FAB_OUTPUT_HTML: &str = "my content";

    type TemplaterX = TemplaterWithTera;

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
        let content_file = TESTY_DIR
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
    fn test_contains_any_template_x_true() {
        let mut templater  = TemplaterX::new();
        templater.register_template_via_name_and_content_text("my-name", "my-content").expect("register_template_via_name_and_content_text");
        let flag = templater.contains_any_template();
        assert_eq!(flag, true);
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

    //TODO
    // #[test]
    // fn test_register_helper_via_name_and_content_text() {
    //     let mut templater = TemplaterX::new();
    //     let name = "alpha";
    //     let content_text = "{{ bravo }}";
    //     assert_eq!(templater.contains_helper_name("alpha"), false);
    //     assert_eq!(templater.contains_helper_name("charlie"), false);
    //     let result = templater.register_helper_via_name_and_content_text(&name, &content_text);
    //     assert!(result.is_ok());
    //     assert_eq!(templater.contains_helper_name("alpha"), true);
    //     assert_eq!(templater.contains_helper_name("charlie"), false);
    // }

    //TODO
    // #[test]
    // fn test_register_helper_via_name_and_content_file() {
    //     let mut templater = TemplaterX::new();
    //     let name = "alpha";
    //     let content_file = TESTY_DIR
    //          .join("register_helper_via_name_and_content_file")
    //          .join("helper.rhai");
    //     assert!(content_file.exists());
    //     assert_eq!(templater.contains_helper_name("alpha"), false);
    //     assert_eq!(templater.contains_helper_name("charlie"), false);
    //     let result = templater.register_helper_via_name_and_content_file(&name, &content_file);
    //     assert!(result.is_ok());
    //     assert_eq!(templater.contains_helper_name("alpha"), true);
    //     assert_eq!(templater.contains_helper_name("charlie"), false);
    // }

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
