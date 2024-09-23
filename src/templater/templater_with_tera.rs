//! Templater with Tera

use tera;
use crate::app::args::Args;
use crate::types::{html::*, set::*};
use crate::state::state_enum::StateEnum;
use crate::templater::templater_trait::TemplaterTrait;

#[derive(Debug, Default)]
pub struct TemplaterWithTera {
    pub tera: ::tera::Tera,
}
impl TemplaterTrait for TemplaterWithTera {

    // fn as_any(&self) -> &dyn Any {
    //     self
    // }

    // fn to_templater_enum(&self) -> TemplaterEnum {
    //     TemplaterEnum::TemplaterWithTera(self.clone())
    // }

    fn new() -> Self {
        trace!("new");
        let my_tera = ::tera::Tera::default();
        TemplaterWithTera {
            tera: my_tera,
        }
    }

    fn new_with_args(
        _args: &Args
    ) -> Self {
        trace!("new_with_args");
        let mut my_tera = ::tera::Tera::default();
        my_tera.autoescape_on(vec![]); // disable autoescaping completely
        TemplaterWithTera {
            tera: my_tera,
        }
    }

    fn template_name_default(
        &self
    ) -> String {
        trace!("template_name_default");
        String::from("default")
    }

    fn template_content_text_default(
        &self
    ) -> String {
        trace!("template_name_default");
        String::from("{{ content }}")
    }

    fn register_template_via_name_and_content_text(
        &mut self,
        name: impl AsRef<str>,
        content_text: impl AsRef<str>
    ) -> Result<(), Error> {
        let name = name.as_ref();
        let content_text = content_text.as_ref();
        trace!("register_template_via_name_and_content_text ➡ name: {:?}, content_text.len(): {}", name, content_text.len());
        self.tera.add_raw_template(name, content_text)
        .map_or_else(
            |err| Err(Error::RegisterTemplateViaNameAndContentText(err)),
            |()| Ok(())
        )
    }

    fn contains_any_template(
        &self
    ) -> bool {
        trace!("contains_any_template");
        self.tera.get_template_names().nth(0).is_some()
    }

    fn contains_template_name(
        &self,
        name: impl AsRef<str>
    ) -> bool {
        let name = name.as_ref();
        trace!("contains_template_name ➡ name: {}", name);
        self.tera.get_template_names().any(|x| x == name)
    }

    fn template_names_as_set_str(
        &self
    ) -> Set<&str> {
        trace!("template_names_as_set_str");
        self.tera.get_template_names().collect::<_>()
    }

    fn render_template_with_state_enum(
        &self,
        name: impl AsRef<str>,
        state_enum: &StateEnum
    ) -> Result<HtmlString, Error> {
        let name = name.as_ref();
        trace!("render_template_with_state_enum ➡ name: {}", name);
        let context = from_state_enum_to_tera_context(&state_enum)
        .map_or_else(
            |err| Err(Error::FromStateEnumToTeraContext(err)),
            |context| Ok(context)
        )?;
        debug!("context: {:?}", &context);
        let html = self.tera.render(name, &context)
        .map_or_else(
            |err: tera::Error| Err(Error::Render(err)),
            |html_string| Ok(html_string)
        )?;
        Ok(html)
    }

}

