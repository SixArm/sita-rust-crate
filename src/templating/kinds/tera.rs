//! Templating with Tera

use indoc::indoc;
use std::path::PathBuf;
use crate::app::args::Args;
use crate::errors::*;

pub type TEMPLATER = ::tera::Tera;

// Create a new templater.
//
// Example:
//
// ```
// let templater = new();
// ```
//
pub fn new() -> TEMPLATER {
    let mut templater = ::tera::Tera::default();
    templater.autoescape_on(vec![]); // disable autoescaping completely
    //templater.autoescape_on(vec!["html", ".sql"]);
    //templater.register_filter("do_nothing", do_nothing_filter);
    templater
}

// Create a new templater with args.
//
// Example:
//
// ```
// let args = Args::default();
// let templater = new_with_args(&args);
// ```
//
pub fn new_with_args(_args: &Args) -> TEMPLATER {
    new()
}

// Add a template via name and text.
//
// Example:
//
// ```
// let mut templater = new();
// let name = "alpha";
// let text = "<p>{{ bravo }}</p>";
// add_template_via_name_and_text(&name, &text);
// ```
//
pub fn add_template_via_name_and_text(templater: &mut TEMPLATER, name: &str, text: &str) -> Result<()> {
    templater.add_raw_template(&name, &text)
    .chain_err(|| "add_template_via_name_and_text")
}

// Add a template via name and file.
//
// Example:
//
// ```
// let mut templater = new();
// let name = "alpha";
// let file = PathBuf::from("template.html")
// add_template_via_name_and_file(&name, &file);
// ```
//
pub fn add_template_via_name_and_file(templater: &mut TEMPLATER, name: &str, file: &PathBuf) -> Result<()> {
    templater.add_template_file(&file, Some(&name))
    .chain_err(|| "add_template_via_name_and_file")
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
// args.template_path_buf_list = Some(paths);
// let mut templater = new();
// add_template_files_via_args(templater, args);
// ```
//
pub fn add_template_files_via_args(templater: &mut TEMPLATER, args: &Args) -> Result<()> {
    if let Some(ref path_buf_list) = args.template_path_buf_list {
        for path_buf in path_buf_list {
            trace!("add_template_files_via_args path_buf: {:?}", &path_buf);
            templater.add_template_file(path_buf, None)
            .chain_err(|| "add_template_file")?;
        }
    }
    Ok(())
}

// Add a default template.
//
// Example:
//
// ```
// let mut templater = new();
// add_template_default(templater);
// //-> Tera now has a template name "default" with content "{{ content }}"
// ```
//
pub fn add_template_default(templater: &mut TEMPLATER) -> ::tera::Result<()> {
    templater.add_raw_template(
        &template_default_name(),
        &template_default_content(),
    )
}

// Does the templater have any templates?
//
// Example:
//
// ```
// let mut templater = new();
// let flag = tera_has_template(templater);
// assert_eq!(flag, false);
// ```
//
// ```
// let mut templater = new();
// templater.add_raw_template("my-template", "{{ my-content }}");
// let flag = has_template(templater);
// assert_eq!(flag, true);
// ```
//
pub fn has_template(templater: &TEMPLATER) -> bool {
    templater.get_template_names().nth(0).is_some()
}

// Get the best template name.
//
// The best template name is currently 
// chosen as the first name alphabetically.
//
// Example with default template:
//
// ```
// let mut templater = new();
// let name = best_template_name(templater);
// assert_eq!(name, "default");
// ```
//
// Example with custom template:
//
// ```
// let mut templater = new();
// templater.add_raw_template("my-template", "{{ my-content }}");
// let name = best_template_name(templater);
// assert_eq!(name, "my-template");
// ```
//
pub fn best_template_name(templater: &TEMPLATER) -> String {
    if let Some(name) = templater.get_template_names().min() {
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
    fn test_new() {
        let _templater = super::new();
        //TODO
    }

    #[test]
    fn test_new_with_args() {
        let args = Args::default();
        let _templater = super::new_with_args(&args);
        //TODO
    }

    #[test]
    fn test_add_template_via_name_and_text() {
        let mut templater = super::new();
        let name = "alpha";
        let text = "{{ bravo }}";
        add_template_via_name_and_text(&mut templater, &name, &text);
        assert!(super::has_template(&templater));
    }

    #[test]
    fn test_add_template_via_name_and_file() {
        let mut templater = super::new();
        let name = "alpha";
        let file = TESTS_DIR.join("function").join("add_template_via_name_and_file").join("template.html");
        add_template_via_name_and_file(&mut templater, &name, &file);
        assert!(super::has_template(&templater));
    }

    #[test]
    fn test_has_template_x_true() {
        let mut templater  = super::new();
        templater.add_raw_template("my-name", "my-content").unwrap();
        let flag = super::has_template(&templater);
        assert_eq!(flag, true);
    }

    #[test]
    fn test_has_template_x_false() {
        let templater = super::new();
        let flag = super::has_template(&templater);
        assert_eq!(flag, false);
    }

    #[test]
    fn test_best_template_name_x_default_name() {
        let templater = super::new();
        let name = best_template_name(&templater);
        assert_eq!(name, "default");
    }

    #[test]
    fn test_best_template_name_x_custom_name() {
        let mut templater = super::new();
        templater.add_raw_template("my-name", "{{ my-content }}").unwrap();
        let name = best_template_name(&templater);
        assert_eq!(name, "my-name");
    }

    // #[test]
    // fn test_render_x_html() {
    //     let mut templater = super::new();
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
    //         &::tera::Context::from_serialize(&vars).unwrap()
    //     ).unwrap();
    //     assert_eq!(actual, FAB_OUTPUT_HTML);
    // }

    #[test]
    fn test_render_x_json() {
        let mut templater = super::new();
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
            &::tera::Context::from_serialize(&vars).unwrap()
        ).unwrap();
        assert_eq!(actual, FAB_OUTPUT_HTML);
    }

    #[test]
    fn test_render_x_toml() {
        let mut templater = super::new();
        add_template_default(&mut templater).expect("default");
        let vars = indoc!{r#"
            title = "my title"
            content = "my content"
        "#};
        let vars: ::toml::Value = vars.parse::<::toml::Value>().unwrap();
        let actual = templater.render(
            &template_default_name(),
            &::tera::Context::from_serialize(&vars).unwrap()
        ).unwrap();
        assert_eq!(actual, FAB_OUTPUT_HTML);
    }

    #[test]
    fn test_render_x_yaml() {
        let mut templater = super::new();
        add_template_default(&mut templater).expect("default");
        let vars = indoc!{r#"
            title: "my title"
            content: "my content"
        "#};
        let vars: ::serde_yaml::Value = ::serde_yaml::from_str(&vars).unwrap();
        let actual = templater.render(
            &template_default_name(),
            &::tera::Context::from_serialize(&vars).unwrap()
        ).unwrap();
        assert_eq!(actual, FAB_OUTPUT_HTML);
    }

}
