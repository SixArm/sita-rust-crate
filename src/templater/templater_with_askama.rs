//! Templater with Askama

use askama;
use crate::app::args::Args;
use crate::types::{html::*, set::*};
use crate::state::state_enum::StateEnum;
use crate::templater::templater_trait::TemplaterTrait;

#[derive(Debug, Default)]
pub struct TemplaterWithAskama<'templater> {
    pub askama: ::askama::Askama<'templater>,
}

impl<'templater> TemplaterTrait for TemplaterWithAskama<'templater> {

    // fn as_any(&self) -> &dyn Any {
    //     self
    // }

    // fn to_templater_enum(&self) -> TemplaterEnum {
    //     TemplaterEnum::TemplaterWithAskama(self.clone())
    // }

    fn new() -> Self {
        trace!("new");
        let askama = Askama::default();
        TemplaterWithAskama {
            askama: askama,
        }
    }

    fn new_with_args(
        _args: &Args
    ) -> Self {
        trace!("new_with_args");
        let mut askama = Askama::default();
        askama.set_strict_mode(true);
        TemplaterWithAskama {
            askama: askama,
        }
    }

    fn template_name_default(
        &self
    ) -> &str {
        trace!("template_name_default");
        String::from("default")
    }

    fn template_content_default(
        &self
    ) -> &str {
        trace!("template_content_default");
        String::from("{{{ content }}}")
    }

    fn register_template_via_name_and_content(
        &mut self,
        name: impl AsRef<str>,
        content: impl AsRef<str>
    ) -> Result<(), impl std::error::Error> {
        trace!("register_template_via_name_and_content ➡  name: {:?}, content_text.len(): {}", name.as_ref(), content.as_ref().len());
        self.askama.register_template_string(name.as_ref(), &content)
        .map_or_else(
            |err| Err(Error::RegisterTemplateViaNameAndContentText(err)),
            |()| Ok(())
        )
    }

    fn contains_any_template(
        &self
    ) -> bool {
        trace!("contains_any_template");
        !self.askama.get_templates().is_empty()
    }

    fn contains_template_name(
        &self,
        name: impl AsRef<str>
    ) -> bool {
        trace!("contains_template_name");
        self.askama.get_template(name.as_ref()).is_some()
    }

    fn template_names_as_set_str(
        &self
    ) -> Set<&str> {
        trace!("template_names_as_set_str");
        let mut names: Set<&str> = Set::new();
        for key in self.askama.get_templates().keys() {
            names.insert(key);
        }
        names
    }

    fn render_template_with_state_enum(
        &self,
        template_name: impl AsRef<str>,
        state_enum: &StateEnum
    ) -> Result<HtmlString, impl std::error::Error> {
        trace!("render_template_with_state_enum");
        //TODO make generic
        match state_enum {
            StateEnum::StateWithMap(x) =>  self.askama.render(template_name.as_ref(), x),
            StateEnum::StateWithJSON(x) => self.askama.render(template_name.as_ref(), x),
            StateEnum::StateWithTOML(x) => self.askama.render(template_name.as_ref(), x),
            StateEnum::StateWithYAML(x) => self.askama.render(template_name.as_ref(), x),
        }.map_or_else(
            |err| Err(Error::Render(err)),
            |html_string| Ok(html_string)
        )
    }

}

#[derive(thiserror::Error, Debug)]
pub enum Error {

    #[error ("RegisterTemplateViaNameAndContentText ➡ {0:?}")]
    RegisterTemplateViaNameAndContentText(askama::TemplateError),

