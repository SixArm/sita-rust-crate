//! Templater management.
//!
//! This can use Handlebars, Liquid, Tera.
//!
//! This can be expanded for potential future formats.

use std::path::PathBuf;
use crate::app::args::Args;
use crate::errors::*;
use crate::types::*;

pub trait TemplaterTrait {

    // Create a new templater.
    //
    // Example:
    //
    // ```
    // let templater: Templater = TemplaterWithHandlebars::new();
    // ```
    //
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
    fn new_with_args(args: &Args) -> Self where Self: Sized;

    // Get the template name default e.g. "default".
    //
    // ```
    // let name = template_name_default();
    // assert_eq!(name, "default");
    // ```
    //
    fn template_name_default(&self) -> String;

    // Get the template content text e.g. "{{ content }}".
    //
    // ```
    // let content_text = template_content_text_default();
    // assert_eq!(content_text, "{{ content }}");
    // ```
    //
    fn template_content_text_default(&self) -> String;

    // Add a default template.
    //
    // Example:
    //
    // ```
    // let templater: Templater = TemplaterWithHandlebars::new();
    // register_template_via_default(templater);
    // //-> Handlebars now has a template name "default" with content "{{ content }}"
    // ```
    //
    fn register_template_via_default(&mut self) -> Result<()> where Self: Sized {
        let name = &self.template_name_default();
        let content = &self.template_content_text_default();
        self.register_template_via_name_and_content_text(&name, &content)
    }

    // Add a template via template name (i.e. key) and template text (i.e. value).
    //
    // Example:
    //
    // ```
    // let templater: Templater = TemplaterWithHandlebars::new();
    // let name = "alpha";
    // let content_text = "{{ bravo }}";
    // templater.register_template_via_name_and_content_text(&name, &content_text);
    // ```
    //
    fn register_template_via_name_and_content_text(&mut self, name: &str, content_text: &str) -> Result<()>;

    // Add a template via template name (i.e. key) and template file (i.e. value).
    //
    // Example:
    //
    // ```
    // let mut templater: Templater = TemplaterWithHandlebars::new();
    // let name = "alpha";
    // let file = PathBuf::from("template.html")
    // register_template_via_name_and_content_file(&name, &content_file);
    // ```
    //
    fn register_template_via_name_and_content_file(&mut self, name: &str, content_file: &PathBuf) -> Result<()>;

    // Does the templater contain any template?
    //
    // Example:
    //
    // ```
    // let mut templater: Templater = TemplaterWithHandlebars::new();
    // templater.register_template_via_name_and_content_text("alpha", "bravo");
    // let flag = templater.contains_any_template();
    // assert_eq!(flag, true);
    // ```
    //
    fn contains_any_template(&self) -> bool;

    // Does the templater contain a specific template key i.e. template name?
    //
    // Example:
    //
    // ```
    // let mut templater: Templater = TemplaterWithHandlebars::new();
    // templater.register_template_via_name_and_content_text("alpha", "bravo");
    // let flag = templater.contains_template_name("alpha");
    // assert_eq!(flag, true);
    // ```
    //
    fn contains_template_name(&self, name: &str) -> bool;

    // Get the template names.
    //
    // Example:
    //
    // ```
    // let mut templater: Templater = TemplaterWithHandlebars::new();
    // register_template_via_name_and_content_text("alpha", "alpha text".into());
    // register_template_via_name_and_content_text("bravo", "bravo text".into());
    // let template_names: Set<&str> = template_names_as_set_str(&templater);
    // assert_eq!(template_names, set!["alpha", "bravo"]);
    // ```
    //
    fn template_names_as_set_str(&self) -> Set<&str>;

    // Register a helper via helper name (i.e. key) and helper text (i.e. value).
    //
    // Example:
    //
    // ```
    // let templater: Templater = TemplaterWithHandlebars::new();
    // let name = "alpha";
    // let content_text = "{{ bravo }}";
    // templater.register_helper_via_name_and_content_text(&name, &content_text);
    // ```
    //
    fn register_helper_via_name_and_content_text(&mut self, name: &str, content_text: &str) -> Result<()>;

    // Register a helper via helper name (i.e. key) and helper file (i.e. value).
    //
    // Example:
    //
    // ```
    // let mut templater: Templater = TemplaterWithHandlebars::new();
    // let name = "alpha";
    // let file = PathBuf::from("helper.html")
    // register_helper_via_name_and_content_file(&name, &content_file);
    // ```
    //
    fn register_helper_via_name_and_content_file(&mut self, name: &str, content_file: &PathBuf) -> Result<()>;

    // Render a template name with the state.
    //
    // ```
    // let templater: Templater = TemplaterWithHandlebars::new();
    // templater.register_template_via_name_and_content_text("alpha", "<p>{{ content }}</p>");
    // let name = template_name_default();
    // let json: ::serde_json::Value = ::serde_json::from_str(indoc!{r#"{"content": "bravo"}"#}).unwrap();
    // let state_enum: StateEnum::JSON(json);
    // let html = templater.render(&name, &state_enum);
    // assert_eq!(html, "<p>bravo</p>");
    // ```
    //
    fn render_template_with_state_enum(&self, template_name: &str, state_enum: &crate::state::state_enum::StateEnum) -> Result<HtmlString>;

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::args::Args;
    use crate::templater::templater_with_handlebars::TemplaterWithHandlebars;

    const FAB_OUTPUT_HTML: &str = "my content";

    type TemplaterX<'templater> = TemplaterWithHandlebars<'templater>;

    #[test]
    fn test_new() {
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
    fn test_register_template_via_name_and_content_text() {
        let mut templater = TemplaterX::new();
        let name = "alpha";
        let content_text = "{{ bravo }}";
        assert!(!templater.contains_any_template());
        let result = templater.register_template_via_name_and_content_text(&name, &content_text);
        assert!(result.is_ok());
        assert!(templater.contains_any_template());
    }

    #[test]
    fn test_register_template_via_name_and_content_file() {
        let mut templater = TemplaterX::new();
        let name = "alpha";
        let content_file = crate::test::TESTS_DIR
            .join("src")
            .join("templater")
            .join("templater_trait")
            .join("register_template_via_name_and_content_file")
            .join("template.html");
        assert!(content_file.exists(), "content_file: {:?}", content_file);
        assert!(!templater.contains_any_template());
        let result = templater.register_template_via_name_and_content_file(&name, &content_file);
        assert!(result.is_ok());
        assert!(templater.contains_any_template());
    }

    #[test]
    fn test_contains_any_template_x_true() {
        let mut templater  = TemplaterX::new();
        templater.register_template_via_name_and_content_text("my-name", "my-content").expect("register_template_via_name_and_content_text");
        let flag = templater.contains_any_template();
        assert_eq!(flag, true);
    }

    #[test]
    fn test_contains_any_template_x_false() {
        let templater = TemplaterX::new();
        let flag = templater.contains_any_template();
        assert_eq!(flag, false);
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

}
