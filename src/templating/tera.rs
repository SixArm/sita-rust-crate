use ::indoc::indoc;
use ::tera::Tera;
use ::std::path::PathBuf;
use crate::app::args::Args;
use crate::errors::*;

// Return the template default name, which is "default".
//
// ```rust
// template_default_name()
// //-> "default"
// ```
//
pub fn template_default_name() -> &'static str {
    "default"
}

// Return the template default content, which is "{{ content }}".
//
// ```rust
// template_default_content()
// //-> "{{ content }}"
// ```
//
pub fn template_default_content() -> &'static str {
    indoc!{r#"{{ content }}"#}
}

// Tera new or default.
//
// If args has a template glob, then call Tera::new,
// otherwise call Tera::default().
//
// ```
// let args = Args::default();
// let mut tera = tera_new_via_args(args);
// //-> A new Tera object via Tera::default()
// ```
//
// let mut args = Args::default();
// args.template_glob = PathBuf::from("./templates");
// let mut tera = tera_new_via_args(args);
// //-> A new Tera object via Tera::new(…) with the glob
//
fn tera_new_via_args(args: &Args) -> Result<Tera> {
    match &args.template_glob {
        Some(template_glob) => {
            Ok(
                Tera::new(&*template_glob.as_os_str().to_string_lossy())
                .chain_err(|| format!("create tera. template glob: {:?}", template_glob))?
            )
        },
        _ => {
            Ok(Tera::default())
        }
    }
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
// let mut tera: Tera::default();
// tera_add_template_files_via_vec_path_buf(tera, args);
// ```
//
fn tera_add_template_files_via_args(tera: &mut Tera, args: &Args) -> ::tera::Result<()> {
    if let Some(files) = args.template_files.as_ref() {
        tera.add_template_files(
            files.into_iter().map(|x| 
                (x.clone(), None)
            ).collect::<Vec<(PathBuf, Option<String>)>>()
        )
    } else {
        Ok(())
    }
}

// Tera: use add_raw_template() to add a default template
//
// Example:
//
// ```
// let mut tera: Tera::default();
// tera_add_template_default(tera);
// //-> Tera now has a template name "default" with content "{{ content }}"
// ```
//
fn tera_add_template_default(tera: &mut Tera) -> ::tera::Result<()> {
    tera.add_raw_template(
        template_default_name(),
        template_default_content(),
    )
}

pub fn init(args: &Args) -> Result<Tera> {
    let mut tera = tera_new_via_args(args)
        .chain_err(|| "tera_new_via_args")?;
    tera_add_template_default(&mut tera)
        .chain_err(|| "tera_add_template_default")?;
    tera_add_template_files_via_args(&mut tera, args)
        .chain_err(|| "tera_add_template_files_via_args")?;
    tera.autoescape_on(vec![]); // disable autoescaping completely
    //tera.autoescape_on(vec!["html", ".sql"]);
    //tera.register_filter("do_nothing", do_nothing_filter);
    Ok(tera)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::indoc::indoc;
    use crate::app::args::Args;
    use crate::templating::vars::Vars;

    fn fab_tera() -> Tera {
        let args = Args::default();
        super::init(&args).unwrap()
    }

    const FAB_OUTPUT_HTML: &str = "my content";

    #[test]
    fn test_init() {
        //TODO
    }

    #[test]
    fn test_tera_x_vars() {
        let tera = fab_tera();
        let vars = Vars {
            title: Some("my title".into()),
            content: Some("my content".into()),
        };
        let actual = tera.render(
            template_default_name(),
            &::tera::Context::from_serialize(&vars).unwrap()
        ).unwrap();
        assert_eq!(actual, FAB_OUTPUT_HTML);
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
            template_default_name(),
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
            template_default_name(),
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
            template_default_name(),
            &::tera::Context::from_serialize(&vars).unwrap()
        ).unwrap();
        assert_eq!(actual, FAB_OUTPUT_HTML);
    }

}
