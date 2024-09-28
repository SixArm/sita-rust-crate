//! Templater management.
//!
//! This can use Handlebars, Liquid, Tera.
//!
//! This can be expanded for potential future formats.

use crate::app::args::Args;
use crate::types::{html::*, set::*};

pub trait TemplaterTrait: std::fmt::Debug {

    /// Convert from the specific to any type.
    // #[allow(dead_code)]
    // fn as_any(&self) -> &dyn Any;

    /// Convert from the specific to the corresponding enum.
    // fn to_templater_enum(&self) -> TemplaterEnum;

    // Create a new templater.
    //
    // Example:
    //
    // ```
    // let templater: Templater = TemplaterWithHandlebars::new();
    // ```
    //
    #[allow(dead_code)]
    fn new() -> Self where Self: Sized;

    // Create a new templater with args.
    //
    // Example:
    //
    // ```
    // let args = Args::default();
    // let templater: Templater = TemplaterWithHandlebars::new_with_args(&args);
    // ```
    //
    #[allow(dead_code)]
    fn new_with_args(args: &Args) -> Self where Self: Sized;

    // Get the template name default e.g. "default".
    //
    // ```
    // let name = template_name_default();
    // assert_eq!(name, "default");
    // ```
    //
    #[allow(dead_code)]
    fn template_name_default(&self) -> &str;

    // Get the template content text e.g. "{{{ content }}}".
    //
    // ```
    // let content_text = template_content_default();
    // assert_eq!(content_text, "{{{ content }}}");
    // ```
    //
    #[allow(dead_code)]
    fn template_content_default(&self) -> &str;

    // Add a default template.
    //
    // Example:
    //
    // ```
    // let templater: Templater = TemplaterWithHandlebars::new();
    // register_template_via_default(templater);
    // //-> Handlebars now has a template name "default" with content "{{{ content }}}"
    // ```
    //
    #[allow(dead_code)]
    fn register_template_via_default(
        &mut self
    ) -> Result<(), impl std::error::Error> where Self: Sized {
        self.register_template_via_name_and_content(
            String::from(self.template_name_default()),
            String::from(self.template_content_default())
        )
    }

    // Add a template via template name (i.e. key) and template text (i.e. value).
    //
    // Example:
    //
    // ```
    // let templater: Templater = TemplaterWithHandlebars::new();
    // let name = "alfa";
    // let content_text = "{{ bravo }}";
    // templater.register_template_via_name_and_content(&name, &content_text);
    // ```
    //
    #[allow(dead_code)]
    fn register_template_via_name_and_content(
        &mut self,
        name: impl AsRef<str>,
        content: impl AsRef<str>
    ) -> Result<(), impl std::error::Error>;

    // Does the templater contain any template?
    //
    // Example:
    //
    // ```
    // let mut templater: Templater = TemplaterWithHandlebars::new();
    // templater.register_template_via_name_and_content("alfa", "bravo");
    // let flag = templater.contains_any_template();
    // assert_eq!(flag, true);
    // ```
    //
    #[allow(dead_code)]
    fn contains_any_template(
        &self
    ) -> bool;

    // Does the templater contain a specific template key i.e. template name?
    //
    // Example:
    //
    // ```
    // let mut templater: Templater = TemplaterWithHandlebars::new();
    // templater.register_template_via_name_and_content("alfa", "bravo");
    // let flag = templater.contains_template_name("alfa");
    // assert_eq!(flag, true);
    // ```
    //
    #[allow(dead_code)]
    fn contains_template_name(
        &self,
        name: impl AsRef<str>
    ) -> bool;

    // Get the template names.
    //
    // Example:
    //
    // ```
    // let mut templater: Templater = TemplaterWithHandlebars::new();
    // register_template_via_name_and_content("alfa", "alfa text".into());
    // register_template_via_name_and_content("bravo", "bravo text".into());
    // let template_names: Set<&str> = template_names_as_set_str(&templater);
    // assert_eq!(template_names, set!["alfa", "bravo"]);
    // ```
    //
    #[allow(dead_code)]
    fn template_names_as_set_str(
        &self
    ) -> Set<&str>;

