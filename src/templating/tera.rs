use ::indoc::indoc;
use ::tera::Tera;
use crate::app::args::Args;
use crate::errors::*;

// Initiatlize Tera
//
// Example:
//
// ```
// let args = Args::default();
// let tera = init(&args);
// ```
//
pub fn init(args: &Args) -> Result<Tera> {
    let mut tera = ::tera::Tera::default();
    add_template_files_via_args(&mut tera, args)
        .chain_err(|| "add_template_files_via_args")?;
    if !has_template(&tera) {
        add_template_default(&mut tera)
        .chain_err(|| "add_template_default")?;
    }
    tera.autoescape_on(vec![]); // disable autoescaping completely
    //tera.autoescape_on(vec!["html", ".sql"]);
    //tera.register_filter("do_nothing", do_nothing_filter);
    Ok(tera)
}

// Tera: add_tempate_files() via args.
//
// Example:
//
// ```rust
// let files: Vec<PathBuf> = vec![
//     PathBuf::from("alpha.html"),
//     PathBuf::from("bravo.html"),
// ];
// let mut args = Args::default();
// args.template_files = Some(files);
// let mut tera = Tera::default();
// add_template_files_via_vec_path_buf(tera, args);
// ```
//
fn add_template_files_via_args(tera: &mut Tera, args: &Args) -> Result<()> {
    if let Some(template_glob_set) = args.template_glob_set.as_ref() {
        for glob in template_glob_set {
            for entry in ::glob::glob(glob).expect("Failed to read glob") {
                match entry {
                    Ok(path) => tera.add_template_file(path, None),
                    Err(e) => bail!("Failed to match entry. {:?}", e),
                };
            };
        };
        Ok(())
    } else {
        Ok(())
    }
}

// Tera: use add_raw_template() to add a default template
//
// Example:
//
// ```
// let mut tera = Tera::default();
// add_template_default(tera);
// //-> Tera now has a template name "default" with content "{{ content }}"
// ```
//
fn add_template_default(tera: &mut Tera) -> ::tera::Result<()> {
    tera.add_raw_template(
        &template_default_name(),
        &template_default_content(),
    )
}

// Tera: does the instance have any templates?
//
// Example:
//
// ```
// let mut tera = Tera::default();
// let flag = tera_has_template(tera);
// assert_eq!(flag, false);
// ```
//
// ```
// let mut tera = Tera::default();
// tera.add_raw_template("my-template", "{{ my-content }}");
// let flag = has_template(tera);
// assert_eq!(flag, true);
// ```
//
pub fn has_template(tera: &Tera) -> bool {
    tera.get_template_names().nth(0).is_some()
}

// Get the best template name.
//
// The best template name is currently 
// chosen as the first name alphabetically.
//
// Example with default template:
//
// ```
// let mut tera = Tera::default();
// let name = best_template_name(tera);
// assert_eq!(name, "default");
// ```
//
// Example with custom template:
//
// ```
// let mut tera = Tera::default();
// tera.add_raw_template("my-template", "{{ my-content }}");
// let name = best_template_name(tera);
// assert_eq!(name, "my-template");
// ```
//
pub fn best_template_name(tera: &Tera) -> String {
    if let Some(name) = tera.get_template_names().min() {
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
    use ::indoc::indoc;
    use ::tera::Tera;
    use crate::app::args::Args;

    fn fab_tera() -> Tera {
        let args = Args::default();
        super::init(&args).unwrap()
    }

    const FAB_OUTPUT_HTML: &str = "my content";

    #[test]
    fn test_init() {
        let args = Args::default();
        let tera = super::init(&args);
        assert!(tera.is_ok());
    }

    #[test]
    fn test_has_template_x_true() {
        let mut tera  = Tera::default();
        tera.add_raw_template("my-name", "my-content").unwrap();
        let flag = super::has_template(&tera);
        assert_eq!(flag, true);
    }

    #[test]
    fn test_has_template_x_false() {
        let tera = Tera::default();
        let flag = super::has_template(&tera);
        assert_eq!(flag, false);
    }

    #[test]
    fn test_best_template_name_x_default_name() {
        let tera = Tera::default();
        let name = best_template_name(&tera);
        assert_eq!(name, "default");
    }

    #[test]
    fn test_best_template_name_x_custom_name() {
        let mut tera = Tera::default();
        tera.add_raw_template("my-name", "{{ my-content }}").unwrap();
        let name = best_template_name(&tera);
        assert_eq!(name, "my-name");
    }

    #[test]
    fn test_tera_x_json() {
        let tera = fab_tera();
        let vars = indoc!{r#"
            {
                "title": "my title",
                "content": "my content"
            }
        "#};
        let vars: ::serde_json::Value = ::serde_json::from_str(vars).unwrap();
        let actual = tera.render(
            &template_default_name(),
            &::tera::Context::from_serialize(&vars).unwrap()
        ).unwrap();
        assert_eq!(actual, FAB_OUTPUT_HTML);
    }

    #[test]
    fn test_tera_x_toml() {
        let tera = fab_tera();
        let vars = indoc!{r#"
            title = "my title"
            content = "my content"
        "#};
        let vars: ::toml::Value = vars.parse::<::toml::Value>().unwrap();
        let actual = tera.render(
            &template_default_name(),
            &::tera::Context::from_serialize(&vars).unwrap()
        ).unwrap();
        assert_eq!(actual, FAB_OUTPUT_HTML);
    }

    #[test]
    fn test_tera_x_yaml() {
        let tera = fab_tera();
        let vars = indoc!{r#"
            title: "my title"
            content: "my content"
        "#};
        let vars: ::serde_yaml::Value = ::serde_yaml::from_str(&vars).unwrap();
        let actual = tera.render(
            &template_default_name(),
            &::tera::Context::from_serialize(&vars).unwrap()
        ).unwrap();
        assert_eq!(actual, FAB_OUTPUT_HTML);
    }

}
