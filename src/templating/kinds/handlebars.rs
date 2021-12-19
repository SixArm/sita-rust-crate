//! Templating with Handlebars

use handlebars::Handlebars;
use indoc::indoc;
use std::path::PathBuf;
use crate::app::args::Args;
use crate::errors::*;
use crate::types::*;

pub type Templater<'a> = handlebars::Handlebars<'a>;

// Create a default templater.
//
// Example:
//
// ```
// let templater = default();
// ```
//
pub fn default<'a>() -> Templater<'a> {
    Handlebars::default()
}

// Create a default templater with args.
//
// Example:
//
// ```
// let args = Args::default();
// let templater = default_with_args(&args);
// ```
//
pub fn default_with_args(_args: &Args) -> Templater {
    let mut templater = default();
    templater.set_strict_mode(true);
    templater
}

// Add a template via name and text.
//
// Example:
//
// ```
// let mut templater = default();
// let name = "alpha";
// let text = "<p>{{ bravo }}</p>";
// add_template_via_name_and_text(&name, &text);
// ```
//
pub fn add_template_via_name_and_text(templater: &mut Templater, name: &str, text: &str) -> Result<()> {
  templater.register_template_string(&name, &text)
  .chain_err(|| "add_template_via_name_and_text")
}

// Add a template via name and file.
//
// Example:
//
// ```
// let mut templater = default();
// let name = "alpha";
// let file = PathBuf::from("template.html")
// add_template_via_name_and_file(&name, &file);
// ```
//
pub fn add_template_via_name_and_file(templater: &mut Templater, name: &str, file: &PathBuf) -> Result<()> {
    let s = ::std::fs::read_to_string(file)
    .chain_err(|| "add_template_via_name_and_text read_to_string")?;
    templater.register_template_string(&name, &s)
    .chain_err(|| "add_template_via_name_and_text register_template_string")
}

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
// let mut templater = default();
// add_template_files_via_args(templater, args);
// ```
//
pub fn add_template_files_via_args(templater: &mut Templater, args: &Args) -> Result<()> {
    if let Some(ref path_buf_list) = args.template_list_path_buf {
        for path_buf in path_buf_list {
            trace!("add_template_files_via_args path_buf: {:?}", &path_buf);
            let name = path_buf.file_name().unwrap().to_string_lossy(); //TODO err
            add_template_via_name_and_file(templater, &name, path_buf)
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
// let mut templater = default();
// add_template_default(templater);
// //-> Templater now has a template name "default" with content "{{ content }}"
// ```
//
pub fn add_template_default(templater: &mut Templater) -> Result<()> {
    add_template_via_name_and_text(
        templater,
        &template_default_name(),
        &template_default_content(),
    )
}

// Does the templater have any templates?
//
// Example:
//
// ```
// let mut templater = default();
// let flag = has_template(templater);
// assert_eq!(flag, false);
// ```
//
// ```
// let mut templater = default();
// templater.add_raw_template("my-template", "{{ my-content }}");
// let flag = has_template(templater);
// assert_eq!(flag, true);
// ```
//
pub fn has_template(templater: &Templater) -> bool {
    !templater.get_templates().is_empty()
}

// Get the template names.
//
// Example:
//
// ```
// let mut templater = default();
// add_template_via_name_and_text("alpha".into(), "alpha text".into());
// add_template_via_name_and_text("bravo".into(), "bravo text".into());
// let template_names: Set<&String> = template_names_as_set_string(&templater);
// assert_eq!(template_names, set![&"alpha".into(), &"bravo".into()]);
// ```
//
pub fn template_names_as_set_string<'a>(templater: &'a Templater) -> Set<&'a String> {
    templater.get_templates().keys().collect::<Set<&'a String>>()
}

// Get the best template name.
//
// The best template name is currently 
// chosen as the first name alphabetically.
//
// Example with default template:
//
// ```
// let mut templater = default();
// let name = best_template_name(templater);
// assert_eq!(name, "default");
// ```
//
// Example with custom template:
//
// ```
// let mut templater = default();
// templater.add_raw_template("my-template", "{{ my-content }}");
// let name = best_template_name(templater);
// assert_eq!(name, "my-template");
// ```
//
pub fn best_template_name(templater: &Templater) -> String {
    if let Some(name) = templater.get_templates().keys().min() {
        String::from(name)
    } else {
        template_default_name()
    }
}

// Get the template default name, which is "default".
//
// ```
// let name = template_default_name();
// assert_eq!(name, "default");
// ```
//
pub fn template_default_name() -> String {
    String::from("default")
}

// Get the template default content, which is "{{ content }}".
//
// ```
// let content = template_default_content();
// assert_eq!(content, "{{ content }}");
// ```
//
pub fn template_default_content() -> String {
    String::from(indoc!{r#"{{ content }}"#})
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

    #[test]
    fn test_default() {
        let _templater = super::default();
        //TODO
    }

    #[test]
    fn test_new_with_args() {
        let args = Args::default();
        let _templater = super::default_with_args(&args);
        //TODO
    }

    #[test]
    fn test_add_template_via_name_and_text() {
        let mut templater = super::default();
        let name = "alpha";
        let text = "{{ bravo }}";
        add_template_via_name_and_text(&mut templater, &name, &text);
        assert!(super::has_template(&templater));
    }

    #[test]
    fn test_add_template_via_name_and_file() {
        let mut templater = super::default();
        let name = "alpha";
        let file = TESTS_DIR.join("function").join("add_template_via_name_and_file").join("template.html");
        add_template_via_name_and_file(&mut templater, &name, &file);
        assert!(super::has_template(&templater));
    }
        
    #[test]
    fn test_has_template_x_true() {
        let mut templater  = super::default();
        templater.register_template_string("my-name", "my-content").unwrap();
        let flag = super::has_template(&templater);
        assert_eq!(flag, true);
    }

    #[test]
    fn test_has_template_x_false() {
        let templater = super::default();
        let flag = super::has_template(&templater);
        assert_eq!(flag, false);
    }

    #[test]
    pub fn test_template_names_as_set_string() {
        let mut templater = default();
        let name_0: String = "my-name-0".into();
        let name_1: String = "my-name-1".into();
        add_template_via_name_and_text(&mut templater, &name_0, "my text 0").expect("add_template_via_name_and_text");
        add_template_via_name_and_text(&mut templater, &name_1, "my text 1").expect("add_template_via_name_and_text");
        let actual: Set<&String> = template_names_as_set_string(&templater);
        let expect: Set<&String> = set!(&name_0, &name_1);
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_best_template_name_x_default_name() {
        let templater = super::default();
        let name = best_template_name(&templater);
        assert_eq!(name, "default");
    }

    #[test]
    fn test_best_template_name_x_custom_name() {
        let mut templater = super::default();
        templater.register_template_string("my-name", "{{ my-content }}").unwrap();
        let name = best_template_name(&templater);
        assert_eq!(name, "my-name");
    }

    // #[test]
    // fn test_render_x_html() {
    //     let mut templater = super::default();
    //     add_template_default(&mut templater).expect("default");
    //     let vars = indoc!{r#"
    //         <!--
    //             title: my title
    //             content: my content
    //         -->
    //     "#};
    //     let vars: ::serde_json::Value = ::serde_json::from_str(vars).unwrap();
    //     let actual = templater.render(
    //         &template_default_name(),
    //         &::tera::= super::default()t::from_serialize(&vars).unwrap()
    //     ).unwrap();
    //     assert_eq!(actual, FAB_OUTPUT_HTML);
    // }

    #[test]
    fn test_render_x_json() {
        let mut templater = super::default();
        add_template_default(&mut templater).expect("default");
        let vars = indoc!{r#"
            {
                "title": "my title",
                "content": "my content"
            }
        "#};
        let vars: ::serde_json::Value = ::serde_json::from_str(vars).unwrap();
        let actual = templater.render(
            &template_default_name(),
            &vars
        ).unwrap();
        assert_eq!(actual, FAB_OUTPUT_HTML);
    }

    #[test]
    fn test_render_x_toml() {
        let mut templater = super::default();
        add_template_default(&mut templater).expect("default");
        let vars = indoc!{r#"
            title = "my title"
            content = "my content"
        "#};
        let vars: ::toml::Value = vars.parse::<::toml::Value>().unwrap();
        let actual = templater.render(
            &template_default_name(),
            &vars
        ).unwrap();
        assert_eq!(actual, FAB_OUTPUT_HTML);
    }

    #[test]
    fn test_render_x_yaml() {
        let mut templater = super::default();
        add_template_default(&mut templater).expect("default");
        let vars = indoc!{r#"
            title: "my title"
            content: "my content"
        "#};
        let vars: ::serde_yaml::Value = ::serde_yaml::from_str(&vars).unwrap();
        let actual = templater.render(
            &template_default_name(),
            &vars
        ).unwrap();
        assert_eq!(actual, FAB_OUTPUT_HTML);
    }

}