pub fn from_state_enum_to_tera_context(
    state_enum: &crate::state::state_enum::StateEnum
) -> ::tera::Result<::tera::Context> {
    match state_enum {
        crate::state::state_enum::StateEnum::StateWithMap(x) => ::tera::Context::from_serialize(x),
        crate::state::state_enum::StateEnum::StateWithJSON(x) => ::tera::Context::from_serialize(x),
        crate::state::state_enum::StateEnum::StateWithTOML(x) => ::tera::Context::from_serialize(x),
        crate::state::state_enum::StateEnum::StateWithYAML(x) => ::tera::Context::from_serialize(x),
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {

    #[error ("RegisterTemplateViaNameAndContentText ➡ {0:?}")]
    RegisterTemplateViaNameAndContentText(tera::Error),

    #[error ("RegisterTemplateViaNameAndContentFile ➡ {0:?}")]
    RegisterTemplateViaNameAndContentFile(tera::Error),

    #[error ("RegisterExtraViaNameAndContentText ➡ {0:?}")]
    RegisterExtraViaNameAndContentText(tera::Error),

    #[error ("RegisterExtraViaNameAndContentFile ➡ {0:?}")]
    RegisterExtraViaNameAndContentFile(tera::Error),

    #[error ("FromStateEnumToTeraContext ➡ {0:?}")]
    FromStateEnumToTeraContext(tera::Error),

    #[error ("Render ➡ {0:?}")]
    Render(tera::Error),

}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use once_cell::sync::Lazy;
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

    pub static TESTS_DIR: Lazy<PathBuf> = Lazy::new(||
        [env!("CARGO_MANIFEST_DIR"), "tests"].iter().collect::<PathBuf>()
    );

    pub static TESTY_DIR: Lazy<PathBuf> = Lazy::new(||
        [env!("CARGO_MANIFEST_DIR"), "tests", "src", "templater", "templater_with_tera"].iter().collect::<PathBuf>()
    );

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
        assert_eq!(
            templater.template_name_default(),
            "default"
        );
    }

    #[test]
    fn test_templater_content_text_default() {
        let templater = TemplaterX::new();
        assert_eq!(
            templater.template_content_text_default(),
            "{{ content }}"
        );
    }

    #[test]
    fn test_register_template_via_name_and_content_text() {
        let mut templater = TemplaterX::new();
        let name = "alfa";
        let content_text = "{{ bravo }}";
        assert_eq!(
            templater.contains_template_name("alfa"),
            false
        );
        assert_eq!(
            templater.contains_template_name("charlie"),
            false
        );
        templater.register_template_via_name_and_content_text(
            String::from(name),
            String::from(content_text)
        ).expect("register_template_via_name_and_content_text");
        assert_eq!(
            templater.contains_template_name("alfa"),
            true
        );
        assert_eq!(
            templater.contains_template_name("charlie"),
            false
        );
    }

    #[test]
    fn test_contains_any_template_x_true() {
        let mut templater  = TemplaterX::new();
        templater.register_template_via_name_and_content_text(
            "my-name",
            "my-content"
        ).expect("register_template_via_name_and_content_text");
        let flag = templater.contains_any_template();
        assert_eq!(
            flag,
            true
        );
    }

    #[test]
    fn test_contains_any_template() {
        let mut templater  = TemplaterX::new();
        assert_eq!(
            templater.contains_any_template(), false
        );
        templater.register_template_via_default().expect("register_template_via_default");
        assert_eq!(
            templater.contains_any_template(),
            true
        );
    }

    #[test]
    fn test_contains_template_name() {
        let mut templater  = TemplaterX::new();
        assert_eq!(
            templater.contains_template_name("default"), false
        );
        templater.register_template_via_default().expect("register_template_via_default");
        assert_eq!(
            templater.contains_template_name("default"),
            true
        );
    }


    #[test]
    fn test_template_names_as_set_str() {
        let mut templater = TemplaterX::new();
        let name_0: &str = "my-name-0";
        let name_1: &str = "my-name-1";
        let content_text_0 = "my text 0";
        let content_text_1 = "my text 1";
        templater.register_template_via_name_and_content_text(
            String::from(name_0),
            String::from(content_text_0)
        ).expect("register_template_via_name_and_content_text");
        templater.register_template_via_name_and_content_text(
            String::from(name_1),
            String::from(content_text_1)
        ).expect("register_template_via_name_and_content_text");
        let actual: Set<&str> = templater.template_names_as_set_str();
        let expect: Set<&str> = set!(name_0, name_1);
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_render_template_with_state_enum_x_html() {
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
    fn test_render_template_with_state_enum_x_markdown_comments() {
        let mut templater = TemplaterX::new();
        templater.register_template_via_default().expect("default");
        let matter_text = indoc!{r#"
            [//]: # (title: my title)
            [//]: # (content: my content)
        "#};
        let name = templater.template_name_default();
        let state: StateWithMap = MatterParserWithMarkdownComments{}.parse_matter_text_to_state(matter_text).expect("parse_matter_text_to_state");
        let state_enum = StateEnum::StateWithMap(state);
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