    // Render a template name with the state.
    //
    // ```
    // let templater: Templater = TemplaterWithHandlebars::new();
    // templater.register_template_via_name_and_content("alfa", "<p>{{ bravo }}</p>");
    // let state_enum = crate::state::state_enum::StateEnum::StateWithMap(map!("bravo" => "charlie"));
    // let html = templater.render_template_with_state_enum("alfa", &state_enum).expect("render_template_with_state_enum");
    // assert_eq!(html, "<p>charlie</p>");
    // ```
    //
    #[allow(dead_code)]
    fn render_template_with_state_enum(
        &self,
        name: impl AsRef<str>,
        state_enum: &crate::state::state_enum::StateEnum
    ) -> Result<HtmlString, impl std::error::Error>;

}

#[cfg(test)]
mod tests {
    use super::*;
    use assertables::*;
    use crate::app::args::Args;
    use crate::templater::templater_with_handlebars::TemplaterWithHandlebars;

    type TemplaterX<'templater> = TemplaterWithHandlebars<'templater>;

    #[test]
    fn test_new() {
        let templater = TemplaterX::new();
        assert!(!templater.contains_any_template()); // Exists
    }

    #[test]
    fn test_new_with_args() {
        let args = Args::default();
        let templater = TemplaterX::new_with_args(&args);
        assert!(!templater.contains_any_template()); // Exists
    }

    #[test]
    fn test_template_name_default() {
        let templater = TemplaterX::new();
        let actual = templater.template_name_default();
        assert_gt!(actual.len(), 0);
    }

    #[test]
    fn test_template_content_default() {
        let templater = TemplaterX::new();
        let actual = templater.template_content_default();
        assert_gt!(actual.len(), 0);
    }

    #[test]
    fn test_register_template_via_default() {
        let mut templater = TemplaterX::new();
        assert!(!templater.contains_any_template());
        templater.register_template_via_default().expect("register_template_via_default");
        assert!(templater.contains_any_template());
    }
    
    #[test]
    fn test_register_template_via_name_and_content() {
        let mut templater = TemplaterX::new();
        let name = "alfa";
        let content_text = "{{ bravo }}";
        assert!(!templater.contains_any_template());
        templater.register_template_via_name_and_content(
            name,
            String::from(content_text)
        ).expect("register_template_via_name_and_content");
        assert!(templater.contains_any_template());
    }

    #[test]
    fn test_contains_any_template_x_true() {
        let mut templater  = TemplaterX::new();
        templater.register_template_via_name_and_content(
            "my-name",
            "my-content"
        ).expect("register_template_via_name_and_content");
        let flag = templater.contains_any_template();
        assert_eq!(
            flag,
            true
        );
    }

    #[test]
    fn test_contains_any_template_x_false() {
        let templater = TemplaterX::new();
        let flag = templater.contains_any_template();
        assert_eq!(
            flag,
            false
        );
    }

    #[test]
    fn test_contains_template_name() {
        let mut templater = TemplaterX::new();
        let name = "alfa";
        let content_text = "bravo";
        assert!(!templater.contains_template_name(name));
        templater.register_template_via_name_and_content(
            name,
            content_text
        ).expect("register_template_via_name_and_content");
        assert!(templater.contains_template_name(name));
    }
    
    #[test]
    fn test_template_names_as_set_str() {
        let mut templater: TemplaterWithHandlebars = TemplaterX::new();
        let name_0 = "my-name-0";
        let name_1 = "my-name-1";
        let content_text_0 = "my text 0";
        let content_text_1 = "my text 1";
        templater.register_template_via_name_and_content(
            name_0,
            content_text_0
        ).expect("register_template_via_name_and_content");
        templater.register_template_via_name_and_content(
            name_1,
            content_text_1
        ).expect("register_template_via_name_and_content");
        let actual: Set<&str> = templater.template_names_as_set_str();
        let expect: Set<&str> = set!(name_0, name_1);
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_render_template_with_state_enum() {
        let mut templater: TemplaterWithHandlebars = TemplaterX::new();
        templater.register_template_via_name_and_content("alfa", "<p>{{ bravo }}</p>").expect("register_template_via_name_and_content");
        let map = map!(
            String::from("bravo") => String::from("charlie")
        );
        let state_enum = crate::state::state_enum::StateEnum::StateWithMap(map);
        let html = templater.render_template_with_state_enum("alfa", &state_enum).expect("render_template_with_state_enum");
        assert_eq!(html, "<p>charlie</p>");
    }
    
}

