use indoc::indoc;
use std::path::PathBuf;
use crate::app::args::Args;
use crate::errors::*;
use crate::types::*;

pub trait Templater {

    // Create a new templater.
    //
    // Example:
    //
    // ```
    // let templater: Templater = TemplaterWithTera::new();
    // ```
    //
    fn new() -> Self where Self: Sized;

    // Create a new templater with args.
    //
    // Example:
    //
    // ```
    // let args = Args::default();
    // let templater: Templater = TemplaterWithTera::new_with_args(&args);
    // ```
    //
    fn new_with_args(args: &Args) -> Self where Self: Sized;

    // Add a template via name and text.
    //
    // Example:
    //
    // ```
    // let templater: Templater = TemplaterWithTera::new();
    // let name = "alpha";
    // let text = "<p>{{ bravo }}</p>";
    // templater.add_template_via_name_and_text(&name, &text);
    // ```
    //
    fn add_template_via_name_and_text(&mut self, name: &str, text: &str) -> Result<()>;

    // Add a template via name and file.
    //
    // Example:
    //
    // ```
    // let mut templater: Templater = TemplaterWithTera::new();
    // let name = "alpha";
    // let file = PathBuf::from("template.html")
    // add_template_via_name_and_file(&name, &file);
    // ```
    //
    fn add_template_via_name_and_file(&mut self, name: &str, file: &PathBuf) -> Result<()>;

    // Add tempate files via args, such as template file name.
    //
    // Example:
    //
    // ```rust
    // let paths: List<PathBuf> = vec![
    //     PathBuf::from("alpha.html"),
    //     PathBuf::from("bravo.html"),
    // ];
    // let mut args = Args::default();
    // args.template_list_path_buf = Some(paths);
    // let templater: Templater = TemplaterWithTera::new();
    // add_template_files_via_args(templater, args);
    // ```
    //
    fn add_template_files_via_args(&mut self, args: &Args) -> Result<()> {
        if let Some(ref path_buf_list) = args.template_list_path_buf {
            for path_buf in path_buf_list {
                trace!("add_template_files_via_args path_buf: {:?}", &path_buf);
                let name = path_buf.file_name().unwrap().to_string_lossy(); //TODO err
                self.add_template_via_name_and_file(&name, path_buf)
                .chain_err(|| "add_template_via_name_and_file")?;
            }
        }
        Ok(())
    }

    // Add a default template.
    //
    // Example:
    //
    // ```
    // let templater: Templater = TemplaterWithTera::new();
    // add_template_default(templater);
    // //-> Tera now has a template name "default" with content "{{ content }}"
    // ```
    //
    fn add_template_default(&mut self) -> Result<()> where Self: Sized {
        let name = &self.template_default_name();
        let content = &self.template_default_content();
        self.add_template_via_name_and_text(&name, &content)
    }

    // Does the templater have any templates?
    //
    // Example:
    //
    // ```
    // let mut templater: Templater = TemplaterWithTera::new();
    // let flag = tera_has_template(templater);
    // assert_eq!(flag, false);
    // ```
    //
    // ```
    // let mut templater: Templater = TemplaterWithTera::new();
    // templater.add_template_via_name_and_text("my-template", "{{ my-content }}");
    // let flag = has_template(templater);
    // assert_eq!(flag, true);
    // ```
    //
    fn has_template(&self) -> bool;

    // Get the template names.
    //
    // Example:
    //
    // ```
    // let mut templater: Templater = TemplaterWithTera::new();
    // add_template_via_name_and_text("alpha", "alpha text".into());
    // add_template_via_name_and_text("bravo", "bravo text".into());
    // let template_names: Set<&str> = template_names_as_set_str(&templater);
    // assert_eq!(template_names, set!["alpha", "bravo"]);
    // ```
    //
    fn template_names_as_set_str(&self) -> Set<&str>;

    // Get the template default name, which is "default".
    //
    // ```
    // let name = template_default_name();
    // assert_eq!(name, "default");
    // ```
    //
    fn template_default_name(&self) -> String {
        String::from("default")
    }

    // Get the template default content, which is "{{ content }}".
    //
    // ```
    // let content = template_default_content();
    // assert_eq!(content, "{{ content }}");
    // ```
    //
    fn template_default_content(&self) -> String {
        String::from(indoc!{r#"{{ content }}"#})
    }

    // Render a template name with the given variables such as front matter.
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
    fn render_template_with_vars<S: AsRef<str> + Sized>(&self, template_name: S, vars: &crate::matter::state::State) -> Result<HtmlString>;

}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::*;
    use crate::app::args::Args;
    use crate::templating::templater_with_tera::TemplaterWithTera;

    lazy_static! {
        pub static ref TESTS_DIR: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests"].iter().collect::<PathBuf>();
    }

    const FAB_OUTPUT_HTML: &str = "my content";

    type TemplaterX = TemplaterWithTera;

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

}