    #[error ("Render ➡ {0:?}")]
    Render(askama::RenderError),

}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use crate::app::args::Args;
    use crate::matter::matter_parser_trait::MatterParserTrait;
    use crate::matter::matter_parser_with_html::MatterParserWithHTML;
    use crate::matter::matter_parser_with_json::MatterParserWithJSON;
    use crate::matter::matter_parser_with_markdown_comments::MatterParserWithMarkdownComments;
    use crate::matter::matter_parser_with_toml::MatterParserWithTOML;
    use crate::matter::matter_parser_with_yaml::MatterParserWithYAML;
    use crate::state::state_enum::StateEnum;
    use crate::state::state_with_map::StateWithMap;
    use crate::state::state_with_json::StateWithJSON;
    use crate::state::state_with_toml::StateWithTOML;
    use crate::state::state_with_yaml::StateWithYAML;

    const FAB_OUTPUT_HTML: &str = "my content";

    type TemplaterX<'templater> = TemplaterWithAskama<'templater>;

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
        assert_eq!(templater.template_name_default(), super::TEMPLATE_NAME_DEFAULT);
    }

    #[test]
    fn test_templater_content_default() {
        let templater = TemplaterX::new();
        assert_eq!(templater.template_content_default(), super::TEMPLATE_CONTENT_DEFAULT);
    }

    #[test]
    fn test_register_template_via_name_and_content() {
        let mut templater = TemplaterX::new();
        let name = "alfa";
        let content_text = "{{ bravo }}";
        assert!(!templater.contains_template_name(name));
        templater.register_template_via_name_and_content(
            String::from(name),
            String::from(content_text)
        ).expect("register_template_via_name_and_content");
        assert!(templater.contains_template_name(name));
    }

    #[test]
    fn test_contains_any_template() {
        let mut templater  = TemplaterX::new();
        assert!(!templater.contains_any_template());
        templater.register_template_via_default().expect("register_template_via_default");
        assert!(templater.contains_any_template());
    }

    #[test]
    fn test_contains_template_name() {
        let mut templater  = TemplaterX::new();
        assert!(!templater.contains_template_name("default"));
        templater.register_template_via_default().expect("register_template_via_default");
        assert!(templater.contains_template_name("default"));
    }

    #[test]
    fn test_template_names_as_set_string() {
        let mut templater = TemplaterX::new();
        let name_0: &str = "my-name-0";
        let name_1: &str = "my-name-1";
        let content_text_0 = "my text 0";
        let content_text_1 = "my text 1";
        templater.register_template_via_name_and_content(
            String::from(name_0),
            String::from(content_text_0)
        ).expect("register_template_via_name_and_content");
        templater.register_template_via_name_and_content(
            String::from(name_1),
            String::from(content_text_1)
        ).expect("register_template_via_name_and_content");
        let actual: Set<&str> = templater.template_names_as_set_str();
        let expect: Set<&str> = set!(name_0, name_1);
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_render_template_x_matter_parser_with_html() {
        let mut templater = TemplaterX::new();
        templater.register_template_via_default().expect("default");
        let matter_text = indoc!{r#"
            <!--
            title: my title
            content: my content
            -->
        "#};
        let name = templater.template_name_default();
        let state: StateWithMap = MatterParserWithHTML{}.parse_matter_text_to_state(matter_text).expect("parse_matter_text_to_state");
        let state_enum = StateEnum::StateWithMap(state);
        let actual = templater.render_template_with_state_enum(&name, &state_enum).expect("render_template_with_state");
        assert_eq!(actual, FAB_OUTPUT_HTML);
    }

    #[test]
    fn test_render_template_x_matter_parser_with_json() {
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
    fn test_render_template_x_matter_parser_with_markdown_comments() {
        let mut templater = TemplaterX::new();
        templater.register_template_via_default().expect("default");
        let matter_text = indoc!{r#"
            [//]: # (title: my title)
            [//]: # (content: my content)
        "#};
        let name: String = templater.template_name_default();
        let state: StateWithMap = MatterParserWithMarkdownComments{}.parse_matter_text_to_state(matter_text).expect("parse_matter_text_to_state");
        let state_enum = StateEnum::StateWithMap(state);
        let actual = templater.render_template_with_state_enum(&name, &state_enum).expect("render_template_with_state");
        assert_eq!(actual, FAB_OUTPUT_HTML);
    }

    #[test]
    fn test_render_template_x_matter_parser_with_toml() {
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
    fn test_render_template_x_matter_parser_with_yaml() {
        let mut templater = TemplaterX::new();
        templater.register_template_via_default().expect("default");
        let matter_text = indoc!{r#"
            title: my title
            content: my content
        "#};
        let name = templater.template_name_default();
        let state: StateWithYAML = MatterParserWithYAML{}.parse_matter_text_to_state(matter_text).expect("parse_matter_text_to_state");
        let state_enum = StateEnum::StateWithYAML(state);
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
